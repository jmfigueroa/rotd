use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::fs;

use crate::common;
use crate::fs_ops::{append_jsonl, read_jsonl};
use crate::schema::{TaskEntry, TaskHistoryEvent, RotdConfig};

pub fn get_agent_id() -> String {
    env::var("ROTD_AGENT_ID").unwrap_or_else(|_| "human".to_string())
}

pub fn append_task_history(
    task: &TaskEntry,
    prev_task: Option<&TaskEntry>,
    comment: Option<String>,
    pss_delta: Option<f64>,
) -> Result<()> {
    let agent_id = get_agent_id();
    
    let mut event = TaskHistoryEvent::new(
        task.id.clone(),
        agent_id,
        format!("{:?}", task.status).to_lowercase(),
    );
    
    // Set previous values if we have them
    if let Some(prev) = prev_task {
        event.prev_status = Some(format!("{:?}", prev.status).to_lowercase());
        
        if prev.priority != task.priority {
            event.prev_priority = prev.priority.as_ref().map(|p| p.as_str().to_string());
            event.priority = task.priority.as_ref().map(|p| p.as_str().to_string());
        }
    }
    
    event.comment = comment.map(|c| {
        if c.len() > 280 {
            format!("{}...", &c[..277])
        } else {
            c
        }
    });
    
    event.pss_delta = pss_delta;
    
    event.validate()?;
    
    let history_file = common::task_history_file(&task.id);
    append_jsonl(&history_file, &event)
}

pub fn read_task_history(task_id: &str) -> Result<Vec<TaskHistoryEvent>> {
    let history_file = common::task_history_file(task_id);
    read_jsonl(&history_file)
}

pub fn get_task_history_stats(task_id: &str) -> Result<TaskHistoryStats> {
    let events = read_task_history(task_id)?;
    
    let mut status_counts: HashMap<String, u32> = HashMap::new();
    let mut agent_contributions: HashMap<String, u32> = HashMap::new();
    let mut total_pss_delta = 0.0;
    
    for event in &events {
        *status_counts.entry(event.status.clone()).or_insert(0) += 1;
        *agent_contributions.entry(event.agent_id.clone()).or_insert(0) += 1;
        
        if let Some(delta) = event.pss_delta {
            total_pss_delta += delta;
        }
    }
    
    Ok(TaskHistoryStats {
        total_events: events.len(),
        status_counts,
        agent_contributions,
        total_pss_delta,
        first_event: events.first().cloned(),
        last_event: events.last().cloned(),
    })
}

pub fn get_history_size_mib(task_id: &str) -> Result<f64> {
    let history_file = common::task_history_file(task_id);
    if !history_file.exists() {
        return Ok(0.0);
    }
    
    let metadata = fs::metadata(&history_file)?;
    Ok(metadata.len() as f64 / (1024.0 * 1024.0))
}

pub fn load_config() -> Result<RotdConfig> {
    let config_path = common::config_path();
    if !config_path.exists() {
        return Ok(RotdConfig::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .context("Failed to read config file")?;
    
    // Remove comments for JSON5/JSONC compatibility
    let json_content = remove_jsonc_comments(&content);
    
    serde_json::from_str(&json_content)
        .context("Failed to parse config file")
}

pub fn save_config(config: &RotdConfig) -> Result<()> {
    let config_path = common::config_path();
    
    // Add helpful comments
    let jsonc_content = format!(
        r#"{{
  // Max uncompressed size per task history before rotation (MiB)
  "history_max_size_mib": {},
  // Compress closed tasks? ("closed" means status == "complete")
  "history_compress_closed": {},
  // Hard cap on total history directory size (MiB)
  "history_total_cap_mib": {}
}}"#,
        config.history_max_size_mib,
        config.history_compress_closed,
        config.history_total_cap_mib
    );
    
    fs::write(&config_path, jsonc_content)
        .context("Failed to write config file")
}

fn remove_jsonc_comments(content: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    let mut chars = content.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => {
                result.push(ch);
                escape_next = true;
            }
            '"' => {
                result.push(ch);
                in_string = !in_string;
            }
            '/' if !in_string => {
                if let Some(&'/') = chars.peek() {
                    // Single-line comment - skip to end of line
                    chars.next(); // consume second '/'
                    while let Some(ch) = chars.next() {
                        if ch == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                } else if let Some(&'*') = chars.peek() {
                    // Multi-line comment - skip to */
                    chars.next(); // consume '*'
                    let mut prev = ' ';
                    while let Some(ch) = chars.next() {
                        if prev == '*' && ch == '/' {
                            break;
                        }
                        prev = ch;
                    }
                } else {
                    result.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }
    
    result
}

pub fn ensure_history_dir() -> Result<()> {
    let history_path = common::task_history_path();
    if !history_path.exists() {
        fs::create_dir_all(&history_path)
            .context("Failed to create task_history directory")?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct TaskHistoryStats {
    pub total_events: usize,
    pub status_counts: HashMap<String, u32>,
    pub agent_contributions: HashMap<String, u32>,
    pub total_pss_delta: f64,
    pub first_event: Option<TaskHistoryEvent>,
    pub last_event: Option<TaskHistoryEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_jsonc_comments() {
        let jsonc = r#"{
  // This is a comment
  "key": "value", // inline comment
  /* Multi-line
     comment */
  "number": 42
}"#;
        
        let cleaned = remove_jsonc_comments(jsonc);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned).unwrap();
        
        assert_eq!(parsed["key"], "value");
        assert_eq!(parsed["number"], 42);
    }
}