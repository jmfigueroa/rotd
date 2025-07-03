# ROTD Update Protocol

> Systematic approach for evolving ROTD methodology while maintaining project integrity

## Overview

As ROTD evolves to incorporate new practices and improvements, existing projects need a reliable way to adopt these changes without disrupting ongoing work. This protocol ensures smooth transitions between ROTD versions.

## Version Tracking

### Update Manifest
Each ROTD update includes a manifest file `.rotd/update_manifest.json`:

```json
{
  "version": "1.2.0",
  "date": "2025-07-03",
  "changes": [
    {
      "type": "feature",
      "component": "task_schema",
      "description": "Added priority field with 5-level system",
      "breaking": false,
      "migration_required": true
    },
    {
      "type": "feature", 
      "component": "workflow",
      "description": "Added periodic review process",
      "breaking": false,
      "migration_required": false
    }
  ],
  "previous_version": "1.1.0"
}
```

## Update Process

### 1. Pull Updates (Human)
```bash
# Update ROTD CLI to latest version
rotd update

# Or manually pull documentation updates
git pull origin main
```

### 2. Check Current Version
```bash
# Check project ROTD version
rotd version --project

# Compare with latest
rotd version --latest
```

### 3. Apply Updates (LLM)
Use the update prompt (see below) to have the LLM:
- Review change manifest
- Apply schema migrations
- Update existing artifacts
- Verify compliance

### 4. Verification
```bash
# Run comprehensive compliance check
rotd check --strict

# Validate updated schemas
rotd validate --all
```

## Update Categories

### 1. Schema Updates
Changes to JSON/JSONL structures in artifacts.

**Example**: Adding priority field to tasks
- **Migration**: Add default priority to existing tasks
- **Validation**: Ensure all tasks have valid priority values
- **Rollback**: Store original schema version

### 2. Workflow Updates  
New processes or modifications to existing workflows.

**Example**: Periodic review process
- **Migration**: None required (additive)
- **Validation**: Check review schedule created
- **Rollback**: Not needed (optional feature)

### 3. Tool Updates
New CLI commands or modified behavior.

**Example**: New `rotd review` command
- **Migration**: Update documentation references
- **Validation**: Test new commands work
- **Rollback**: Keep old command aliases

## LLM Update Prompts

### Initial Update Assessment
```
ROTD Update Required: Review and apply methodology updates to this project.

📋 **Update Manifest Location**: `.rotd/update_manifest.json`

1. **Read Update Manifest**
   - Identify all changes since current version
   - Note any breaking changes
   - List required migrations

2. **Backup Critical Artifacts**
   - Create `.rotd/backup/` directory
   - Copy current tasks.jsonl, schemas, and scores
   - Log backup completion

3. **Report Update Plan**
   - List all changes to apply
   - Identify risks or conflicts
   - Estimate completion time
```

### Schema Migration Prompt
```
Apply ROTD Schema Updates:

📦 **Migration Task**: Update task.jsonl schema from v1.1.0 to v1.2.0

**Changes to Apply**:
1. Add "priority" field (enum: urgent|high|medium|low|deferred)
2. Add optional "priority_score" field (0-100)

**Migration Steps**:
1. Read each line of `.rotd/tasks.jsonl`
2. Parse JSON object
3. Add default priority based on:
   - "blocked" status → "urgent"
   - "in_progress" → "high"  
   - "pending" → "medium"
   - "complete" → "low"
4. Write updated line to new file
5. Validate all entries
6. Replace original with backup

**Verification**:
- All tasks have valid priority field
- Schema validation passes
- No data loss occurred
```

### Workflow Addition Prompt
```
Implement New ROTD Workflow Feature:

🔄 **New Feature**: Periodic Review Process

**Implementation Steps**:
1. Create review schedule in `.rotd/review_schedule.json`
2. Set initial review for next Monday
3. Add review template to project docs
4. Update project README with review process

**Integration Checklist**:
- [ ] Review schedule created
- [ ] Template accessible
- [ ] Team notified of new process
- [ ] First review date set

**Validation Command**: `rotd check --feature periodic-review`
```

## Update Verification Checklist

### Pre-Update
- [ ] Current version identified
- [ ] Backup directory created
- [ ] All artifacts backed up
- [ ] Active tasks documented

### During Update  
- [ ] Manifest reviewed
- [ ] Each change applied
- [ ] Migrations completed
- [ ] No data corruption

### Post-Update
- [ ] All validations pass
- [ ] `rotd check --strict` succeeds
- [ ] Project remains functional
- [ ] Version updated in manifest

## Rollback Procedures

### Automatic Rollback
```bash
# Restore from backup if update fails
rotd rollback --auto

# Or manual restoration
cp .rotd/backup/* .rotd/
```

### Rollback Triggers
- Schema validation failures
- Data corruption detected
- Breaking changes without migration
- Project health score drops below threshold

## Change Communication

### Update Announcement Template
```markdown
## ROTD Update: v1.2.0

### What's New
- Task prioritization system (5 levels + optional scoring)
- Periodic review process for project health
- Enhanced schema validation

### Migration Required
- Existing tasks will receive default priorities
- Run update prompt to apply changes

### Breaking Changes
- None

### How to Update
1. Pull latest ROTD version
2. Use update prompt with your LLM
3. Verify with `rotd check --strict`
```

## Best Practices

### For ROTD Maintainers
1. **Semantic Versioning**: Use MAJOR.MINOR.PATCH format
2. **Migration Scripts**: Provide automated migrations when possible
3. **Clear Documentation**: Explain every change thoroughly
4. **Testing**: Test updates on sample projects
5. **Communication**: Announce changes clearly

### For Project Teams  
1. **Regular Updates**: Check monthly for updates
2. **Team Communication**: Notify all members of changes
3. **Backup Always**: Never skip backup step
4. **Verify Thoroughly**: Run all validation checks

## Update History Tracking

Maintain `.rotd/update_history.jsonl`:
```json
{"version":"1.2.0","updated_at":"2025-07-03T10:00:00Z","updated_by":"Claude","status":"success"}
{"version":"1.1.0","updated_at":"2025-06-15T09:00:00Z","updated_by":"GPT-4","status":"success"}
```

## Troubleshooting

### Common Issues

**Schema Validation Fails**
- Check for manual edits to artifacts
- Ensure all required fields present
- Validate JSON syntax

**Data Loss During Migration**
- Restore from backup immediately
- Check migration logic
- Report issue to ROTD maintainers

**Version Mismatch**
- Update CLI first
- Clear cache if needed
- Force version sync

## Summary

The ROTD Update Protocol ensures:
- **Continuity**: Projects keep working during updates
- **Safety**: Backups prevent data loss
- **Clarity**: Clear communication of changes
- **Verification**: Systematic validation of updates
- **Flexibility**: Rollback options available

Always prioritize project stability over adopting latest features.