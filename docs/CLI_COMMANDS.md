# ROTD CLI Command Reference

> Agent-aware Rust CLI tool for Runtime-Oriented Test Discipline project management

## 🧠 Philosophy

Traditional tools assume human context. ROTD projects **offload much development to LLMs**, but humans remain **final validators and orchestrators**.

The `rotd` CLI has two primary modes:
- **Human Mode**: Friendly output, interactive prompts, colored text
- **Agent Mode (`--agent`)**: Minimal JSON output, strict validation, schema enforcement

## 📦 Installation

Install via Cargo from GitHub:

```bash
cargo install --git https://github.com/jmfigueroa/rotd --branch main
```

## 🔧 Global Flags

```bash
--agent           # Agent mode: minimal JSON output, strict validation
--verbose         # Extended output (human mode only)
--dry-run         # Show what would be done without making changes
```

## 🧍 Human Commands

### Project Management

#### `rotd init`
Creates `.rotd/` directory structure and template files for a new ROTD project.

```bash
rotd init              # Initialize ROTD in current directory
rotd init --force      # Overwrite existing .rotd directory
```

**Creates:**
- `.rotd/tasks.jsonl` - Task tracking
- `.rotd/test_summaries/` - Test results directory
- `.rotd/session_state.json` - Session continuity
- `.rotd/coverage_history.json` - Coverage tracking
- `.rotd/coordination/` - Multi-agent coordination (v1.3+)
- `.rotd/ROTD.md` - Complete methodology
- `.rotd/pss_score.py` - Scoring script

#### `rotd check`
Validates ROTD compliance and project health.

```bash
rotd check                      # Health check with issues report
rotd check --fix                # Attempt automatic issue resolution
rotd check --buckle-trigger     # Check if Buckle Mode needed
```

**Validates:**
- Directory structure completeness
- JSONL file format validity
- Test summaries for completed tasks
- Absence of stubs in main code
- Session state currency

#### `rotd buckle-mode`
Buckle Mode recovery operations for critical failures.

```bash
rotd buckle-mode enter <task_id>     # Enter Buckle Mode for task
rotd buckle-mode diagnose            # Generate diagnostic report
rotd buckle-mode fix-compilation     # Fix compilation errors
rotd buckle-mode fix-artifacts       # Fix missing artifacts
rotd buckle-mode check-exit          # Verify exit criteria
rotd buckle-mode exit                # Exit Buckle Mode
```

#### `rotd update`
Updates ROTD methodology from remote repository.

```bash
rotd update          # Update with confirmation
rotd update --yes    # Skip confirmation prompts
rotd update --check  # Check for available updates
```

**Updates:**
- `.rotd/ROTD.md` - Core methodology
- `.rotd/pss_score.py` - Scoring system
- `.rotd/prompts.md` - LLM prompts
- Creates update manifest for migration tracking

#### `rotd remove`
Removes ROTD tracking from current project only.

```bash
rotd remove          # Remove with confirmation
rotd remove --yes    # Skip confirmation prompts
```

**⚠️ Warning:** This deletes all task history, test summaries, and lessons learned.

### Information Display

#### `rotd show-task`
Display detailed task information.

```bash
rotd show-task 6.1           # Show task details
rotd show-task 6.1 --verbose # Extended metadata
```

#### `rotd show-lessons`
List logged lessons in readable format.

```bash
rotd show-lessons               # All lessons
rotd show-lessons --tag=testing # Filter by tag
```

#### `rotd show-audit`
Display recent audit log entries.

```bash
rotd show-audit              # Last 10 entries
rotd show-audit --limit=20   # Last 20 entries
```

### Scoring

#### `rotd score`
Generates PSS (Progress Scoring System) scores for tasks or entire project.

```bash
rotd score                   # Score all tasks
rotd score 6.1               # Score specific task
rotd score --format json     # JSON output
rotd score --format summary  # Summary format
```

**Output formats:**
- `table` (default) - Formatted table display
- `json` - Machine-readable JSON
- `summary` - Project health overview

### Utilities

#### `rotd completions`
Generates shell completion scripts.

```bash
rotd completions bash        # Bash completions
rotd completions zsh         # Zsh completions  
rotd completions fish        # Fish completions
rotd completions powershell  # PowerShell completions
```

#### `rotd version`
Show version information.

```bash
rotd version            # CLI version
rotd version --project  # Project ROTD version
rotd version --latest   # Latest available version
```

#### `rotd validate`
Validate ROTD artifacts against schemas.

```bash
rotd validate --all                  # Validate all schemas
rotd validate --schema task          # Validate specific schema
rotd validate --strict               # Strict validation mode
```

## 🤝 Multi-Agent Coordination Commands (v1.3+)

### `rotd coord claim`
Claim the next available task based on priority and capabilities.

```bash
rotd coord claim                          # Claim highest priority task
rotd coord claim --capability backend_rust # Filter by capability
rotd coord claim --skill-level <=intermediate # Filter by skill level
rotd coord claim --any                    # Ignore priority ordering
```

**Capabilities:** `frontend_ts`, `backend_rust`, `tests_only`, `docs`, `refactor`
**Skill Levels:** `entry`, `intermediate`, `expert`

### `rotd coord release`
Release a claimed task, marking it as done.

```bash
rotd coord release <task_id>   # Release and mark task done
```

### `rotd coord approve`
Approve a task in review status.

```bash
rotd coord approve <task_id>   # Approve task for completion
```

### `rotd coord msg`
Append message to coordination log.

```bash
rotd coord msg "Blocked on API design review"
```

