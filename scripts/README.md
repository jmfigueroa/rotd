# ROTD Update Scripts

This directory contains utility scripts for managing ROTD updates.

## Manual Update Script

`manual_update.sh` is a utility for updating older ROTD repositories to the latest version, especially useful when dealing with repositories that had the old hardcoded version checking.

### Features

- Creates a timestamped backup of all ROTD files
- Updates tasks.jsonl to add priority field
- Updates version tracking
- Records update history
- Creates an update manifest
- Sets up periodic review scheduling

### Usage

```bash
# From the root of your ROTD project
./scripts/manual_update.sh [OPTIONS]
```

#### Options

- `--dry-run`: Show what would be done without making changes
- `--force`: Skip confirmations and force updates

### Requirements

- `jq`: Required for JSON processing
- `git`: Required for repository operations

### Example

```bash
# Test what would happen without making changes
./scripts/manual_update.sh --dry-run

# Apply the update
./scripts/manual_update.sh
```

### Post-Update Verification

After running the script, you should verify the update:

```bash
# Validate all schemas
rotd validate --all --strict

# Check project health
rotd check --strict

# Fix any issues automatically
rotd check --fix

# Test GitHub integration
rotd update --check --verbose
```

### Troubleshooting

If you encounter issues with the update:

1. Restore from the backup created in `.rotd/backup_TIMESTAMP/`
2. Check the JSON files for any syntax errors
3. Run specific updates manually using the commands in the script

For assistance, please open an issue on the ROTD GitHub repository.