use anyhow::Result;
use chrono::Utc;

use crate::fs_ops::append_line;
use crate::schema::AuditEntry;

pub fn log_violation(task_id: Option<&str>, rule: &str, severity: &str, message: &str) -> Result<()> {
    let entry = AuditEntry {
        timestamp: Utc::now(),
        task_id: task_id.map(|s| s.to_string()),
        rule: rule.to_string(),
        severity: severity.to_string(),
        message: message.to_string(),
    };

    let log_line = format!(
        "[{}] [{}] {} {} - {}",
        entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        entry.severity.to_uppercase(),
        entry.rule,
        entry.task_id.as_deref().unwrap_or("GLOBAL"),
        entry.message
    );

    append_line(&crate::common::audit_log_path(), &log_line)
}

pub fn log_info(task_id: Option<&str>, rule: &str, message: &str) -> Result<()> {
    log_violation(task_id, rule, "info", message)
}

#[allow(dead_code)]
pub fn log_warning(task_id: Option<&str>, rule: &str, message: &str) -> Result<()> {
    log_violation(task_id, rule, "warning", message)
}

#[allow(dead_code)]
pub fn log_error(task_id: Option<&str>, rule: &str, message: &str) -> Result<()> {
    log_violation(task_id, rule, "error", message)
}

pub fn log_entry(task_id: &str, rule: &str, severity: &str, message: &str) -> Result<()> {
    log_violation(Some(task_id), rule, severity, message)
}

#[allow(dead_code)]
pub fn read_audit_log(limit: usize) -> Result<Vec<String>> {
    let audit_path = crate::common::audit_log_path();
    
    if !audit_path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&audit_path)?;
    let lines: Vec<String> = content
        .lines()
        .rev()
        .take(limit)
        .map(|s| s.to_string())
        .collect();
    
    Ok(lines)
}