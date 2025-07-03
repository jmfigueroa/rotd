# ROTD CLI Command Reference

> Quick reference for all ROTD CLI commands

## 🔧 Global Flags

```bash
--agent           # Agent mode: minimal JSON output, strict validation
--verbose         # Extended output (human mode only)
--dry-run         # Show what would be done without making changes
```

## 🧍 Human Commands

### Project Management
```bash
rotd init [--force]                    # Initialize ROTD project
rotd check [--fix] [--buckle-trigger]      # Health check and compliance
rotd buckle-mode <command> [options]         # Buckle Mode recovery operations
rotd completions <shell>               # Generate shell completions
```

### Information Display
```bash
rotd show-task <task_id> [--verbose]   # Display task details
rotd show-lessons [--tag <tag>]        # List lessons learned
rotd show-audit [--limit <n>]          # Show audit log entries
```

### Scoring
```bash
rotd score <task_id> [--format <fmt>]  # Generate PSS scores
  --format table|json|summary          # Output format
```

## 🤖 Agent Commands

### Task Management
```bash
rotd agent update-task [options]       # Update task from JSON
  --file <file>                        # Read from file instead of stdin
  --strict                             # Enforce strict validation
  --pss                                # Trigger scoring after update
  --timestamp                          # Auto-populate updated_at
```

### Artifact Management  
```bash
rotd agent append-summary --file <file>  # Add test summary
rotd agent log-lesson [--file <file>]    # Log lesson learned
rotd agent ratchet-coverage <pct> [--task-id <id>]  # Update coverage
```

### Information
```bash
rotd agent info                        # Show agent command reference
```

## 📊 Output Formats

### Human Mode
- **Colored text** with status indicators
- **Interactive prompts** for confirmations
- **Verbose tables** with detailed information
- **Help text** and usage examples

### Agent Mode
- **JSON-only output** for programmatic use
- **Minimal responses** with status and errors
- **Schema validation** with detailed error messages
- **No interactive prompts** or color formatting

## 🎯 Common Workflows

### Initialize New Project
```bash
rotd init
rotd check
```

### Complete a Task (Agent)
```bash
echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp --pss
rotd agent append-summary --file test_summaries/6.2.json
```

### Recover from Buckle Mode
```bash
# Enter Buckle Mode
rotd buckle-mode enter 6.2

# Fix compilation issues
rotd buckle-mode fix-compilation

# Fix missing artifacts
rotd buckle-mode fix-artifacts

# Exit when all checks pass
rotd buckle-mode exit
```

### Review Project Health (Human)
```bash
rotd check --verbose
rotd show-audit --limit=10
rotd show-lessons --tag=recent
```

### Log Error Recovery (Agent)
```bash
echo '{"id":"fix-001","diagnosis":"...","remediation":"..."}' | rotd agent log-lesson
```

## 🚨 Error Handling

### Exit Codes
- **0**: Success
- **1**: General error (invalid arguments, file not found)
- **2**: Validation error (invalid JSON, schema violation)
- **3**: ROTD compliance error (missing .rotd directory, failed checks)

### Buckle Mode Commands
```bash
# Check if Buckle Mode is needed
rotd check --buckle-trigger

# Manage Buckle Mode
rotd buckle-mode enter <task_id>             # Enter Buckle Mode for task
rotd buckle-mode diagnose                    # Generate diagnostic report
rotd buckle-mode fix-compilation            # Fix compilation errors
rotd buckle-mode fix-artifacts              # Fix missing artifacts
rotd buckle-mode check-exit                 # Verify exit criteria
rotd buckle-mode exit                       # Exit Buckle Mode
```

### Common Errors
```bash
# No .rotd directory
rotd check
# Error: No .rotd directory found. Run 'rotd init' first.

# Invalid JSON input
echo 'invalid json' | rotd agent update-task
# Error: {"error":"invalid_json","message":"expected value at line 1 column 1"}

# Missing required fields
echo '{"id":""}' | rotd agent update-task --strict
# Error: {"error":"validation_failed","message":"Task ID cannot be empty"}
```

## 🔗 Integration Examples

### Git Hooks
```bash
# pre-commit hook
#!/bin/sh
rotd check || exit 1
```

### CI/CD Pipeline
```bash
# Validate ROTD compliance
rotd --agent check
if [ $? -ne 0 ]; then
  echo "ROTD compliance check failed"
  exit 1
fi
```

### IDE Integration
```bash
# VS Code task
{
  "label": "ROTD Health Check",
  "type": "shell", 
  "command": "rotd check --verbose"
}
```

This reference covers all CLI functionality for both human and agent usage patterns.