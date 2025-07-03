# ROTD Buckle Mode

> A recovery protocol for ROTD-compliant projects when compilation or artifact integrity breaks down.

## 📍 Purpose

**Buckle Mode** is a structured recovery state triggered when a project fails to meet minimal ROTD standards for task completeness, such as compilation failure, missing test artifacts, or untracked work.

This document defines when and how to enter Buckle Mode, and what steps must be followed to restore the project to a clean, compliant state.

---

## 🔥 When to Enter Buckle Mode

Enter Buckle Mode if **any** of the following are true at session close:

- ❌ More than **50 compilation errors** (`cargo check` output)
- 📉 Task status is `in_progress` or missing from `.rotd/tasks.jsonl`
- 🧪 `.rotd/test_summaries/<task>.json` is missing or incomplete
- 🚫 No `rotd --agent score` entry for the task
- 📂 `session_state.json` not updated to reflect reality
- 📛 CI pipeline fails for current task
- 🧠 `lessons_learned.jsonl` is empty despite major error occurrence

---

## 🧠 Buckle Mode Protocol

When in Buckle Mode, follow this strict workflow:

### 1. Diagnose Compilation Issues
- Run `cargo check`
- Summarize error types and root causes
- Prioritize fixing high-error files first

### 2. Fix Compilation Errors
- Use LLM or manual edits
- Ensure each fix reduces error count
- Do **not** implement new features

### 3. Run Tests
- Ensure all unit and integration tests pass
- If no tests exist, create minimal coverage

### 4. Write Test Summary
- Add `.rotd/test_summaries/<task_id>.json`
- Must show input/output, coverage notes, and pass/fail counts

### 5. Score the Task
- Run `rotd --agent score <task_id>`
- Provide rationale for each metric

### 6. Log Lessons Learned
- If cause of failure was process-related, log in `lessons_learned.jsonl`
- Include:
  - `diagnosis`
  - `remediation`
  - `tags` (e.g., `compilation`, `testing`, `missing-artifact`)

### 7. Update Session State
- Ensure `.rotd/session_state.json` reflects current task ID and mode

---

## ✅ Exit Criteria

You may exit Buckle Mode when:

- [ ] `cargo check` returns **zero errors**
- [ ] All tests for the task pass
- [ ] `.rotd/test_summaries/<task>.json` exists and is valid
- [ ] Task has an entry in `.rotd/pss_scores.jsonl`
- [ ] Task is marked `complete` in `.rotd/tasks.jsonl`
- [ ] Session state is current
- [ ] No active violations remain in `.rotd/audit.log`

---

## 🧾 Template Prompt for LLM Agent

```
You are now in ROTD Buckle Mode due to failed compilation and incomplete artifacts.

STOP feature work.

📋 Your tasks are:
1. Fix all compilation errors until `cargo check` passes
2. Run all tests and write `.rotd/test_summaries/<task_id>.json`
3. Score the task using `rotd --agent score <task_id>`
4. Log any problems encountered using `rotd agent log-lesson`
5. Update `.rotd/tasks.jsonl` to mark the task as `complete`
6. Confirm session state is up to date

Exit Buckle Mode only when the project passes all compliance checks.
```

---

## CLI Commands

```bash
# Check if Buckle Mode is needed
rotd check --buckle-trigger

# Enter Buckle Mode explicitly
rotd buckle-mode enter <task_id>

# Generate diagnostic report
rotd buckle-mode diagnose

# Run incremental fixes
rotd buckle-mode fix-compilation
rotd buckle-mode fix-artifacts

# Verify exit criteria
rotd buckle-mode check-exit

# Exit Buckle Mode
rotd buckle-mode exit
```

---

## Integration with ROTD Workflow

Buckle Mode integrates with the standard ROTD workflow in these ways:

1. **Automatic Triggering**: CI pipelines can trigger Buckle Mode when detecting integrity issues
2. **Pre-commit Hooks**: Local checks prevent committing in broken states 
3. **Session Boundaries**: Enforces clean task state at session ends
4. **Recovery Path**: Provides clear steps to restore project health
5. **Audit Trail**: All Buckle Mode entries and exits are logged

---

## Task.jsonl Protection

One key protection offered by Buckle Mode is ensuring `.rotd/tasks.jsonl` integrity:

**NEVER** use in-memory task management. **ALWAYS** use `.rotd/tasks.jsonl` for tracking all tasks:

- **Real-time Updates**: Update task status in `.rotd/tasks.jsonl` after every session run
- **Continuity Requirement**: Task state in `.rotd/tasks.jsonl` is the ONLY source of truth
- **Session Boundary Enforcement**: Before ending ANY work session, verify `.rotd/tasks.jsonl` reflects current state
- **Recovery Protection**: In-memory task tracking leads to lost work and violates ROTD principles

Failure to maintain `.rotd/tasks.jsonl` will trigger Buckle Mode.

---

By formalizing Buckle Mode and integrating it into your audit policies, ROTD becomes a resilient, self-correcting development framework—even during chaotic build failures.