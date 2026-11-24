use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer: String, // e.g., "A", "B", "C", "D" or exact option text
    pub points: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>, // optional base64 data URI
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateQuestionRequest {
    pub question_text: String,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    pub section_id: String,
    pub section_name: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvaluationTest {
    pub test_id: String,
    pub test_title: String,
    pub sections: Vec<Section>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub department: Option<String>,
    #[serde(default)]
    pub test_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Participant {
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestSummary {
    pub test_id: String,
    pub test_title: String,
    pub level: Option<i32>,
    pub department: Option<String>,
    pub test_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnswerSubmission {
    pub question_id: String,
    pub selected_answer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestSubmission {
    pub test_id: String,
    pub user_name: String,
    pub answers: Vec<AnswerSubmission>,
    pub mode: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResult {
    pub submission_id: i64,
    pub test_id: String,
    pub user_name: String,
    pub total_score: i32,
    pub max_score: i32,
    pub percentage: f64,
    pub evaluated_at: String,
    pub test_date: String,
    pub fiscal_year: String,
    pub mode: Option<String>,
    pub level: Option<i32>,
    pub section: Option<String>,
    pub test_name: Option<String>,
}
