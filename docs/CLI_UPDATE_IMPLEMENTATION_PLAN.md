# CLI Update Implementation Plan

> Development plan for implementing ROTD update protocol in the CLI

## Status: COMPLETED (v1.3.0)

This plan has been successfully implemented with the addition of multi-agent coordination features in ROTD v1.3.0. The original update protocol features have been enhanced with coordination capabilities.

## Overview

This plan outlined the implementation of update protocol features in the ROTD CLI, enabling automated version tracking, manifest generation, and user guidance for applying methodology updates. Additional multi-agent coordination features were added in v1.3.0.

## Current State Analysis

### Existing CLI Structure
```
src/
├── main.rs           # CLI entry point and argument parsing
├── agent.rs          # Agent mode commands (JSON I/O)
├── human.rs          # Human mode commands (interactive)
├── schema.rs         # Data structures and validation
├── fs_ops.rs         # File operations and safety
├── pss.rs            # Progress Scoring System
├── audit.rs          # Audit logging
└── common.rs         # Shared utilities
```

### Originally Missing Features (Now Implemented)
1. ✅ Version tracking and comparison
2. ✅ Update manifest generation
3. ✅ Enhanced validation commands
4. ✅ Post-update user guidance
5. ✅ Update history logging

### Additional Features Added (v1.3.0)
6. ✅ Multi-agent coordination commands
7. ✅ Artifact-level file locking
8. ✅ Agent heartbeat mechanism
9. ✅ Priority-aware task claiming
10. ✅ Dependency validation

## Implementation Plan

### Phase 1: Version Management
**Files to modify:** `src/main.rs`, `src/common.rs`, `src/schema.rs`

#### 1.1 Version Commands
```rust
// Add to main.rs command structure
pub enum Commands {
    // ... existing commands
    Version {
        #[arg(long)]
        project: bool,
        #[arg(long)]
        latest: bool,
    },
}
```

#### 1.2 Version Tracking Structure
```rust
// Add to schema.rs
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectVersion {
    pub version: String,
    pub updated_at: String,
    pub manifest_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifest {
    pub version: String,
    pub date: String,
    pub changes: Vec<ChangeEntry>,
    pub previous_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeEntry {
    pub change_type: String,
    pub component: String,
    pub description: String,
    pub breaking: bool,
    pub migration_required: bool,
}
```

#### 1.3 Implementation Details
- Store current version in `.rotd/version.json`
- Compare against embedded CLI version
- Fetch latest version from repository (if network available)

### Phase 2: Enhanced Update Command
**Files to modify:** `src/human.rs`, `src/fs_ops.rs`

#### 2.1 Update Command Enhancement
```rust
// Modify existing update command in human.rs
pub fn update_command(check_only: bool, force: bool) -> Result<()> {
    if check_only {
        return check_for_updates();
    }
    
    // 1. Backup current state
    backup_rotd_files()?;
    
    // 2. Download updates
    let manifest = download_updates(force)?;
    
    // 3. Generate update manifest
    write_update_manifest(&manifest)?;
    
    // 4. Show user guidance
    display_update_guidance(&manifest)?;
    
    Ok(())
}
```

#### 2.2 Update Manifest Generation
```rust
// Add to fs_ops.rs
pub fn write_update_manifest(manifest: &UpdateManifest) -> Result<()> {
    let manifest_path = rotd_dir()?.join("update_manifest.json");
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(manifest_path, json)?;
    Ok(())
}
```

### Phase 3: Enhanced Validation
**Files to modify:** `src/human.rs`, `src/schema.rs`

#### 3.1 Validation Commands
```rust
// Add to main.rs
Validate {
    #[arg(long)]
    all: bool,
    #[arg(long)]
    schema: Option<String>,
    #[arg(long)]
    strict: bool,
},
```

