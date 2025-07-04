use std::path::{Path, PathBuf};

pub const ROTD_DIR: &str = ".rotd";
pub const TASKS_FILE: &str = "tasks.jsonl";
pub const LESSONS_FILE: &str = "lessons_learned.jsonl";
pub const PSS_SCORES_FILE: &str = "pss_scores.jsonl";
pub const SESSION_STATE_FILE: &str = "session_state.json";
pub const COVERAGE_HISTORY_FILE: &str = "coverage_history.json";
pub const AUDIT_LOG_FILE: &str = "audit.log";
pub const TEST_SUMMARIES_DIR: &str = "test_summaries";
#[allow(dead_code)]
pub const COORDINATION_DIR: &str = "coordination";
#[allow(dead_code)]
pub const ACTIVE_WORK_REGISTRY_FILE: &str = "active_work_registry.json";

pub fn rotd_path() -> PathBuf {
    Path::new(ROTD_DIR).to_path_buf()
}

pub fn tasks_path() -> PathBuf {
    rotd_path().join(TASKS_FILE)
}

pub fn lessons_path() -> PathBuf {
    rotd_path().join(LESSONS_FILE)
}

pub fn pss_scores_path() -> PathBuf {
    rotd_path().join(PSS_SCORES_FILE)
}

pub fn session_state_path() -> PathBuf {
    rotd_path().join(SESSION_STATE_FILE)
}

pub fn coverage_history_path() -> PathBuf {
    rotd_path().join(COVERAGE_HISTORY_FILE)
}

pub fn audit_log_path() -> PathBuf {
    rotd_path().join(AUDIT_LOG_FILE)
}

#[allow(dead_code)]
pub fn active_work_registry_path() -> PathBuf {
    rotd_path()
        .join(COORDINATION_DIR)
        .join(ACTIVE_WORK_REGISTRY_FILE)
}

pub fn test_summaries_path() -> PathBuf {
    rotd_path().join(TEST_SUMMARIES_DIR)
}

pub fn test_summary_file(task_id: &str) -> PathBuf {
    test_summaries_path().join(format!("{}.json", task_id))
}

pub fn check_rotd_initialized() -> anyhow::Result<()> {
    if !rotd_path().exists() {
        return Err(anyhow::anyhow!(
            "No .rotd directory found. Run 'rotd init' first."
        ));
    }
    Ok(())
}
