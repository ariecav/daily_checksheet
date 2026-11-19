use crate::backend::models::{EvaluationTest, TestResult};
use sqlx::{Row, SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS test_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                test_id TEXT NOT NULL,
                user_name TEXT NOT NULL,
                total_score INTEGER NOT NULL,
                max_score INTEGER NOT NULL,
                percentage REAL NOT NULL,
                evaluated_at TEXT NOT NULL,
                test_date TEXT,
                fiscal_year TEXT,
                mode TEXT,
                level INTEGER
            )
            "#,
        )
        .execute(&pool)
        .await?;

        let columns = sqlx::query("PRAGMA table_info(test_results)")
            .fetch_all(&pool)
            .await?
            .into_iter()
            .filter_map(|row| row.try_get::<String, _>("name").ok())
            .collect::<std::collections::HashSet<_>>();

        if !columns.contains("mode") {
            sqlx::query("ALTER TABLE test_results ADD COLUMN mode TEXT")
                .execute(&pool)
                .await?;
        }

        if !columns.contains("level") {
            sqlx::query("ALTER TABLE test_results ADD COLUMN level INTEGER")
                .execute(&pool)
                .await?;
        }

        if !columns.contains("test_date") {
            sqlx::query("ALTER TABLE test_results ADD COLUMN test_date TEXT")
                .execute(&pool)
                .await?;
        }

        if !columns.contains("fiscal_year") {
            sqlx::query("ALTER TABLE test_results ADD COLUMN fiscal_year TEXT")
                .execute(&pool)
                .await?;
        }

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tests (
                test_id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS departments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )
            "#,
        )
        .execute(&pool)
        .await?;

        for department in ["QA", "IMP", "SAFETY"] {
            sqlx::query("INSERT OR IGNORE INTO departments (name) VALUES (?)")
                .bind(department)
                .execute(&pool)
                .await?;
        }

        Ok(Self { pool })
    }

    pub async fn get_departments(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query_scalar::<_, String>("SELECT name FROM departments ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn save_result(
        &self,
        test_id: &str,
        user_name: &str,
        total_score: i32,
        max_score: i32,
        percentage: f64,
        evaluated_at: &str,
        test_date: &str,
        fiscal_year: &str,
        mode: Option<&str>,
        level: Option<i32>,
    ) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query(
            r#"
            INSERT INTO test_results (test_id, user_name, total_score, max_score, percentage, evaluated_at, test_date, fiscal_year, mode, level)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
        )
        .bind(test_id)
        .bind(user_name)
        .bind(total_score)
        .bind(max_score)
        .bind(percentage)
        .bind(evaluated_at)
        .bind(test_date)
        .bind(fiscal_year)
        .bind(mode)
        .bind(level)
        .fetch_one(&self.pool)
        .await?;

        use sqlx::Row;
        Ok(rec.get("id"))
    }

    pub async fn delete_results(&self, ids: &[i64]) -> Result<(), sqlx::Error> {
        if ids.is_empty() {
            return Ok(());
        }

        for id in ids {
            sqlx::query("DELETE FROM test_results WHERE id = ?")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    pub async fn delete_all_results(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM test_results")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn save_test(&self, test_id: &str, data: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO tests (test_id, data) VALUES (?, ?) ON CONFLICT(test_id) DO UPDATE SET data = excluded.data",
        )
        .bind(test_id)
        .bind(data)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_tests(&self, ids: &[String]) -> Result<(), sqlx::Error> {
        for id in ids {
            sqlx::query("DELETE FROM tests WHERE test_id = ?")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_all_tests(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tests").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_all_tests(&self) -> Result<Vec<EvaluationTest>, sqlx::Error> {
        let rows: Vec<String> = sqlx::query_scalar("SELECT data FROM tests")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .filter_map(|d| serde_json::from_str(&d).ok())
            .collect())
    }

    pub async fn get_all_results(&self) -> Result<Vec<TestResult>, sqlx::Error> {
        let rows = sqlx::query_as::<_, TestResultRow>(
            "SELECT id, test_id, user_name, total_score, max_score, percentage, evaluated_at, test_date, fiscal_year, mode, level FROM test_results ORDER BY id DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| TestResult {
                submission_id: r.id,
                test_id: r.test_id,
                user_name: r.user_name,
                total_score: r.total_score,
                max_score: r.max_score,
                percentage: r.percentage,
                evaluated_at: r.evaluated_at.unwrap_or_default(),
                test_date: r.test_date.unwrap_or_default(),
                fiscal_year: r.fiscal_year.unwrap_or_default(),
                mode: r.mode,
                level: r.level,
            })
            .collect();

        Ok(results)
    }
}

#[derive(sqlx::FromRow)]
struct TestResultRow {
    id: i64,
    test_id: String,
    user_name: String,
    total_score: i32,
    max_score: i32,
    percentage: f64,
    evaluated_at: Option<String>,
    test_date: Option<String>,
    fiscal_year: Option<String>,
    mode: Option<String>,
    level: Option<i32>,
}