#### 3.2 Schema Validation
```rust
// Add to schema.rs
pub fn validate_all_schemas() -> Result<ValidationReport> {
    let mut report = ValidationReport::new();
    
    // Validate tasks.jsonl
    report.add_result("tasks", validate_tasks_schema()?);
    
    // Validate test summaries
    report.add_result("summaries", validate_summaries_schema()?);
    
    // Validate other artifacts
    report.add_result("scores", validate_scores_schema()?);
    report.add_result("lessons", validate_lessons_schema()?);
    
    Ok(report)
}
```

### Phase 4: Update History Tracking
**Files to modify:** `src/fs_ops.rs`, `src/schema.rs`

#### 4.1 Update History Structure
```rust
// Add to schema.rs
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHistoryEntry {
    pub version: String,
    pub updated_at: String,
    pub updated_by: String,
    pub status: String,
    pub changes_applied: Vec<String>,
    pub migration_notes: Option<String>,
}
```

#### 4.2 History Logging
```rust
// Add to fs_ops.rs
pub fn log_update_completion(entry: &UpdateHistoryEntry) -> Result<()> {
    let history_path = rotd_dir()?.join("update_history.jsonl");
    let line = serde_json::to_string(entry)?;
    append_line(&history_path, &line)?;
    Ok(())
}
```

### Phase 5: Post-Update Guidance
**Files to modify:** `src/human.rs`

#### 5.1 User Guidance Display
```rust
// Add to human.rs
pub fn display_update_guidance(manifest: &UpdateManifest) -> Result<()> {
    println!("🔄 ROTD Update Complete!");
    println!("Version: {} → {}", manifest.previous_version, manifest.version);
    println!();
    
    // Show changes summary
    show_changes_summary(&manifest.changes);
    
    // Show required actions
    show_required_actions(&manifest.changes);
    
    // Display update prompt
    show_update_prompt(&manifest.changes);
    
    Ok(())
}

fn show_update_prompt(changes: &[ChangeEntry]) -> Result<()> {
    if changes.iter().any(|c| c.migration_required) {
        println!("📋 Next Steps:");
        println!("Use this prompt with your LLM to apply updates to your project:");
        println!();
        println!("```");
        display_update_prompt_text(changes);
        println!("```");
    }
    Ok(())
}
```

#### 5.2 Dynamic Prompt Generation
```rust
fn display_update_prompt_text(changes: &[ChangeEntry]) {
    println!("ROTD methodology has been updated. Apply the latest changes to this project.");
    println!();
    println!("📋 **Update Process**:");
    println!();
    println!("1. **Review Changes**");
    println!("   - Check `.rotd/update_manifest.json` for list of updates");
    
    for change in changes.iter().filter(|c| c.migration_required) {
        match change.component.as_str() {
            "task_schema" => print_task_schema_prompt(),
            "workflow" => print_workflow_prompt(&change.description),
            _ => print_generic_prompt(change),
        }
    }
    
    println!();
    println!("3. **Verify Updates**");
    println!("   ```bash");
    println!("   rotd check --strict");
    println!("   rotd validate --all");
    println!("   ```");
}
```

## Implementation Timeline (Completed)

### Week 1: Foundation ✅
- [x] Implemented version tracking structures
- [x] Added version commands (`rotd version --project`, `rotd version --latest`)
- [x] Created update manifest generation
- [x] Tested basic version functionality

### Week 2: Core Updates ✅
- [x] Enhanced update command with new features
- [x] Implemented backup functionality
- [x] Added update history logging
- [x] Tested update download and manifest creation

### Week 3: Validation & Guidance ✅
- [x] Implemented enhanced validation commands
- [x] Added post-update guidance display
- [x] Created dynamic prompt generation
- [x] Tested complete update workflow

### Week 4: Testing & Polish ✅
- [x] Integration testing
- [x] Error handling improvements
- [x] Documentation updates
- [x] User acceptance testing

### Additional Implementation (v1.3.0) ✅
- [x] Multi-agent coordination module (coord.rs)
- [x] File locking mechanism (with_lock_result)
- [x] Agent heartbeat system
- [x] Work registry management
- [x] Dependency validation logic

## Technical Requirements

### Dependencies
```toml
# Add to Cargo.toml
[dependencies]
# ... existing deps
sha2 = "0.10"          # For manifest hashing
reqwest = "0.11"       # For fetching updates (optional)
tokio = "1.0"          # For async operations (if needed)
```

### File Structure Changes
```
.rotd/
├── version.json              # Current project version
├── update_manifest.json      # Latest update details
├── update_history.jsonl      # Update application log
└── backup/                   # Backup directory
    ├── tasks.jsonl
    ├── *.json
    └── timestamp.txt