### `rotd coord beat`
Update agent heartbeat to signal liveness.

```bash
rotd coord beat                # Touch heartbeat file
```

### `rotd coord clean-stale`
Clean stale locks from timed-out agents and rotate logs.

```bash
rotd coord clean-stale              # Default 15min timeout
rotd coord clean-stale --timeout 1800  # 30min timeout
```

### `rotd coord quota`
Track and update quota usage.

```bash
rotd coord quota                    # Show current quota
rotd coord quota --add 1500        # Add 1500 tokens to usage
```

### `rotd coord ls`
List current work registry with task statuses.

```bash
rotd coord ls                       # List all tasks
rotd coord ls --verbose             # Show additional details
```

## 🤖 Agent Commands

All agent commands output minimal JSON and enforce strict validation. Use `--agent` flag globally or `rotd agent` subcommands.

### `rotd agent update-task`
Update task entries from JSON input.

```bash
# From stdin
echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp --pss

# From file
rotd agent update-task --file task_update.json --strict
```

**Options:**
- `--file <file>` - Read from file instead of stdin
- `--strict` - Enforce strict validation
- `--pss` - Trigger scoring after update
- `--timestamp` - Auto-populate updated_at

**Input Schema:**
```json
{
  "id": "6.2",
  "title": "Optional title update",
  "status": "complete",
  "tests": ["test1.tsx", "test2.tsx"],
  "description": "Task description",
  "priority": "high",
  "capability": "backend_rust",
  "skill_level": "intermediate"
}
```

### `rotd agent append-summary`
Add test summary to project.

```bash
rotd agent append-summary --file test_summaries/6.2.json
```

**Input Schema:**
```json
{
  "task_id": "6.2",
  "status": "complete",
  "total_tests": 35,
  "passed": 30,
  "failed": 5,
  "coverage": 0.857,
  "verified_by": "Claude Code",
  "timestamp": "2025-07-02T10:00:00Z"
}
```

### `rotd agent log-lesson`
Log lesson learned from JSON input.

```bash
echo '{"id":"fix-router","diagnosis":"BrowserRouter conflict","remediation":"Remove nested Router"}' | rotd agent log-lesson
rotd agent log-lesson --file lesson.json
```

### `rotd agent ratchet-coverage`
Update coverage floor if threshold exceeded.

```bash
rotd agent ratchet-coverage 85.7 --task-id 6.2
```

### `rotd agent info`
Show minified command reference for LLM agents.

```bash
rotd agent info
```

### `rotd agent seed-tasks`
Seed tasks from JSON input (for multi-agent task planning).

```bash
cat task_plan.jsonl | rotd agent seed-tasks --stdin --validate
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

### Multi-Agent Development (v1.3+)
```bash
# Agent 1: Claim and work on task
ROTD_AGENT_ID=agent-1 rotd coord beat
ROTD_AGENT_ID=agent-1 rotd coord claim --capability backend_rust
# ... work on task ...
echo '{"id":"1.5","status":"complete"}' | rotd agent update-task --timestamp
ROTD_AGENT_ID=agent-1 rotd coord release 1.5

# Agent 2: Different capability
ROTD_AGENT_ID=agent-2 rotd coord claim --capability tests_only
```

### Complete a Task (Single Agent)
```bash
echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp --pss
rotd agent append-summary --file test_summaries/6.2.json
```

### Recover from Buckle Mode
```bash
# Enter Buckle Mode
rotd buckle-mode enter 6.2

# Fix issues
rotd buckle-mode fix-compilation
rotd buckle-mode fix-artifacts

# Exit when ready
rotd buckle-mode exit
```

### Review Project Health
```bash
rotd check --verbose
rotd show-audit --limit=10
rotd show-lessons --tag=recent
rotd coord ls  # If using multi-agent
```

## 🚨 Error Handling

### Exit Codes
- **0**: Success
- **1**: General error (invalid arguments, file not found)
- **2**: Validation error (invalid JSON, schema violation)
- **3**: ROTD compliance error (missing .rotd directory, failed checks)
- **4**: Lock timeout (multi-agent coordination)
- **5**: Stale lock cleared (retry operation)

### Common Errors
```bash
# No .rotd directory
rotd check
# Error: No .rotd directory found. Run 'rotd init' first.

# Invalid JSON input
echo 'invalid json' | rotd agent update-task
# Error: {"error":"invalid_json","message":"expected value at line 1 column 1"}

# Lock timeout (multi-agent)
rotd coord claim
# Error: E_LOCK_TIMEOUT
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

### Environment Variables
```bash
ROTD_AGENT_ID=my-agent-uuid    # Persistent agent identifier
ROTD_AGENT_CAPS=backend_rust,tests_only  # Agent capabilities
ROTD_AGENT_SKILL=intermediate  # Agent skill level
```

## 📈 Health Monitoring

Use `rotd check` and `rotd score` regularly to maintain project health:

- **Score 9-10**: Excellent ROTD compliance
- **Score 7-8**: Good, minor cleanup needed
- **Score 6**: Passing threshold
- **Score <6**: Requires remediation

## 🔄 Updates

The methodology evolves. Use `rotd update` to sync with latest practices:

```bash
rotd update         # Fetches latest ROTD.md, scoring, and prompts
rotd update --check # Check for available updates without applying
```

After updating, follow the [ROTD Update Protocol](./ROTD_UPDATE_PROTOCOL.md) to apply changes to your project.

This reference covers all CLI functionality for both human and agent usage patterns, including v1.3 multi-agent coordination features.