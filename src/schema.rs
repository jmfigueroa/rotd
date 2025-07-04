use anyhow::Result;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskEntry {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub tests: Option<Vec<String>>,
    pub description: Option<String>,
    pub summary_file: Option<String>,
    pub origin: Option<String>,
    pub phase: Option<String>,
    pub depends_on: Option<Vec<String>>,
    pub priority: Option<Priority>,
    pub priority_score: Option<f64>,
    pub created: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Complete,
    Blocked,
    Scaffolded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RegistryStatus {
    Unclaimed,
    Claimed,
    Done,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryEntry {
    pub id: String,
    pub title: String,
    pub status: RegistryStatus,
    pub priority: Priority,
    pub claimed_by: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveWorkRegistry {
    pub tasks: Vec<RegistryEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Urgent,
    High,
    Medium,
    Low,
    Deferred,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Urgent => "urgent",
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
            Priority::Deferred => "deferred",
        }
    }

    pub fn normal(&self) -> colored::ColoredString {
        match self {
            Priority::Urgent => "Urgent".red().bold(),
            Priority::High => "High".red(),
            Priority::Medium => "Medium".yellow(),
            Priority::Low => "Low".green(),
            Priority::Deferred => "Deferred".blue(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSummary {
    pub task_id: String,
    pub status: String,
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub warnings: Option<Vec<String>>,
    pub coverage: Option<f64>,
    pub verified_by: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LessonLearned {
    pub id: String,
    pub hash: Option<String>,
    pub trigger: Vec<String>,
    pub context: HashMap<String, serde_json::Value>,
    pub diagnosis: String,
    pub remediation: String,
    pub tags: Vec<String>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PSSScore {
    pub task_id: String,
    pub score: u32,
    pub timestamp: DateTime<Utc>,
    pub criteria: HashMap<String, CriterionScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CriterionScore {
    pub score: u32,
    pub rationale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverageHistory {
    pub floor: f64,
    pub ratchet_threshold: f64,
    pub history: Vec<CoverageEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverageEntry {
    pub task_id: String,
    pub coverage: f64,
    pub timestamp: DateTime<Utc>,
    pub triggered_ratchet: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub current_task: Option<String>,
    pub status: String,
    pub deltas: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub task_id: Option<String>,
    pub rule: String,
    pub severity: String,
    pub message: String,
}

// Validation functions
impl TaskEntry {
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(anyhow::anyhow!("Task ID cannot be empty"));
        }
        if self.title.is_empty() {
            return Err(anyhow::anyhow!("Task title cannot be empty"));
        }
        Ok(())
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}

impl TestSummary {
    pub fn validate(&self) -> Result<()> {
        if self.task_id.is_empty() {
            return Err(anyhow::anyhow!("Task ID cannot be empty"));
        }
        if self.passed + self.failed != self.total_tests {
            return Err(anyhow::anyhow!("Test counts don't add up"));
        }
        Ok(())
    }
}

impl LessonLearned {
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(anyhow::anyhow!("Lesson ID cannot be empty"));
        }
        if self.diagnosis.is_empty() || self.remediation.is_empty() {
            return Err(anyhow::anyhow!("Diagnosis and remediation are required"));
        }
        Ok(())
    }
}

// Update-related structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub version: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub manifest_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    pub date: String,
    pub changes: Vec<ChangeEntry>,
    pub previous_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeEntry {
    pub change_type: String,
    pub component: String,
    pub description: String,
    pub breaking: bool,
    pub migration_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHistoryEntry {
    pub version: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub status: String,
    pub changes_applied: Vec<String>,
    pub migration_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub overall_status: String,
    pub reports: HashMap<String, ValidationResult>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub status: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub items_checked: u32,
}

// Primer-related structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectPrimer {
    pub name: String,
    pub scope: String,
    pub description: String,
    pub status: String,
    pub language: String,
    pub entry_points: Vec<String>,
    pub test_dirs: Vec<String>,
    pub dependencies: Vec<String>,
    pub known_issues: Vec<String>,
    pub key_concepts: Vec<String>,
    pub preferred_agents: Option<Vec<String>>,
    pub suggested_starting_points: Vec<String>,
    pub major_components: Option<HashMap<String, ComponentInfo>>,
    pub update_triggers: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentInfo {
    pub description: String,
    pub files: Vec<String>,
}
