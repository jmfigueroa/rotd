# ROTD Audit Policy Rule: `audit.buckle.trigger.001`

> Enforces automatic entry into Buckle Mode when a session ends in a non-compliant state.

---

## 🧾 Rule Metadata

- **Rule ID**: `audit.buckle.trigger.001`
- **Applies To**: All session-end checkpoints and `rotd check --enforce`
- **Severity**: 🛑 Critical (blocks task completion, commits, and merges)
- **Target Artifacts**:
  - `.rotd/tasks.jsonl`
  - `.rotd/test_summaries/*.json`
  - `.rotd/pss_scores.jsonl`
  - `.rotd/session_state.json`
  - `.rotd/audit.log`
- **Trigger Context**: Session close or pre-commit hook

---

## 🔍 Trigger Conditions

This rule is violated if **any** of the following conditions are met:

1. `cargo check` reports more than **50 errors**
2. Task status in `.rotd/tasks.jsonl` is not `"complete"` after reported "completion"
3. `.rotd/test_summaries/<task_id>.json` is missing or invalid
4. No PSS score entry exists for a task marked as complete
5. `.rotd/session_state.json` contains outdated data
6. CI pipeline has failed for the current task

---

## 🚫 Enforcement Actions

When this rule is triggered:

1. **Immediate**: Block any commit/merge attempts
2. **Process**: Force entry into Buckle Mode
3. **Monitoring**: Log violation to `.rotd/audit.log`
4. **Recovery**: Guide LLM or developer through recovery steps

---

## 🛠️ Remediation Process

1. **Assessment**: Run `rotd check --strict` to identify all violations
2. **Focus**: Fix compilation errors first (if present)
3. **Testing**: Run and fix all failing tests
4. **Documentation**: Generate all missing artifacts
5. **Validation**: Update task status only after all checks pass
6. **Learning**: Document causes in `.rotd/lessons_learned.jsonl`

---

## 📝 Example Violation Log

```json
{
  "ts": "2025-07-03T14:22:10Z",
  "rule": "audit.buckle.trigger.001",
  "severity": "critical",
  "task": "6.2",
  "msg": "Session ended with 53 compilation errors and no test summary",
  "mode_transition": "buckle_mode_activated"
}
```

---

## 🧪 Implementation Notes

This rule forms a critical safety net for ROTD projects. It protects against:

- **Artifact Drift**: Where code and documentation become desynchronized
- **False Completions**: Tasks marked complete without proper validation
- **Lost Context**: Where session state is not properly captured
- **In-Memory Task Tracking**: The dangerous practice of tracking tasks only in agent memory

The rule should be evaluated at every session close and commit attempt to ensure no integrity violations occur.

---

## 📈 Integration with CLI

The ROTD CLI provides commands to work with this audit rule:

```bash
# Check if rule would trigger
rotd audit check audit.buckle.trigger.001

# Get detailed diagnostics
rotd audit diagnose audit.buckle.trigger.001

# Force rule execution
rotd audit enforce audit.buckle.trigger.001

# View audit log entries for this rule
rotd audit log --rule=audit.buckle.trigger.001
```

---

## 📚 Related Documentation

- [ROTD Buckle Mode](../recovery/BUCKLE_MODE.md) - The recovery protocol triggered by this rule
- [Task Tracking Requirements](../workflows/TASK_TRACKING.md) - Best practices for maintaining task state
- [Audit System Overview](../audit/OVERVIEW.md) - How the ROTD audit system works