```

### Error Handling
- Network failures (graceful degradation)
- File permission issues
- Corrupted backup files
- Version mismatch scenarios
- Migration failures

### Security Considerations
- Validate downloaded content
- Secure backup file handling
- User confirmation for destructive operations
- Audit trail for all updates

## Testing Strategy

### Unit Tests
- Version comparison logic
- Manifest generation
- Schema validation
- Backup/restore operations

### Integration Tests
- Complete update workflow
- CLI command integration
- File system operations
- Error scenarios

### User Acceptance Tests
- Update from v1.1.0 to v1.2.0
- Schema migration accuracy
- User guidance clarity
- Rollback functionality

## Rollout Plan

### Phase 1: Internal Testing
- Test on sample ROTD projects
- Validate migration accuracy
- Confirm user guidance quality

### Phase 2: Beta Release
- Release to trusted users
- Gather feedback
- Refine prompts and guidance

### Phase 3: Production Release
- Public release
- Documentation updates
- Community support

## Success Criteria (Achieved)

### Functional Requirements ✅
- [x] All new CLI commands work correctly
- [x] Update manifests generate accurately
- [x] Schema migrations preserve data
- [x] User guidance is clear and actionable
- [x] Validation catches all issues
- [x] Multi-agent coordination fully functional

### User Experience ✅
- [x] Update process is intuitive
- [x] Error messages are helpful
- [x] Prompts are copy-pastable
- [x] Recovery options are available
- [x] Agent workflows documented

### Technical Quality ✅
- [x] Code compiles without errors
- [x] No regressions in existing features
- [x] Performance acceptable
- [x] Memory usage reasonable
- [x] Lock mechanisms prevent race conditions

## Risk Mitigation

### Data Loss Prevention
- Automatic backups before updates
- Validation before overwriting files
- Rollback capability
- User confirmation prompts

### Version Conflicts
- Clear version comparison logic
- Graceful handling of unknowns
- Manual override options
- Detailed error reporting

### Network Dependencies
- Offline mode support
- Cached update information
- Manual update options
- Clear network error messages

## Implementation Summary

This implementation plan was successfully completed with the release of ROTD v1.3.0. The original update protocol features were implemented as designed, and additional multi-agent coordination capabilities were added to support parallel development workflows.

### Key Achievements
1. **Update Protocol**: Full implementation of version tracking, manifest generation, and guided updates
2. **Multi-Agent Support**: Complete coordination system with task claiming, heartbeats, and dependency management
3. **File Safety**: Artifact-level locking prevents concurrent write conflicts
4. **User Experience**: Clear CLI commands and helpful error messages
5. **Documentation**: Comprehensive docs and prompts for all features

### Lessons Learned
- Rust's type system helped catch many potential issues at compile time
- The `fs2` crate provided reliable cross-platform file locking
- Clear separation of agent and human modes improved UX
- Coordination directory structure kept multi-agent state organized

### Future Enhancements
- Path-scoped source file locking
- Advanced quota management
- Task graph visualization
- Distributed coordination support

This plan served as an effective roadmap for implementing robust update protocol support and multi-agent coordination in the ROTD CLI.