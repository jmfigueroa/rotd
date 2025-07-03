# ROTD CLI Utility

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

## 🧩 Human Commands

### `rotd init`
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
- `.rotd/ROTD.md` - Complete methodology
- `.rotd/pss_score.py` - Scoring script

### `rotd score`
Generates PSS (Progress Scoring System) scores for tasks or entire project.

```bash
rotd score                    # Score all tasks
rotd score 6.1               # Score specific task
rotd score --format json    # JSON output
rotd score --format summary # Summary format
```

**Output formats:**
- `table` (default) - Formatted table display
- `json` - Machine-readable JSON
- `summary` - Project health overview

### `rotd check`
Validates ROTD compliance and project health.

```bash
rotd check           # Health check with issues report
rotd check --fix     # Attempt automatic issue resolution
```

**Validates:**
- Directory structure completeness
- JSONL file format validity
- Test summaries for completed tasks
- Absence of stubs in main code
- Session state currency

### `rotd update`
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

**Note:** After updating, use the ROTD update prompts to apply changes to existing projects. See [ROTD Update Protocol](./ROTD_UPDATE_PROTOCOL.md) for detailed process.

### `rotd remove`
Removes ROTD tracking from current project only.

```bash
rotd remove          # Remove with confirmation
rotd remove --yes    # Skip confirmation prompts
```

**⚠️ Warning:** This deletes all task history, test summaries, and lessons learned.

### `rotd uninstall`
Removes ROTD CLI utility system-wide.

```bash
rotd uninstall       # Uninstall with confirmation
rotd uninstall --yes # Skip confirmation prompts
```

**⚠️ Warning:** This removes the `rotd` command entirely.

### `rotd show-task`
Display detailed task information.

```bash
rotd show-task 6.1           # Show task details
rotd show-task 6.1 --verbose # Extended metadata
```

### `rotd show-lessons`
List logged lessons in readable format.

```bash
rotd show-lessons            # All lessons
rotd show-lessons --tag=testing # Filter by tag
```

### `rotd show-audit`
Display recent audit log entries.

```bash
rotd show-audit              # Last 10 entries
rotd show-audit --limit=20   # Last 20 entries
```

### `rotd completions`
Generates shell completion scripts.

```bash
rotd completions bash        # Bash completions
rotd completions zsh         # Zsh completions  
rotd completions fish        # Fish completions
rotd completions powershell  # PowerShell completions
```

## 🤖 Agent Commands

All agent commands output minimal JSON and enforce strict validation. Use `--agent` flag globally or `rotd agent` subcommands.

### `rotd agent update-task`
Update task entries from JSON input.

```bash
# From stdin
echo '{"id":"6.2","status":"complete","updated_at":"2025-07-02T10:00:00Z"}' | rotd agent update-task --timestamp --pss

# From file
rotd agent update-task --file task_update.json --strict
```

**Input Schema:**
```json
{
  "id": "6.2",
  "title": "Optional title update",
  "status": "complete",
  "tests": ["test1.tsx", "test2.tsx"],
  "description": "Task description"
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

**Output:**
```json
{
  "rotd_cli": {
    "agent_commands": {
      "update_task": {
        "usage": "rotd agent update-task [--file FILE] [--strict] [--pss]",
        "purpose": "Update task in tasks.jsonl with validation"
      }
    }
  }
}
```

## 🛠️ Features

- **Color-coded output** - Visual feedback for status and health
- **Interactive prompts** - Safe confirmation for destructive operations
- **JSON schema validation** - Ensures artifact integrity
- **Cross-platform** - Works on Linux, macOS, and Windows
- **Shell completions** - Tab completion for all major shells

## 🎯 Typical Workflows

### Human Developer Workflow

1. **Initialize project:**
   ```bash
   rotd init
   ```

2. **Check project health:**
   ```bash
   rotd check --verbose
   ```

3. **Review task progress:**
   ```bash
   rotd show-task 6.2
   rotd score 6.2 --format summary
   ```

4. **Learn from history:**
   ```bash
   rotd show-lessons --tag=testing
   rotd show-audit --limit=20
   ```

### LLM Agent Workflow

1. **Update task status (agent mode):**
   ```bash
   echo '{"id":"6.2","status":"complete"}' | rotd agent update-task --timestamp --pss
   ```

2. **Log test results:**
   ```bash
   rotd agent append-summary --file test_summaries/6.2.json
   ```

3. **Record lessons learned:**
   ```bash
   echo '{"id":"ll-001","diagnosis":"React import missing","remediation":"Add explicit React import"}' | rotd agent log-lesson
   ```

4. **Update coverage:**
   ```bash
   rotd agent ratchet-coverage 87.5 --task-id 6.2
   ```

5. **Get command reference:**
   ```bash
   rotd agent info
   ```

## 🔧 Configuration

The CLI uses sensible defaults but can be customized through:
- Environment variables (TODO)
- Configuration files (TODO)
- Command-line flags

## 🤝 Integration

Works seamlessly with:
- **Git** - ROTD artifacts track alongside code
- **CI/CD** - `rotd check` validates compliance in pipelines
- **IDEs** - JSON schemas enable IntelliSense
- **LLMs** - Structured prompts in `prompts.md`

## 📊 Progress Scoring

The PSS (Progress Scoring System) evaluates tasks on 10 criteria:

**Execution Sanity (1-3):**
1. LLM Engagement
2. Compilation Success  
3. Core Implementation

**Testing Discipline (4-6):**
4. Tests Written
5. Tests Passing
6. Quality Trajectory Score

**Cleanup Discipline (7-8):**
7. Stub-Free Code
8. Documentation Maintained

**Historical Continuity (9-10):**
9. Project History Maintained
10. Lessons Learned Applied

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

After updating, follow the [ROTD Update Protocol](./ROTD_UPDATE_PROTOCOL.md) to apply changes to your project:

1. **Review Changes**: Check update manifest for what changed
2. **Apply Updates**: Use ROTD update prompts to migrate schemas and workflows  
3. **Verify**: Run `rotd check --strict` to ensure compliance

The CLI creates update manifests and provides structured prompts to help LLMs integrate methodology changes safely.