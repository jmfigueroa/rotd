# ROTD Prompts

> LLM prompts for consistent ROTD implementation across projects and sessions

## 📋 Prompt Selection Guide

- **With CLI**: Use CLI-enabled prompts when `rotd` command is available
- **Without CLI**: Use manual prompts for environments without the CLI tool
- **Legacy**: Keep manual prompts as fallback for older projects

## 🚀 Begin ROTD-guided development (CLI-enabled)

Use this prompt when the `rotd` CLI is available:

~~~markdown
You are operating in a Runtime-Oriented Test Discipline (ROTD) project with CLI support. Your job is to assist in development while maintaining strict alignment with the ROTD workflow and artifacts.

🔧 **CLI Available**: Use `rotd` commands for all ROTD operations
- Check project health: `rotd check`
- Update tasks: `echo '{"id":"X.Y","status":"complete"}' | rotd agent update-task --timestamp --pss`
- Log test results: `rotd agent append-summary --file test_summaries/X.Y.json`
- Record lessons: `echo '{"id":"lesson-id","diagnosis":"...","remediation":"..."}' | rotd agent log-lesson`
- Get agent help: `rotd agent info`

📂 **Project Structure**: The `.rotd/` directory persists task and test state:
- `tasks.jsonl`: Canonical list of all project tasks and their test coverage
- `test_summaries/`: Structured test results (per task)
- `lessons_learned.jsonl`: Problem/solution pairs from past sessions
- `pss_scores.jsonl`: Evaluation metrics for completed work
- `session_state.json`: Markers to resume from last completed effort
- `audit.log`: Violations and policy overrides (do not ignore)

✅ **Before coding, you must**:
1. Run `rotd check` to verify project health
2. Use `rotd show-lessons` to avoid repeating prior errors
3. Confirm the current task ID exists and has tests (or write them if not)
4. Use CLI commands for all ROTD operations - DO NOT manually edit .rotd files

🧠 **Report your next planned step** in the context of the current task:
- What test or function you're writing
- Whether you are continuing, refactoring, or starting fresh
- Current task status from `rotd show-task <id>`

**IMPORTANT**: Use `rotd agent` commands for all updates. The CLI ensures validation, audit logging, and artifact integrity.

ROTD compliance is mandatory.
~~~

## 📋 Begin ROTD-guided development (Manual)

Use this prompt when the CLI is NOT available:

~~~markdown
You are operating in a Runtime-Oriented Test Discipline (ROTD) project. Your job is to assist in development while maintaining strict alignment with the ROTD workflow and artifacts.

📂 The project uses the `.rotd/` directory to persist task and test state:
- `tasks.jsonl`: Canonical list of all project tasks and their test coverage
- `test_summaries/`: Structured test results (per task)
- `lessons_learned.jsonl`: Problem/solution pairs from past sessions
- `pss_scores.jsonl`: Evaluation metrics for completed work
- `session_state.json`: Markers to resume from last completed effort
- `audit.log`: Violations and policy overrides (do not ignore)

✅ Before coding, you must:
1. Load `session_state.json` to resume the last known task
2. Read `lessons_learned.jsonl` to avoid repeating prior errors
3. Confirm the current task ID exists and has tests (or write them if not)
4. Strictly follow ROTD process — test-first preferred, or "Now & Later" if scaffolded

🧠 Report your next planned step in the context of the current task, include:
- What test or function you're writing
- Whether you are continuing, refactoring, or starting fresh
- Whether this task is marked `in_progress`, `scaffolded`, or `complete`

Do not skip any enforcement logic unless explicitly permitted in `.rotd/audit.log`.

ROTD compliance is mandatory.
~~~

## 🔄 Resume ROTD-guided development (CLI-enabled)

Use this prompt when continuing work on an existing ROTD project with CLI:

~~~markdown
This is NOT a new project session — you are continuing an active ROTD-compliant project where prior tasks have been worked on and artifacts were generated.

🔧 **CLI Commands Available**: Use these for all ROTD operations:

**Before continuing development:**

