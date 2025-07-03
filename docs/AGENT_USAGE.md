# ROTD Agent Usage Guide

> Quick reference for LLM agents using the ROTD CLI

## 🚀 Installation Check

```bash
# Verify rotd is installed
rotd --version

# Get agent command reference
rotd agent info
```

## 📝 Essential Agent Commands

### Update Task Status
```bash
# Complete a task with timestamp
echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp --pss

# Update from file with strict validation
rotd agent update-task --file task.json --strict
```

### Log Test Results
```bash
# Add test summary for completed task
rotd agent append-summary --file test_summaries/6.2.json
```

### Record Lessons Learned
```bash
# Log a lesson from experience
echo '{"id":"fix-001","diagnosis":"Missing React import","remediation":"Add explicit import React"}' | rotd agent log-lesson
```

### Update Coverage
```bash
# Trigger coverage ratchet if threshold exceeded
rotd agent ratchet-coverage 87.5 --task-id 6.2
```

### Check Project Health
```bash
# Minimal health check (agent mode)
rotd --agent check
```

## 📊 JSON Schemas

### Task Update Schema
```json
{
  "id": "6.2",
  "title": "Task title (optional)",
  "status": "pending|in_progress|complete|blocked|scaffolded", 
  "tests": ["test1.tsx", "test2.tsx"],
  "description": "Task description",
  "phase": "6",
  "priority": "urgent|high|medium|low|deferred",
  "priority_score": 75.5
}
```

### Test Summary Schema
```json
{
  "task_id": "6.2",
  "status": "complete",
  "total_tests": 35,
  "passed": 30,
  "failed": 5,
  "coverage": 0.857,
  "verified_by": "Claude Code",
  "timestamp": "2025-07-02T10:00:00Z",
  "notes": "Optional notes"
}
```

### Lesson Learned Schema
```json
{
  "id": "unique-lesson-id",
  "diagnosis": "Problem description",
  "remediation": "Solution applied",
  "tags": ["testing", "react", "imports"],
  "context": {
    "task_id": "6.2",
    "component": "HelpModal"
  }
}
```

## 🎯 Common Patterns

### Complete Task Workflow
```bash
# 1. Update task to complete
echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp

# 2. Log test results
rotd agent append-summary --file test_summaries/6.2.json

# 3. Score the task
rotd --agent score 6.2 --format json

# 4. Update coverage if needed
rotd agent ratchet-coverage 85.7 --task-id 6.2
```

### Error Handling Pattern
```bash
# Log lesson when encountering known issue
echo '{"id":"router-conflict","diagnosis":"BrowserRouter nesting","remediation":"Remove wrapper Router in tests"}' | rotd agent log-lesson

# Update task with failure context
echo '{"id":"6.2","status":"blocked","description":"Test environment issues"}' | rotd agent update-task --timestamp
```

### Task Prioritization Pattern
```bash
# Set task priority when creating/updating
echo '{"id":"6.3","priority":"urgent","priority_score":95.0}' | rotd agent update-task --timestamp

# Decision logic for priorities:
# - urgent: Blocking other tasks or breaking CI
# - high: Critical path for milestone
# - medium: Normal development tasks
# - low: Nice-to-have improvements
# - deferred: Intentionally postponed
```

## ⚠️ Agent Mode Rules

1. **Always use JSON output** - No colored text or verbose messages
2. **Validate input** - Use `--strict` for schema enforcement
3. **Log operations** - All updates are automatically audited
4. **Check errors** - Non-zero exit codes indicate failures
5. **Use dry-run** - Test operations with `--dry-run` first

## 🔍 Output Examples

### Successful Task Update
```json
{"status":"success","action":"update_task","task_id":"6.2"}
```

### Health Check Output
```json
{"health_score":4,"total_checks":5,"passed":4,"issues":["missing_test_summaries"],"health_percentage":80.0}
```

### PSS Score Output
```json
{"task_id":"6.2","score":8,"timestamp":"2025-07-02T10:00:00Z"}
```

## 🛠️ Troubleshooting

### Common Issues
- **"No .rotd directory found"** → Run `rotd init` first
- **"Invalid JSON"** → Validate JSON syntax before piping
- **"Task not found"** → Check task ID exists in tasks.jsonl
- **"Validation failed"** → Use schema examples above

### Debug Commands
```bash
# Check project structure
rotd --agent check

# View recent audit log
rotd show-audit --limit=5

# Get command help
rotd agent info
```

This guide provides everything an LLM agent needs to effectively use the ROTD CLI for project management and artifact tracking.