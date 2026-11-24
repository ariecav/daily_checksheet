use crate::backend::db::Database;
use crate::backend::models::{EvaluationTest, Participant, TestResult, TestSubmission, TestSummary};
use chrono::{Datelike, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EvaluationService {
    db: Arc<Database>,
    // In-memory store for loaded test JSON structures keyed by test_id
    tests: Arc<RwLock<HashMap<String, EvaluationTest>>>,
}

impl EvaluationService {
    pub fn new(db: Database) -> Self {
        let service = Self {
            db: Arc::new(db),
            tests: Arc::new(RwLock::new(HashMap::new())),
        };
        service
    }

    pub async fn preload_tests(&self) {
        if let Ok(persisted) = self.db.get_all_tests().await {
            let mut tests_map = self.tests.write().await;
            for test in persisted {
                tests_map.insert(test.test_id.clone(), test);
            }
        }

        // Seed from IK001.json only if it hasn't already been persisted to the DB
        // (otherwise a restart would wipe out edits made via update_question).
        let path = "IK001.json";
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(test) = serde_json::from_str::<EvaluationTest>(&content) {
                let already_exists = {
                    let tests_map = self.tests.read().await;
                    tests_map.contains_key(&test.test_id)
                };
                if !already_exists {
                    if let Ok(serialized) = serde_json::to_string(&test) {
                        let _ = self.db.save_test(&test.test_id, &serialized).await;
                    }
                    let mut tests_map = self.tests.write().await;
                    tests_map.insert(test.test_id.clone(), test);
                    println!("✅ Preloaded test: {}", path);
                }
            }
        }
    }

    pub async fn upload_test(&self, mut test: EvaluationTest) -> Result<String, String> {
        if test.test_id.is_empty() {
            return Err("Test ID cannot be empty".to_string());
        }
        let requested_id = test.test_id.clone();
        let assigned_id = {
            let tests_map = self.tests.read().await;
            if !tests_map.contains_key(&requested_id) {
                requested_id.clone()
            } else {
                let mut suffix = 2;
                loop {
                    let candidate = format!("{requested_id}-{suffix}");
                    if !tests_map.contains_key(&candidate) {
                        break candidate;
                    }
                    suffix += 1;
                }
            }
        };
        test.test_id = assigned_id.clone();
        let serialized = serde_json::to_string(&test).map_err(|e| e.to_string())?;
        self.db
            .save_test(&test.test_id, &serialized)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        let mut tests_map = self.tests.write().await;
        tests_map.insert(test.test_id.clone(), test);
        Ok(assigned_id)
    }

    pub async fn get_test(&self, test_id: &str) -> Option<EvaluationTest> {
        let tests_map = self.tests.read().await;
        tests_map.get(test_id).cloned()
    }

    pub async fn update_question(
        &self,
        test_id: &str,
        question_id: &str,
        question_text: String,
        image: Option<String>,
    ) -> Result<(), String> {
        let updated_test = {
            let mut tests_map = self.tests.write().await;
            let test = tests_map
                .get_mut(test_id)
                .ok_or_else(|| format!("Test with ID '{}' not found", test_id))?;

            let mut found = false;
            for section in test.sections.iter_mut() {
                for q in section.questions.iter_mut() {
                    if q.id == question_id {
                        q.question_text = question_text.clone();
                        q.image = image.clone();
                        found = true;
                    }
                }
            }
            if !found {
                return Err(format!("Question with ID '{}' not found", question_id));
            }
            test.clone()
        };

        let serialized = serde_json::to_string(&updated_test).map_err(|e| e.to_string())?;
        self.db
            .save_test(test_id, &serialized)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_departments(&self) -> Result<Vec<String>, String> {
        self.db
            .get_departments()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_participant_sections(&self) -> Result<Vec<String>, String> {
        self.db
            .get_participant_sections()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_participants_by_section(
        &self,
        section: &str,
    ) -> Result<Vec<Participant>, String> {
        self.db
            .get_participants_by_section(section)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn delete_tests(&self, test_ids: Vec<String>) -> Result<(), String> {
        self.db
            .delete_tests(&test_ids)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        let mut tests_map = self.tests.write().await;
        for id in test_ids {
            tests_map.remove(&id);
        }
        Ok(())
    }

    pub async fn delete_all_tests(&self) -> Result<(), String> {
        self.db
            .delete_all_tests()
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        let mut tests_map = self.tests.write().await;
        tests_map.clear();
        Ok(())
    }

    pub async fn list_tests(&self) -> Vec<TestSummary> {
        let tests_map = self.tests.read().await;
        tests_map
            .values()
            .map(|t| TestSummary {
                test_id: t.test_id.clone(),
                test_title: t.test_title.clone(),
                level: t.level,
                department: t.department.clone(),
                test_name: t.test_name.clone(),
            })
            .collect()
    }

    pub async fn evaluate_and_save(
        &self,
        submission: TestSubmission,
    ) -> Result<TestResult, String> {
        let test = match self.get_test(&submission.test_id).await {
            Some(t) => t,
            None => return Err(format!("Test with ID '{}' not found", submission.test_id)),
        };

        let mut total_score = 0;
        let mut max_score = 0;

        // Build a lookup map for correct answers: question_id -> (correct_answer, points)
        let mut answer_key = HashMap::new();
        for section in &test.sections {
            for q in &section.questions {
                answer_key.insert(q.id.clone(), (q.correct_answer.clone(), q.points));
                max_score += q.points;
            }
        }

        // Evaluate submitted answers (max nesting level = 2, well within max 3)
        for sub in &submission.answers {
            if let Some((correct_ans, pts)) = answer_key.get(&sub.question_id) {
                if sub
                    .selected_answer
                    .trim()
                    .eq_ignore_ascii_case(correct_ans.trim())
                {
                    total_score += pts;
                }
            }
        }

        let percentage = if max_score > 0 {
            (total_score as f64 / max_score as f64) * 100.0
        } else {
            0.0
        };

        let now = Utc::now();
        let evaluated_at = now.to_rfc3339();
        let test_date = now.format("%Y-%m-%d").to_string();
        let fiscal_year = format!("FY{}", if now.month() >= 4 {
            now.year() % 100
        } else {
            (now.year() - 1) % 100
        });

        let section = self
            .db
            .get_participant_section(&submission.user_name)
            .await
            .unwrap_or(None);
        let test_name = test.test_name.clone();

        let submission_id = self
            .db
            .save_result(
                &test.test_id,
                &submission.user_name,
                total_score,
                max_score,
                percentage,
                &evaluated_at,
                &test_date,
                &fiscal_year,
                submission.mode.as_deref(),
                submission.level,
                section.as_deref(),
                test_name.as_deref(),
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(TestResult {
            submission_id,
            test_id: test.test_id,
            user_name: submission.user_name,
            total_score,
            max_score,
            percentage,
            evaluated_at,
            test_date,
            fiscal_year,
            mode: submission.mode,
            level: submission.level,
            section,
            test_name,
        })
    }

    pub async fn delete_results(&self, ids: Vec<i64>) -> Result<(), String> {
        self.db
            .delete_results(&ids)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn delete_all_results(&self) -> Result<(), String> {
        self.db
            .delete_all_results()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_results(&self) -> Result<Vec<TestResult>, String> {
        self.db
            .get_all_results()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }
}
