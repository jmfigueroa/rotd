# ROTD Update Verification Checklist

> Use this checklist after applying ROTD methodology updates to ensure everything works correctly

## Pre-Update Backup ✅

- [ ] Created `.rotd/backup/` directory
- [ ] Copied all `.jsonl` files to backup
- [ ] Copied all `.json` files to backup  
- [ ] Documented current ROTD version
- [ ] Noted active tasks and their status

## Schema Migration Verification ✅

### Task Schema (if updated)
- [ ] All tasks have required new fields
- [ ] Priority fields use valid enum values (`urgent|high|medium|low|deferred`)
- [ ] Priority scores are between 0-100 (if present)
- [ ] No tasks lost during migration
- [ ] JSON syntax is valid in all entries

```bash
# Verify task schema
cat .rotd/tasks.jsonl | jq empty  # Should complete without errors
cat .rotd/tasks.jsonl | jq -r '.priority' | sort | uniq  # Check priority values
```

### Other Schema Updates
- [ ] Test summaries format updated (if applicable)
- [ ] PSS scores format updated (if applicable)
- [ ] Lessons learned format updated (if applicable)
- [ ] All JSON files parse correctly

```bash
# Check all JSON files
find .rotd -name "*.json" -exec jq empty {} \;
find .rotd -name "*.jsonl" -exec jq empty {} \;
```

## Workflow Integration ✅

### New Processes Added
- [ ] Periodic review schedule created (if applicable)
- [ ] New templates copied to project
- [ ] Documentation updated with new processes
- [ ] Team notified of workflow changes

### Process Validation
- [ ] All new CLI commands work
- [ ] New prompts are accessible
- [ ] Integration points tested
- [ ] No conflicts with existing workflows

## ROTD Compliance Check ✅

```bash
# Run comprehensive health check
rotd check --strict
```

- [ ] No validation errors reported
- [ ] All directory structure intact
- [ ] All required files present
- [ ] Schema validation passes
- [ ] Audit log shows no new violations

## Project Health Verification ✅

```bash
# Check project still builds/compiles
npm test  # or cargo test, etc.

# Score recent tasks to ensure PSS still works
rotd score --format summary
```

- [ ] Project still compiles/builds
- [ ] All tests still pass
- [ ] PSS scoring works correctly
- [ ] No functionality regressions
- [ ] Performance acceptable

## Update History Logging ✅

Add entry to `.rotd/update_history.jsonl`:

```json
{
  "version": "1.2.0",
  "updated_at": "2025-07-03T10:00:00Z", 
  "updated_by": "Claude",
  "status": "success",
  "changes_applied": ["task_priority", "periodic_review"],
  "migration_notes": "Added priority field to 12 existing tasks"
}
```

- [ ] Update history entry created
- [ ] Version number correct
- [ ] All applied changes listed
- [ ] Status reflects actual outcome

## Post-Update Actions ✅

### Team Communication
- [ ] Team notified of completed update
- [ ] New features documented for team
- [ ] Training/guidance provided if needed
- [ ] Update process feedback collected

### Next Steps Planning  
- [ ] First periodic review scheduled (if new feature)
- [ ] Priority assignments reviewed (if new feature)
- [ ] Cleanup tasks identified
- [ ] Monitoring plan for new features

## Rollback Preparation ✅

- [ ] Rollback procedure documented
- [ ] Backup location confirmed accessible
- [ ] Rollback triggers identified
- [ ] Emergency contact information available

## Final Sign-off ✅

**Update Successful** 
- [ ] All checklist items completed
- [ ] No critical issues identified  
- [ ] Project health maintained or improved
- [ ] Ready for normal development to resume

**Completion Signature:**
- Updated by: ________________
- Date: ________________  
- ROTD Version: ________________
- Notes: ________________

---

## Troubleshooting Common Issues

### Schema Validation Fails
1. Check JSON syntax with `jq`
2. Compare against schema files in `/schema/`
3. Restore from backup if corrupted
4. Re-run migration manually

### CLI Commands Not Working
1. Update CLI to latest version: `rotd update`
2. Check command syntax in documentation
3. Verify `.rotd/` directory structure
4. Clear any cached data

### Performance Issues
1. Check file sizes haven't grown unexpectedly
2. Verify no infinite loops in new workflows
3. Monitor resource usage
4. Consider optimization if needed

### Integration Problems
1. Test new features in isolation
2. Check for conflicts with existing tools
3. Review team workflow compatibility
4. Adjust configuration as needed

---

*Keep this checklist with your project documentation for future updates*