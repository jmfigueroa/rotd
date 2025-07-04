# ROTD Prompts

> LLM prompts for consistent ROTD implementation across projects and sessions

## 📑 Table of Contents

1. [Prompt Selection Guide](#-prompt-selection-guide)
2. [Begin ROTD-guided development (CLI-enabled)](#-begin-rotd-guided-development-cli-enabled)
3. [Begin ROTD-guided development (Manual)](#-begin-rotd-guided-development-manual)
4. [Resume ROTD-guided development (CLI-enabled)](#-resume-rotd-guided-development-cli-enabled)
5. [Resume ROTD-guided development (Manual)](#-resume-rotd-guided-development-manual)
6. [Multi-Agent Coordination (CLI-enabled)](#-multi-agent-coordination-cli-enabled)
7. [Multi-Agent Task Planning](#-multi-agent-task-planning)
8. [Convert Existing Project to ROTD (CLI-enabled)](#-convert-existing-project-to-rotd-cli-enabled)
9. [Convert Existing Project to ROTD (Manual)](#-convert-existing-project-to-rotd-manual)
10. [ROTD Preamble (CLI-enabled)](#-rotd-preamble-cli-enabled)
11. [ROTD Preamble (Manual)](#-rotd-preamble-manual)
12. [Quick ROTD Status Check](#quick-rotd-status-check)
13. [ROTD Task Completion (CLI-enabled)](#-rotd-task-completion-cli-enabled)
14. [ROTD Project Cleanup (CLI-enabled)](#-rotd-project-cleanup-cli-enabled)
15. [ROTD Project Cleanup (Manual)](#-rotd-project-cleanup-manual)
16. [ROTD Error Recovery (CLI-enabled)](#-rotd-error-recovery-cli-enabled)
17. [ROTD Progress Review (CLI-enabled)](#-rotd-progress-review-cli-enabled)
18. [ROTD Periodic Review](#-rotd-periodic-review)
19. [ROTD Update Application](#-rotd-update-application)
20. [Prompt Usage Guidelines](#-prompt-usage-guidelines)

---

## 📋 Prompt Selection Guide

- **With CLI**: Use CLI-enabled prompts when `rotd` command is available
- **Without CLI**: Use manual prompts for environments without the CLI tool
- **Multi-Agent**: Use coordination prompts when multiple agents work on same project (v1.3+)
- **Legacy**: Keep manual prompts as fallback for older projects

[↑ Back to Table of Contents](#-table-of-contents)

---

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
- `coordination/`: Multi-agent coordination directory (v1.3+)

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

[↑ Back to Table of Contents](#-table-of-contents)

---

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

[↑ Back to Table of Contents](#-table-of-contents)

---

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
```

This session should leave the project *cleaner*, not just bigger.

ROTD = Runtime-Oriented Test Discipline. Stay compliant. Use the CLI.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

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

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🤝 Multi-Agent Coordination (CLI-enabled)

Use this prompt when multiple agents work on the same ROTD project (v1.3+):

~~~markdown
You are operating as one of multiple agents working on a ROTD project with multi-agent coordination enabled.

🤝 **Multi-Agent Setup**:
1. **Set Agent ID**: Export `ROTD_AGENT_ID=<unique-id>` or let system generate UUID
2. **Declare Capabilities**: Set `ROTD_AGENT_CAPS=backend_rust,tests_only` based on your strengths
3. **Set Skill Level**: Export `ROTD_AGENT_SKILL=intermediate` (entry|intermediate|expert)

📋 **Coordination Workflow**:
```bash
# Start your session
rotd coord beat                          # Signal you're alive
rotd coord ls                            # See available work

# Claim appropriate task
rotd coord claim --capability backend_rust  # Claim by your capability
rotd coord claim --skill-level <=intermediate  # Or by skill level

# Work on task
rotd show-task <task_id> --verbose       # Understand the task
# ... implement solution ...
echo '{"id":"<task_id>","status":"complete"}' | rotd agent update-task --timestamp
rotd agent append-summary --file test_summaries/<task_id>.json

# Release when done
rotd coord release <task_id>             # Mark done and release lock

# Communicate if needed
rotd coord msg "Blocked on API design review for task X.Y"
```

🔒 **Coordination Rules**:
- **One task per agent**: Cannot claim while holding another task
- **Respect dependencies**: Tasks with incomplete dependencies won't be claimable
- **Update heartbeat**: Run `rotd coord beat` every 60 seconds (auto-timeout after 15 min)
- **Clean exit**: Always release tasks before stopping work

📊 **Monitor Progress**:
```bash
rotd coord ls --verbose                  # See all tasks and owners
rotd coord quota                         # Check usage limits
cat .rotd/coordination/coordination.log  # Review agent messages
```

⚠️ **Conflict Avoidance**:
- File locks prevent concurrent writes to ROTD artifacts
- Task locks prevent duplicate work
- Dependencies ensure correct order
- Priority system focuses effort on important tasks

Your agent ID: Run `echo $ROTD_AGENT_ID` to confirm.
Your capabilities: Run `echo $ROTD_AGENT_CAPS` to verify.

Work efficiently, communicate blockers, and maintain ROTD compliance.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 📋 Multi-Agent Task Planning

Use this prompt for Opus or a planning agent to seed tasks for multiple agents:

~~~markdown
You are the planning agent (Opus) responsible for breaking down development work into tasks for multiple specialized agents.

📋 **Task Planning for Multi-Agent Execution**:

1. **Analyze Requirements**:
   - Read the development specification or feature request
   - Identify major components and their relationships
   - Consider testing, documentation, and integration needs

2. **Create Task Breakdown**:
   Output JSONL format with these fields:
   ```json
   {
     "id": "X.Y",
     "title": "Clear, specific task title",
     "description": "Detailed task requirements",
     "priority": "urgent|high|medium|low",
     "deps": ["dependency_ids"],
     "capability": "frontend_ts|backend_rust|tests_only|docs|refactor",
     "skill_level": "entry|intermediate|expert",
     "estimated_hours": 2.5
   }
   ```

3. **Capability Assignment Guidelines**:
   - `frontend_ts`: UI components, React, TypeScript, styling
   - `backend_rust`: Core logic, APIs, data structures, algorithms
   - `tests_only`: Test writing, test fixing, coverage improvement
   - `docs`: Documentation, README updates, API docs
   - `refactor`: Code cleanup, optimization, technical debt

4. **Skill Level Guidelines**:
   - `entry`: Simple changes, clear requirements, minimal context
   - `intermediate`: Standard features, some design decisions, moderate complexity
   - `expert`: Architecture decisions, complex algorithms, deep system knowledge

5. **Priority Guidelines**:
   - `urgent`: Blocking other work or critical path
   - `high`: Core functionality, should be done soon
   - `medium`: Standard features, nice to have
   - `low`: Cleanup, optimization, future nice-to-haves

6. **Dependency Rules**:
   - List all tasks that must complete before this one
   - Consider both technical and logical dependencies
   - Avoid circular dependencies
   - Minimize dependency chains where possible

**Example Output for Modal System**:
```jsonl
{"id":"4.1","title":"Create modal context and provider","description":"React context for modal state management","priority":"high","deps":[],"capability":"frontend_ts","skill_level":"intermediate","estimated_hours":2}
{"id":"4.2","title":"Implement base modal component","description":"Reusable modal with animations","priority":"high","deps":["4.1"],"capability":"frontend_ts","skill_level":"intermediate","estimated_hours":3}
{"id":"4.3","title":"Add keyboard navigation","description":"Focus trap and escape handling","priority":"medium","deps":["4.2"],"capability":"frontend_ts","skill_level":"expert","estimated_hours":2}
{"id":"4.4","title":"Write modal component tests","description":"Unit and integration tests","priority":"high","deps":["4.2"],"capability":"tests_only","skill_level":"intermediate","estimated_hours":2}
{"id":"4.5","title":"Document modal API","description":"Usage examples and props documentation","priority":"medium","deps":["4.2","4.4"],"capability":"docs","skill_level":"entry","estimated_hours":1}
```

Pipe output directly to: `rotd agent seed-tasks --stdin --validate`

Focus on creating atomic, well-defined tasks that different agents can work on independently.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

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

8. **Enable multi-agent support** (v1.3+):
   ```bash
   # Create coordination structure
   mkdir -p .rotd/coordination/{agent_locks,file_locks,heartbeat,.lock}
   
   # Initialize registry from tasks
   rotd coord migrate-tasks  # If available, or create manually
   ```

🧠 **Report when finished**:
- List how many tasks were added
- What the overall test coverage appears to be
- How many scores were recorded and what the score range is
- Summarize any critical lessons learned from retroactive import
- Confirm multi-agent coordination is ready

This is a structural conversion. Going forward, all development MUST follow ROTD compliance, using CLI commands only.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🔄 Convert Existing Project to ROTD (Manual)

Use this prompt to migrate an existing codebase to the ROTD structure manually:

~~~markdown
You are converting an existing project into a Runtime-Oriented Test Discipline (ROTD) environment. This project was not originally developed under ROTD, but you will now retroactively align it with ROTD principles and artifact formats.

📁 **Directory Setup**:
- Create a `.rotd/` directory at the project root
- Inside `.rotd/`, create the following files and folders if they don't exist:
  - `tasks.jsonl` (append-only list of all tracked tasks)
  - `test_summaries/` (a folder with one JSON file per task containing test results)
  - `lessons_learned.jsonl` (append-only log of failure patterns and remediations)
  - `pss_scores.jsonl` (append-only list of progress scores with rationale)
  - `session_state.json` (JSON object with the current task, timestamp, and optional notes)
  - `coverage_history.json` (for tracking test coverage over time)
  - `audit.log` (text file for manual logging of stubs, violations, and policy exceptions)
  - `coordination/` (v1.3+ multi-agent support directory)

🧭 **Conversion Steps**:
1. **List Known Tasks**:
   - Review the project's features and modules
   - Manually enter each as an object into `tasks.jsonl` using JSONL format
     ```json
     {"id":"1.1","title":"User login flow","status":"complete","priority":"high","capability":"backend_rust","skill_level":"intermediate"}
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

7. **Enable Multi-Agent Support** (v1.3+):
   - Create coordination directory structure
   - Initialize `active_work_registry.json` with incomplete tasks
   - Set up `dependency_map.json` based on task relationships

📋 **Final Report**:
Once the conversion is complete, summarize:
- How many tasks were created
- How many test summaries were backfilled
- The average and median PSS scores
- Key failure patterns identified
- Whether the project is now ready to proceed under strict ROTD rules
- Multi-agent coordination readiness status

From this point forward, the project MUST follow the ROTD discipline: structured tasks, runtime-validated progress, and complete artifact tracking.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🎯 ROTD Preamble (CLI-enabled)

Add this preamble when the CLI is available:

~~~markdown
**IMPORTANT**: You are operating under ROTD: Runtime-Oriented Test Discipline — a test-anchored, artifact-driven strategy optimized for LLM-led development. 

* 🔧 **CLI Available**: Use `rotd` commands for all operations. Never manually edit .rotd/ files.
* 📊 **Always defer to CLI outputs**: `rotd check`, `rotd show-task`, `rotd show-lessons`
* 🤖 **Use agent mode**: `rotd agent update-task`, `rotd agent log-lesson`, `rotd agent info`
* 🤝 **Multi-agent aware**: Use `rotd coord` commands if working alongside other agents
* ✅ **Keep up to date**: Update `tasks.jsonl` when you start and finish tasks. Keep `session_state.json` and other applicable ROTD artifacts up to date when you finish a session run.
* 🗣️ **Report ROTD each time**: When you finish a session run, always frame your report through an ROTD lens

Stay aligned with task ID traceability, test coverage requirements, and rationale logging. Proceed with the current task through the lens of ROTD.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 📋 ROTD Preamble (Manual)

Add this preamble when CLI is NOT available:

~~~markdown
You are operating under ROTD: Runtime-Oriented Test Discipline — a test-anchored, artifact-driven strategy optimized for LLM-led development. Always defer to `.rotd` artifacts for task status, test summaries, lessons learned, and scoring. Stay aligned with task ID traceability, test coverage requirements, and rationale logging. Proceed with the current task through the lens of ROTD.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## Quick ROTD Status Check

Use this prompt to get a rapid overview of project health:

~~~markdown
Provide a quick ROTD status report:

1. **Current Task**: Read `.rotd/session_state.json` - what task is active?
2. **Test Health**: Scan `.rotd/test_summaries/` - how many tasks have 100% pass rates?
3. **Score Trends**: Review last 3 entries in `.rotd/pss_scores.jsonl` - are scores improving?
4. **Known Issues**: Check `.rotd/lessons_learned.jsonl` - any recent failure patterns?
5. **Compliance**: Any violations in `.rotd/audit.log` requiring attention?
6. **Multi-Agent Status** (v1.3+): Check `.rotd/coordination/active_work_registry.json` - any blocked tasks?

Format as a concise status table with recommendations for next steps.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🛠️ ROTD Task Completion (CLI-enabled)

Use this prompt when completing a specific task with CLI:

~~~markdown
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
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🧹 ROTD Project Cleanup (CLI-enabled)

Use this prompt to clean up technical debt with CLI:

~~~markdown
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
- Clean stale locks: `rotd coord clean-stale` (v1.3+)

📊 **Health Check Focus**:
- Project structure completeness
- JSONL file validity
- Test summaries for completed tasks
- Absence of stubs in main code
- Session state currency
- Multi-agent lock hygiene (v1.3+)

Example cleanup session:
```bash
rotd check --verbose                    # Identify issues
rotd show-audit --limit=20             # Review recent problems
rotd coord clean-stale                 # Clean stale agent locks
# Fix identified issues...
rotd agent log-lesson < lesson.json    # Log any new lessons
rotd check                             # Verify fixes
```

Report findings and create action items for any issues discovered.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🧹 ROTD Project Cleanup (Manual)

Use this prompt when CLI is not available:

~~~markdown
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
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🚨 ROTD Error Recovery (CLI-enabled)

Use this prompt when encountering errors or failures:

~~~markdown
Recover from errors using ROTD CLI workflow:

🔍 **Error Analysis**:
1. **Check Recent History**: `rotd show-audit --limit=10` for recent violations
2. **Review Lessons**: `rotd show-lessons --tag=error` for similar past issues
3. **Assess Health**: `rotd check --verbose` for current project state
4. **Multi-Agent Check**: `rotd coord ls` to see if other agents are blocked

💡 **Recovery Steps**:
1. **Log the Lesson**: Use `rotd agent log-lesson` to record the error and solution
2. **Update Task Status**: If task is blocked, update with context
3. **Document Context**: Include error details in task description
4. **Communicate**: `rotd coord msg "Error: <description>"` if multi-agent

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
- Release task if unable to continue: `rotd coord release <task_id>`

The goal is to turn every error into shared knowledge for future sessions.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 📊 ROTD Progress Review (CLI-enabled)

Use this prompt for progress assessment:

~~~markdown
Review ROTD project progress using CLI tools:

📈 **Progress Assessment Workflow**:
1. **Overall Health**: `rotd check --verbose` for project status
2. **Task Overview**: Review all tasks and their completion status
3. **Score Analysis**: `rotd score <task_id> --format summary` for recent tasks
4. **Lesson Review**: `rotd show-lessons` to understand learning trajectory
5. **Audit History**: `rotd show-audit --limit=20` to spot patterns
6. **Multi-Agent Activity**: `rotd coord ls --verbose` to see agent collaboration

🎯 **Key Metrics to Report**:
- Project health percentage from `rotd check`
- Task completion ratio (completed vs total)
- Average PSS scores from recent tasks
- Number of lessons learned and their categories
- Recent audit trends (errors, warnings, info)
- Active agents and their current tasks (v1.3+)

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
echo "Active Agents:"
rotd coord ls --verbose
echo ""
echo "Recent Activity:"
rotd show-audit --limit=5
```

🔍 **Focus Areas**:
- Identify tasks with low PSS scores for improvement
- Review lessons learned for patterns
- Check audit log for recurring issues
- Assess test coverage trends
- Monitor multi-agent efficiency

Use this data to plan next development priorities and identify areas needing attention.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🔍 ROTD Periodic Review

Use this prompt for systematic ROTD review:

~~~markdown
Perform a comprehensive ROTD periodic review to ensure project health, compliance, and strategic alignment.

📦 **Review Checklist**:

1. **Artifact Health**
   - Verify all `.rotd/` files are present and up-to-date
   - Check for tasks marked complete without test summaries
   - Identify missing PSS scores for completed tasks
   - Review staleness of lessons_learned.jsonl
   - Verify coordination directory integrity (v1.3+)

2. **ROTD Compliance**
   - Ensure tasks marked complete have 100% passing tests
   - Search for stub annotations (#[rotd_stub]) in complete tasks
   - Verify no TODOs exist on main branch
   - Review audit log violations and patterns
   - Check multi-agent lock hygiene

3. **Project Alignment**
   - Compare current tasks against project roadmap
   - Check task priority distribution
   - Identify any low-priority tasks completed before high-priority ones
   - Assess progress toward current milestone
   - Review agent capability utilization (v1.3+)

4. **Drift Detection**
   - Find tasks stuck in "in_progress" for too long
   - Monitor test coverage trends
   - Check for duplicate lessons learned entries
   - Identify recurring audit violations
   - Detect idle agents or unclaimed high-priority tasks

5. **Task Prioritization Review**
   - Review urgent tasks are being addressed first
   - Check for priority inversions in completion order
   - Assess if deferred tasks should remain deferred
   - Look for tasks needing priority adjustment
   - Verify dependency chains are respected

📊 **Review Output Format**:
Generate a markdown report with:
- Artifact health status (X/10)
- Compliance score (X/10)
- Alignment assessment
- Drift signals detected
- Corrective actions (numbered list)
- Priority adjustments needed
- Multi-agent efficiency score (if applicable)
- Overall project health score

🎯 **Key Questions**:
- Are high-priority tasks truly getting attention first?
- Is technical debt accumulating through stub usage?
- Are lessons being learned and applied?
- Is the project converging toward its goals?
- Are agents working efficiently together? (v1.3+)

Example review structure:
```
## ROTD Periodic Review: [Date]

### ✅ Artifact Health
- Status of each artifact type
- Missing or outdated items

### 🧠 ROTD Compliance 
- Rule violations found
- Test integrity issues

### 📈 Project Alignment
- Roadmap vs actual progress
- Priority adherence

### ⚠️ Drift Signals
- Concerning patterns
- Technical debt indicators

### 🤝 Multi-Agent Performance (v1.3+)
- Agent utilization
- Coordination efficiency

### 🛠️ Corrective Actions
1. Specific action item
2. Another action item

### 📊 Health Score
- Overall: X/10
- Breakdown by category
```

This review should be performed weekly or at major milestones to maintain project discipline and momentum.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 🔄 ROTD Update Application

Use these prompts when applying ROTD methodology updates to existing projects:

### Update Assessment and Planning

~~~markdown
ROTD methodology has been updated. Apply the latest changes to this project.

📋 **Update Process**:

1. **Review Changes**
   - Check `.rotd/update_manifest.json` for list of updates
   - Identify which changes affect this project
   - Note any schema or workflow modifications

2. **Backup Current State**
   ```bash
   mkdir -p .rotd/backup
   cp .rotd/*.jsonl .rotd/backup/
   cp .rotd/*.json .rotd/backup/
   cp -r .rotd/coordination .rotd/backup/ # v1.3+
   ```

3. **Report Update Plan**
   List all changes that need to be applied:
   - Schema updates needed
   - New workflows to implement
   - Documentation to update
   - Multi-agent features to enable (v1.3+)

Begin by analyzing the current project state and the required updates.
~~~

### Apply Schema Updates

~~~markdown
Apply ROTD schema updates based on the update manifest.

**For Multi-Agent Support Update (v1.3.0)**:

📝 **Migration Steps**:
1. Read each task from `.rotd/tasks.jsonl`
2. Add new fields if missing:
   - `capability`: Assign based on task type (frontend_ts|backend_rust|tests_only|docs|refactor)
   - `skill_level`: Default to "intermediate"
3. Create coordination directory structure:
   ```bash
   mkdir -p .rotd/coordination/{agent_locks,file_locks,heartbeat,.lock}
   ```
4. Initialize work registry from incomplete tasks
5. Build dependency map from task relationships

**Verification**:
```bash
# Verify all tasks have new fields
cat .rotd/tasks.jsonl | jq -r '.capability' | grep -v null

# Check coordination setup
rotd coord ls

# Test multi-agent claiming
ROTD_AGENT_ID=test rotd coord claim --any
```

**Example Update**:
```json
// Before
{"id":"4.2","title":"Add validation","status":"in_progress","priority":"high"}

// After  
{"id":"4.2","title":"Add validation","status":"in_progress","priority":"high","capability":"backend_rust","skill_level":"intermediate"}
```

Apply these updates systematically and report any issues encountered.
~~~

### Implement New Workflows

~~~markdown
Implement new ROTD workflow features in this project.

**For Multi-Agent Coordination (v1.3.0)**:

🔄 **Implementation Steps**:

1. **Enable Coordination**
   - Ensure coordination directory exists
   - Initialize active_work_registry.json
   - Set up dependency_map.json
   - Create empty quota.json

2. **Update Development Workflow**
   ```bash
   # Single agent becomes:
   rotd update-task ...
   
   # Multi-agent becomes:
   rotd coord claim
   rotd update-task ...
   rotd coord release
   ```

3. **Document Agent Setup**
   Add to project README:
   - How to set ROTD_AGENT_ID
   - Available capabilities
   - Coordination commands

4. **Test Multi-Agent Flow**
   - Start two terminal sessions
   - Set different ROTD_AGENT_ID in each
   - Verify both can claim different tasks
   - Test heartbeat and stale recovery

**Validation Checklist**:
- [ ] Coordination directory structure exists
- [ ] Work registry populated with tasks
- [ ] Dependencies properly mapped
- [ ] Agents can claim/release tasks
- [ ] Heartbeat mechanism working
- [ ] Stale lock recovery tested

Report completion status for each step.
~~~

### Post-Update Verification

~~~markdown
Verify ROTD updates have been applied correctly.

✅ **Verification Checklist**:

1. **Schema Compliance**
   ```bash
   rotd check --strict
   ```
   - All tasks have required new fields
   - JSON/JSONL files parse correctly
   - No data was lost during migration

2. **Workflow Integration**
   - New processes are documented
   - Schedule files created where needed
   - Team instructions updated
   - Multi-agent coordination working

3. **Update History**
   Add entry to `.rotd/update_history.jsonl`:
   ```json
   {"version":"1.3.0","updated_at":"2025-07-04T14:00:00Z","updated_by":"Claude","status":"success","changes_applied":["multi_agent_coordination","capability_routing","artifact_locking"]}
   ```

4. **Final Health Check**
   ```bash
   rotd check
   rotd score --format summary
   rotd coord ls  # For v1.3+
   ```

**Success Criteria**:
- No validation errors
- All new features accessible
- Project health maintained or improved
- Update history logged
- Multi-agent coordination operational

Report final status and any recommendations for the team.
~~~

[↑ Back to Table of Contents](#-table-of-contents)

---

## 📚 Prompt Usage Guidelines

### CLI vs Manual Selection
- **Check CLI availability**: Run `rotd --version` to verify CLI is installed
- **Use CLI prompts**: When CLI is available for better validation and automation
- **Multi-agent prompts**: Use coordination prompts for v1.3+ projects with multiple agents
- **Fallback to manual**: Only when CLI cannot be installed or is unavailable
- **Hybrid approach**: Start with CLI, fallback to manual if CLI issues occur

### Customization
- **Project-specific**: Modify prompts to include project-specific task IDs or phases
- **Team standards**: Adapt language and requirements to team preferences
- **Tool integration**: Include additional tools (IDEs, CI/CD) as needed
- **Agent capabilities**: Customize capability lists based on your agent mix

### Best Practices
- **Always specify mode**: CLI-enabled vs manual in your prompt selection
- **Include examples**: Real commands and JSON structures help LLM understanding
- **Emphasize validation**: CLI prompts stress using tools for verification
- **Maintain consistency**: Use the same prompt style across a project
- **Multi-agent awareness**: Consider coordination needs even in single-agent mode

These prompts ensure consistent ROTD implementation and help maintain the discipline across different development sessions and team members, with clear distinction between CLI-enabled, manual, and multi-agent workflows.

[↑ Back to Table of Contents](#-table-of-contents)