📊 **ROTD Status Check**
1. Run `rotd check --verbose` to get comprehensive health report
2. Use `rotd show-audit --limit=10` to see recent violations
3. Run `rotd show-lessons --tag=recent` to review relevant lessons

📋 **Task Management**
- View current task: `rotd show-task <current_id> --verbose`
- Check incomplete tasks: Look for `"status": "in_progress"` or `"status": "scaffolded"`
- Update task status: `echo '{"id":"X.Y","status":"complete"}' | rotd agent update-task --timestamp --pss`

🧠 **Your Responsibilities Now**
- Use CLI for ALL ROTD operations - never manually edit .rotd files
- Validate all operations with `rotd check` before and after changes
- Echo current task ID and summarize what you intend to do next
- Use `rotd --agent score <task_id>` to track progress

**Example Session Start:**
```bash
rotd check                          # Health overview
rotd show-task 6.2 --verbose       # Current task details
rotd show-lessons --tag=testing    # Recent lessons

This session should leave the project *cleaner*, not just bigger.

ROTD = Runtime-Oriented Test Discipline. Stay compliant. Use the CLI.
~~~

## 🔄 Resume ROTD-guided development (Manual)

Use this prompt when continuing work without CLI:

~~~markdown
This is NOT a new project session — you are continuing an active ROTD-compliant project where prior tasks have been worked on and artifacts were generated.

Before continuing development:

🔄 **ROTD Maintenance Actions**
1. Scan `.rotd/tasks.jsonl` and identify any tasks with:
   - `"status": "in_progress"` or `"status": "scaffolded"` but missing `test_summaries/*`
   - Tasks marked for later implementation but no tests yet
2. Load `lessons_learned.jsonl` and confirm it contains entries relevant to recent failures. If not, retroactively add any insights or fixes from last session.
3. Check `audit.log` for unresolved violations. Summarize any that may require human escalation or correction.
4. If `session_state.json` is out-of-date, update it to point to the current task before continuing.

📊 **ROTD Progress Check**
Parse `.rotd/pss_scores.jsonl` to:
- Show the **current task's score (0–10)** and breakdown per metric.
- Identify any incomplete scores for recent tasks (missing rationale or pending postmortem).
- Suggest low-effort improvements (e.g., fill missing docs, fix stubs, add documentation).

🧠 **Your Responsibilities Now**
- Maintain ROTD artifacts before generating code.
- Re-check that all code being resumed still compiles.
- Echo current task ID and summarize what you intend to do next.
- List the last score and your plan to raise or finalize it (if score < 10).

This session should leave the project *cleaner*, not just bigger.

ROTD = Runtime-Oriented Test Discipline. Stay compliant.
~~~

## 🔄 Convert Existing Project to ROTD (CLI-enabled)

Use this prompt to migrate an existing codebase into ROTD using the CLI:

~~~markdown
You are converting an existing software project to use Runtime-Oriented Test Discipline (ROTD). This project was not originally structured for ROTD, but you will now retroactively bring it into compliance using the `rotd` CLI.

🔧 **Migration Steps**:
1. **Initialize** the ROTD structure:
   ```bash
   rotd init
   ```

2. **Enumerate key tasks** in the current project and log them:
   - Use `rotd task new` or `rotd agent update-task` to create entries in `tasks.jsonl`
   - For each existing feature, create a scaffolded or complete task

3. **Backfill test results**:
   - Add test outputs to `.rotd/test_summaries/` with `rotd agent append-summary`
   - Use real test results when possible

4. **Retro-score recent tasks**:
   - Run `rotd --agent score <task_id>` to assign initial scores
   - Edit rationale fields to explain retroactive judgments

5. **Log prior lessons**:
   - Add failure insights from project history to `lessons_learned.jsonl`
   - Use `rotd agent log-lesson` for proper formatting

6. **Audit hygiene**:
   - Review and log any code hygiene issues (e.g., TODOs, stubs) with `rotd agent log-violation`

7. **Snapshot session state**:
   - Set up `session_state.json` to track current task for LLM context

🧠 **Report when finished**:
- List how many tasks were added
- What the overall test coverage appears to be
- How many scores were recorded and what the score range is
- Summarize any critical lessons learned from retroactive import

This is a structural conversion. Going forward, all development MUST follow ROTD compliance, using CLI commands only.
~~~

## 🔄 Convert Existing Project to ROTD (Manual)

Use this prompt to migrate an existing codebase to the ROTD structure manually:

~~~markdown
You are converting an existing project into a Runtime-Oriented Test Discipline (ROTD) environment. This project was not originally developed under ROTD, but you will now retroactively align it with ROTD principles and artifact formats.

📁 **Directory Setup**:
- Create a `.rotd/` directory at the project root
- Inside `.rotd/`, create the following files and folders if they don’t exist:
  - `tasks.jsonl` (append-only list of all tracked tasks)
  - `test_summaries/` (a folder with one JSON file per task containing test results)
  - `lessons_learned.jsonl` (append-only log of failure patterns and remediations)
  - `pss_scores.jsonl` (append-only list of progress scores with rationale)
  - `session_state.json` (JSON object with the current task, timestamp, and optional notes)
  - `coverage_history.json` (for tracking test coverage over time)
  - `audit.log` (text file for manual logging of stubs, violations, and policy exceptions)

🧭 **Conversion Steps**:
1. **List Known Tasks**:
   - Review the project’s features and modules
   - Manually enter each as an object into `tasks.jsonl` using JSONL format
     ```json
     {"id":"1.1","title":"User login flow","status":"complete"}
     ```

2. **Document Test Results**:
   - For each completed feature, record test pass/fail data in a JSON file under `test_summaries/`
   - Use `{task_id}.json` naming (e.g., `test_summaries/1.1.json`)
   - Include total/pass/fail counts and notes if tests do not exist

3. **Score the Work**:
   - For each task, assign a Progress Scoring System (PSS) score out of 10
   - Append the result to `pss_scores.jsonl`:
     ```json
     {"task_id":"1.1","score":8,"rationale":{"compiles":true,"core_complete":true,"tests_written":true,...}}
     ```

4. **Log Lessons**:
   - Identify past project issues and add them to `lessons_learned.jsonl` with root cause and fix
   - Include tags and affected components for reuse

5. **Set Up Session State**:
   - Create or update `session_state.json` to indicate the current task ID and timestamp:
     ```json
     {"current_task_id":"1.2","resumed":"2025-07-02T15:30:00Z"}
     ```

6. **Audit Artifacts**:
   - Create a text-based `audit.log` listing stubs, `TODO` markers, or other incomplete work
   - Use it to track ROTD violations or exemptions going forward

📋 **Final Report**:
Once the conversion is complete, summarize:
- How many tasks were created
- How many test summaries were backfilled
- The average and median PSS scores
- Key failure patterns identified
- Whether the project is now ready to proceed under strict ROTD rules

From this point forward, the project MUST follow the ROTD discipline: structured tasks, runtime-validated progress, and complete artifact tracking.
~~~


## 🎯 ROTD Preamble (CLI-enabled)

Add this preamble when the CLI is available:

```
You are operating under ROTD: Runtime-Oriented Test Discipline — a test-anchored, artifact-driven strategy optimized for LLM-led development. 

🔧 **CLI Available**: Use `rotd` commands for all operations. Never manually edit .rotd/ files.
📊 **Always defer to CLI outputs**: `rotd check`, `rotd show-task`, `rotd show-lessons`
🤖 **Use agent mode**: `rotd agent update-task`, `rotd agent log-lesson`, `rotd agent info`

Stay aligned with task ID traceability, test coverage requirements, and rationale logging. Proceed with the current task through the lens of ROTD.
```

## 📋 ROTD Preamble (Manual)

Add this preamble when CLI is NOT available:

```
You are operating under ROTD: Runtime-Oriented Test Discipline — a test-anchored, artifact-driven strategy optimized for LLM-led development. Always defer to `.rotd` artifacts for task status, test summaries, lessons learned, and scoring. Stay aligned with task ID traceability, test coverage requirements, and rationale logging. Proceed with the current task through the lens of ROTD.
```

## Quick ROTD Status Check

Use this prompt to get a rapid overview of project health:

```
Provide a quick ROTD status report:

1. **Current Task**: Read `.rotd/session_state.json` - what task is active?
2. **Test Health**: Scan `.rotd/test_summaries/` - how many tasks have 100% pass rates?
3. **Score Trends**: Review last 3 entries in `.rotd/pss_scores.jsonl` - are scores improving?
4. **Known Issues**: Check `.rotd/lessons_learned.jsonl` - any recent failure patterns?
5. **Compliance**: Any violations in `.rotd/audit.log` requiring attention?

Format as a concise status table with recommendations for next steps.
```

## 🛠️ ROTD Task Completion (CLI-enabled)

Use this prompt when completing a specific task with CLI:

```
Complete the specified ROTD task using CLI workflow:

🎯 **Task Completion Workflow**:
1. **Start**: `rotd show-task <task_id> --verbose` to understand current state
2. **Work**: Implement the required functionality and tests
3. **Test**: Run tests and ensure they pass
4. **Log Results**: `rotd agent append-summary --file test_summaries/<task_id>.json`
5. **Complete**: `echo '{"id":"<task_id>","status":"complete"}' | rotd agent update-task --timestamp --pss`
6. **Verify**: `rotd check` to ensure project health

📊 **Required Artifacts**:
- Test summary with passing tests
- Updated task status
- Any lessons learned logged via `rotd agent log-lesson`

🚫 **Do NOT**:
- Mark task complete without 100% passing tests
- Skip logging test summaries
- Manually edit .rotd files

Example completion:
```bash
# Check current task
rotd show-task 6.2 --verbose

# After implementing and testing
rotd agent append-summary --file test_summaries/6.2.json
echo '{"id":"6.2","status":"complete","description":"Modal shortcuts implemented"}' | rotd agent update-task --timestamp --pss

# Verify completion
rotd --agent score 6.2
rotd check
```

Task is only complete when CLI confirms 100% test passage and artifacts are properly logged.
```

## 🧹 ROTD Project Cleanup (CLI-enabled)

Use this prompt to clean up technical debt with CLI:

```
Perform ROTD project maintenance and cleanup using CLI tools:

🔧 **Cleanup Workflow**:
1. **Health Check**: `rotd check --verbose` to identify all issues
2. **Review History**: `rotd show-audit --limit=20` for recent problems
3. **Learn from Past**: `rotd show-lessons` to avoid repeating mistakes
4. **Fix Issues**: Address each item found by health check
5. **Validate**: `rotd check` again to confirm fixes

🧹 **Cleanup Tasks**:
- Remove any `#[rotd_stub]` annotations and implement properly
- Ensure all TODO comments are tracked or resolved
- Update outdated test summaries using `rotd agent append-summary`
- Log any missing lessons with `rotd agent log-lesson`
- Score unscored completed tasks with `rotd --agent score <task_id>`

📊 **Health Check Focus**:
- Project structure completeness
- JSONL file validity
- Test summaries for completed tasks
- Absence of stubs in main code
- Session state currency

Example cleanup session:
```bash
rotd check --verbose                    # Identify issues
rotd show-audit --limit=20             # Review recent problems
# Fix identified issues...
rotd agent log-lesson < lesson.json    # Log any new lessons
rotd check                             # Verify fixes
```

Report findings and create action items for any issues discovered.
```

## 🧹 ROTD Project Cleanup (Manual)

Use this prompt when CLI is not available:

```
Perform ROTD project maintenance and cleanup:

🧹 **Cleanup Tasks**
1. Remove any `#[rotd_stub]` annotations and implement or properly defer them
2. Ensure all TODO comments follow proper format and are tracked in `tasks.jsonl`
3. Update any outdated test summaries in `.rotd/test_summaries/`
4. Consolidate duplicate entries in `.rotd/lessons_learned.jsonl`
5. Verify all completed tasks have corresponding PSS scores

📊 **Health Check**
1. Run the test suite and validate all test summaries are accurate
2. Check that compilation passes cleanly
3. Verify documentation is up to date
4. Score any unscored completed tasks using the PSS framework

🎯 **Optimization**
Focus on improving the lowest-scoring recent tasks first. Prioritize test coverage, documentation, and artifact maintenance over new features.

Report findings and create action items in `tasks.jsonl` for any issues discovered.
```

## 🚨 ROTD Error Recovery (CLI-enabled)

Use this prompt when encountering errors or failures:

```
Recover from errors using ROTD CLI workflow:

🔍 **Error Analysis**:
1. **Check Recent History**: `rotd show-audit --limit=10` for recent violations
2. **Review Lessons**: `rotd show-lessons --tag=error` for similar past issues
3. **Assess Health**: `rotd check --verbose` for current project state

💡 **Recovery Steps**:
1. **Log the Lesson**: Use `rotd agent log-lesson` to record the error and solution
2. **Update Task Status**: If task is blocked, update with context
3. **Document Context**: Include error details in task description

📝 **Lesson Logging Template**:
```bash
echo '{
  "id": "error-$(date +%s)",
  "diagnosis": "Specific error description",
  "remediation": "Exact steps taken to fix",
  "tags": ["error", "component-name", "error-type"],
  "context": {
    "task_id": "current_task",
    "error_type": "compilation|test|runtime",
    "component": "affected_component"
  }
}' | rotd agent log-lesson
```

🎯 **Recovery Verification**:
- Run `rotd check` to ensure system stability
- Use `rotd --agent score <task_id>` to assess impact
- Update task status appropriately with learned context

The goal is to turn every error into shared knowledge for future sessions.
```

## 📊 ROTD Progress Review (CLI-enabled)

Use this prompt for progress assessment:

```
Review ROTD project progress using CLI tools:

📈 **Progress Assessment Workflow**:
1. **Overall Health**: `rotd check --verbose` for project status
2. **Task Overview**: Review all tasks and their completion status
3. **Score Analysis**: `rotd score <task_id> --format summary` for recent tasks
4. **Lesson Review**: `rotd show-lessons` to understand learning trajectory
5. **Audit History**: `rotd show-audit --limit=20` to spot patterns

🎯 **Key Metrics to Report**:
- Project health percentage from `rotd check`
- Task completion ratio (completed vs total)
- Average PSS scores from recent tasks
- Number of lessons learned and their categories
- Recent audit trends (errors, warnings, info)

📊 **Progress Report Template**:
```bash
echo "ROTD Progress Report $(date)"
echo "=========================="
rotd check --verbose
echo ""
echo "Recent Task Scores:"
# Score last 3-5 completed tasks
echo ""
echo "Lessons Learned Summary:"
rotd show-lessons | head -10
echo ""
echo "Recent Activity:"
rotd show-audit --limit=5
```

🔍 **Focus Areas**:
- Identify tasks with low PSS scores for improvement
- Review lessons learned for patterns
- Check audit log for recurring issues
- Assess test coverage trends

Use this data to plan next development priorities and identify areas needing attention.
```

---

## 📚 Prompt Usage Guidelines

### CLI vs Manual Selection
- **Check CLI availability**: Run `rotd --version` to verify CLI is installed
- **Use CLI prompts**: When CLI is available for better validation and automation
- **Fallback to manual**: Only when CLI cannot be installed or is unavailable
- **Hybrid approach**: Start with CLI, fallback to manual if CLI issues occur

### Customization
- **Project-specific**: Modify prompts to include project-specific task IDs or phases
- **Team standards**: Adapt language and requirements to team preferences
- **Tool integration**: Include additional tools (IDEs, CI/CD) as needed

### Best Practices
- **Always specify mode**: CLI-enabled vs manual in your prompt selection
- **Include examples**: Real commands and JSON structures help LLM understanding
- **Emphasize validation**: CLI prompts stress using tools for verification
- **Maintain consistency**: Use the same prompt style across a project

These prompts ensure consistent ROTD implementation and help maintain the discipline across different development sessions and team members, with clear distinction between CLI-enabled and manual workflows.
