# ROTD CLI Update Guide

## Overview of Changes

This document details the changes needed to update the ROTD CLI to version 1.2.1, including replacing stub implementations with real GitHub API integration for version checking.

## Current Issues

Based on testing, we've identified the following issues:

1. **Inconsistent Version References**: 
   - Cargo.toml shows version 0.1.0
   - Manual update scripts refer to version 1.2.1
   - Hardcoded fallbacks in code show 1.1.0
   - Some test examples reference version 1.2.0

2. **GitHub API Integration**:
   - Real API calls are implemented but fallback values are incorrect
   - Update checking via `rotd update --check` displays incorrect version info

3. **Code Warnings**:
   - Unused imports in various files
   - Unused variables and functions

4. **JSON Auto-fix Implementation**:
   - Functionality is implemented but had build errors

## Required Changes

The following changes need to be made:

### 1. Update Version References

Update all version references throughout the codebase to consistently show 1.2.1:

- **Cargo.toml**:
  ```toml
  [package]
  name = "rotd"
  version = "1.2.1"  # Change from 0.1.0
  ```

- **src/human.rs**:
  - Line ~186: Change `"1.1.0".to_string()` to `"1.2.1".to_string()`
  - Line ~220: Change `"1.1.0".to_string()` to `"1.2.1".to_string()`

- **src/agent.rs**:
  - Line ~582: Change `"1.1.0".to_string()` to `"1.2.1".to_string()`
  - Line ~613: Change `"1.1.0".to_string()` to `"1.2.1".to_string()`
  - Update version info in the info method output

- **src/config.rs**:
  - Update default version in RotdConfig implementation

- **src/github.rs**:
  - Update test examples from "Release v1.2.0" to "Release v1.2.1"

- **Verify scripts**:
  - Ensure src/scripts/manual_update.fish has LATEST_VERSION="1.2.1"
  - Ensure src/scripts/manual_update.sh has LATEST_VERSION="1.2.1"

### 2. Remove Code Warnings

Remove all warnings to ensure clean compilation:

- **src/agent.rs**:
  - Remove unused import: `use semver::Version;`

- **src/cli/commands/buckle_mode.rs**:
  - Remove unused imports: 
    - `use anyhow::Result;`
    - `use colored::Colorize;`

- **src/human.rs**:
  - Fix unused variable: Change `let missing_count = missing.len();` to `let _missing_count = missing.len();`
  - Add `#[allow(dead_code)]` attribute above the `show_help` function:
    ```rust
    #[allow(dead_code)]
    pub fn show_help(verbose: bool) -> Result<()> {
    ```

- **src/audit.rs**:
  - Add `#[allow(dead_code)]` attribute above the `read_audit_log` function:
    ```rust
    #[allow(dead_code)]
    pub fn read_audit_log(limit: usize) -> Result<Vec<String>> {
    ```

### 3. Fix GitHub API Integration

The GitHub API integration is correctly implemented but needs some modifications:

- Ensure error handling in fetch_latest_release() is robust
- Update fallback values to match the current version
- Test the integration with proper error reporting

### 4. JSON Auto-fix Implementation

The JSON auto-fix functionality is implemented with these components:

- Agent-mode implementation in `agent.rs`
- Human-friendly implementation in `human.rs`
- Supporting functions in manual update scripts

Make sure the regex patterns use raw string literals (r#"..."#) to avoid escaping issues and implement proper error handling with `if let Ok(re) = regex::Regex::new(...)` pattern.

## Build and Test Instructions

After making these changes:

1. **Build the CLI**:
   ```bash
   cargo build --release
   ```

2. **Install the updated CLI**:
   ```bash
   cp target/release/rotd /path/to/installation/rotd
   ```

3. **Test version reporting**:
   ```bash
   rotd --version         # Should show 1.2.1
   rotd version           # Should show more detailed version info
   ```

4. **Test update checking**:
   ```bash
   rotd update --check    # Should show current version as 1.2.1
   ```

5. **Test JSON auto-fix**:
   ```bash
   rotd check --fix       # Should fix any JSON issues
   ```

## GitHub Repository Changes

These changes need to be committed to the repository with appropriate messages:

1. "Update all version references to 1.2.1"
2. "Remove warnings from code"
3. "Fix GitHub API integration for version checking"

After pushing these changes, ensure to rebuild and reinstall the CLI to see the effects.

## Common Issues and Troubleshooting

1. **Still seeing old version numbers**:
   - Verify the installed binary is the newly built one
   - Check PATH is prioritizing the correct installation
   - Use `which rotd` to confirm the binary location

2. **GitHub API calls failing**:
   - Check network connectivity and firewalls
   - Ensure GitHub API URLs are correct
   - The API might be rate-limited

3. **JSON auto-fix not working**:
   - Check if files exist and are accessible
   - Verify regex patterns are correct and properly escaped
   - Review the error handling logic