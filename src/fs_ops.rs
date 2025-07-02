use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::schema::*;

pub fn read_jsonl<T>(file_path: &Path) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read {}", file_path.display()))?;

    let mut items = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let item: T = serde_json::from_str(line)
            .context(format!("Invalid JSON on line {} in {}", line_num + 1, file_path.display()))?;
        
        items.push(item);
    }

    Ok(items)
}

pub fn append_jsonl<T>(file_path: &Path, item: &T) -> Result<()>
where
    T: Serialize,
{
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create parent directory")?;
    }

    let json_line = serde_json::to_string(item)
        .context("Failed to serialize item")?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context("Failed to open file for appending")?;

    writeln!(file, "{}", json_line)
        .context("Failed to write to file")?;

    Ok(())
}

pub fn write_json<T>(file_path: &Path, item: &T) -> Result<()>
where
    T: Serialize,
{
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create parent directory")?;
    }

    let json_content = serde_json::to_string_pretty(item)
        .context("Failed to serialize item")?;

    fs::write(file_path, json_content)
        .context("Failed to write file")?;

    Ok(())
}

pub fn read_json<T>(file_path: &Path) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read {}", file_path.display()))?;

    serde_json::from_str(&content)
        .context(format!("Failed to parse JSON from {}", file_path.display()))
}

pub fn append_line(file_path: &Path, line: &str) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create parent directory")?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context("Failed to open file for appending")?;

    writeln!(file, "{}", line)
        .context("Failed to write to file")?;

    Ok(())
}

pub fn read_stdin() -> Result<String> {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;
    Ok(buffer.trim().to_string())
}

// Safe file operations with validation
pub fn safe_update_task(task: &TaskEntry, dry_run: bool) -> Result<()> {
    task.validate()?;
    
    if dry_run {
        println!("Would update task: {}", serde_json::to_string_pretty(task)?);
        return Ok(());
    }

    append_jsonl(&crate::common::tasks_path(), task)
}

pub fn safe_append_summary(summary: &TestSummary, dry_run: bool) -> Result<()> {
    summary.validate()?;
    
    let file_path = crate::common::test_summary_file(&summary.task_id);
    
    if dry_run {
        println!("Would write test summary to: {}", file_path.display());
        println!("{}", serde_json::to_string_pretty(summary)?);
        return Ok(());
    }

    write_json(&file_path, summary)
}

pub fn safe_log_lesson(lesson: &LessonLearned, dry_run: bool) -> Result<()> {
    lesson.validate()?;
    
    if dry_run {
        println!("Would append lesson: {}", serde_json::to_string_pretty(lesson)?);
        return Ok(());
    }

    append_jsonl(&crate::common::lessons_path(), lesson)
}