#!/usr/bin/env fish
# ROTD Manual Update Script (Fish Shell Version)
# This script helps update older ROTD repositories to the latest version
# Usage: ./manual_update.fish [--dry-run] [--force]

# Configuration
set -l DRY_RUN false
set -l FORCE false
set -l LATEST_VERSION "1.3.0"
set -l GITHUB_REPO_URL "https://github.com/jmfigueroa/rotd"

# Get current date and timestamp in a cross-platform way
set -l CURRENT_DATE (date "+%Y-%m-%d")
set -l CURRENT_TIMESTAMP (date "+%Y-%m-%dT%H:%M:%S%z")

# Process arguments
for arg in $argv
  switch $arg
    case --dry-run
      set DRY_RUN true
    case --force
      set FORCE true
  end
end

# Text formatting
set -l GREEN '\033[0;32m'
set -l YELLOW '\033[1;33m'
set -l CYAN '\033[0;36m'
set -l RED '\033[0;31m'
set -l NC '\033[0m' # No Color
set -l BOLD '\033[1m'

# Helper functions
function log_info
  echo -e "$CYAN""INFO:""$NC $argv"
end

function log_success
  echo -e "$GREEN""✓ SUCCESS:""$NC $argv"
end

function log_warning
  echo -e "$YELLOW""⚠ WARNING:""$NC $argv"
end

function log_error
  echo -e "$RED""✗ ERROR:""$NC $argv"
  exit 1
end

function check_command
  if not command -sq $argv[1]
    log_error "Required command '$argv[1]' not found. Please install it first."
  end
end

function run_command
  if test "$DRY_RUN" = true
    echo -e "$YELLOW""DRY RUN:""$NC Would run: $argv"
  else
    echo -e "$CYAN""RUNNING:""$NC $argv"
    eval $argv
  end
end

function get_current_version
  set -l version_path ".rotd/version.json"
  if test -f "$version_path"
    jq -r '.version' "$version_path" 2>/dev/null; or echo "1.0.0"
  else
    echo "1.0.0"
  end
end

# Check for required tools
check_command jq
check_command git

# Check if we're in a directory with .rotd
if not test -d ".rotd"
  log_error "No .rotd directory found. Please run this script from the root of a ROTD project."
end

log_info "Starting ROTD manual update process"
echo -e "$BOLD""This script will update your ROTD project to version $LATEST_VERSION""$NC"
echo

# Get current version
set -l CURRENT_VERSION (get_current_version)
log_info "Current ROTD version: $CURRENT_VERSION"
log_info "Target ROTD version: $LATEST_VERSION"

# Backup existing ROTD files
log_info "Creating backup of ROTD files"
set -l BACKUP_DIR ".rotd/backup_"(date "+%Y%m%d_%H%M%S")
run_command "mkdir -p $BACKUP_DIR"

# Copy all important files to backup
set -l ROTD_FILES "tasks.jsonl" "session_state.json" "coverage_history.json" "pss_scores.jsonl" "lessons_learned.jsonl"

for file in $ROTD_FILES
  if test -f ".rotd/$file"
    run_command "cp .rotd/$file $BACKUP_DIR/"
  end
end

if test -d ".rotd/test_summaries"
  run_command "cp -r .rotd/test_summaries $BACKUP_DIR/"
end

log_success "Backup created at $BACKUP_DIR"

# Update schema files based on version
log_info "Updating ROTD schemas"

# 1. Update tasks.jsonl (add priority field and fix JSON errors)
if test -f ".rotd/tasks.jsonl"
  # Check if tasks.jsonl is valid JSON
  set -l is_valid_json (jq empty ".rotd/tasks.jsonl" 2>/dev/null; echo $status)
  
  if test $is_valid_json -ne 0
    log_info "Found invalid JSON in tasks.jsonl, attempting to fix"
    if test "$DRY_RUN" = false
      # Create a backup first
      cp ".rotd/tasks.jsonl" ".rotd/tasks.jsonl.bak"
      
      # Read the file line by line and try to fix each line
      set -l temp_file ".rotd/tasks.jsonl.fixed"
      rm -f $temp_file
      touch $temp_file
      
      set -l fixed_count 0
      set -l error_count 0
      
      while read -l line
        if test -z "$line"
          continue
        end
        
        # Try to parse and fix JSON
        set -l fixed_line ""
        set -l valid_json (echo $line | jq empty 2>/dev/null; echo $status)
        
        if test $valid_json -eq 0
          # Line is already valid JSON, just normalize it
          set fixed_line (echo $line | jq -c '.')
        else
          # Try to fix common JSON errors
          # 1. Missing quotes around keys
          set fixed_line (echo $line | sed -E 's/\{([^:]*):/{"\1":/g')
          # 2. Missing quotes around values
          set fixed_line (echo $fixed_line | sed -E 's/:([^"{}\[\],]*)(,|\})/:\"\1\"\2/g')
          # 3. Fix trailing commas
          set fixed_line (echo $fixed_line | sed -E 's/,\s*\}/}/g')
          
          # Check if our fix worked
          set valid_json (echo $fixed_line | jq empty 2>/dev/null; echo $status)
          if test $valid_json -eq 0
            set fixed_count (math $fixed_count + 1)
          else
            # If we couldn't fix it, keep the original
            set fixed_line $line
            set error_count (math $error_count + 1)
          end
        end
        
        # Write the line to the temp file
        echo $fixed_line >> $temp_file
      end < ".rotd/tasks.jsonl"
      
      # Replace the original file if we fixed anything
      if test $fixed_count -gt 0
        mv $temp_file ".rotd/tasks.jsonl"
        log_success "Fixed $fixed_count JSON errors in tasks.jsonl"
        if test $error_count -gt 0
          log_warning "Could not fix $error_count lines in tasks.jsonl"
        end
      else
        rm $temp_file
        if test $error_count -gt 0
          log_warning "Could not fix any JSON errors in tasks.jsonl ($error_count lines with errors)"
        end
      end
    end
  end
  
  # Now check and add priority field if needed
  if grep -q "\"priority\":" ".rotd/tasks.jsonl"
    log_info "Tasks already have priority field"
  else
    log_info "Adding priority field to tasks"
    if test "$DRY_RUN" = false
      # Create a temporary file
      cat ".rotd/tasks.jsonl" | jq 'if .priority == null then 
        . + {
          "priority": (
            if .status == "in_progress" then "high"
            elif .status == "blocked" then "urgent"
            elif .status == "pending" then "medium"
            elif .status == "complete" then "low"
            else "medium"
            end
          )
        } 
        else . end' > ".rotd/tasks.jsonl.new"
      
      # Check if conversion was successful
      if jq empty ".rotd/tasks.jsonl.new" 2>/dev/null
        mv ".rotd/tasks.jsonl.new" ".rotd/tasks.jsonl"
        log_success "Priority field added to tasks"
      else
        rm ".rotd/tasks.jsonl.new"
        log_error "Failed to update tasks.jsonl. Please check the file format."
      end
    end
  end
end

# Create or update version.json
log_info "Updating version tracking"
if test "$DRY_RUN" = false
  echo "{\"version\":\"$LATEST_VERSION\"}" > ".rotd/version.json"
end
log_success "Version updated to $LATEST_VERSION"

# Create update history entry
log_info "Adding update history entry"
set -l UPDATE_HISTORY_ENTRY "{\"version\":\"$LATEST_VERSION\",\"updated_at\":\"$CURRENT_TIMESTAMP\",\"updated_by\":\"Manual Update Script\",\"status\":\"success\",\"changes_applied\":[\"github_integration\",\"task_priority\"]}"

if test "$DRY_RUN" = false
  echo $UPDATE_HISTORY_ENTRY >> ".rotd/update_history.jsonl"
end
log_success "Update history recorded"

# Create update manifest
log_info "Creating update manifest"
set -l UPDATE_MANIFEST "{
  \"version\": \"$LATEST_VERSION\",
  \"date\": \"$CURRENT_DATE\",
  \"previous_version\": \"$CURRENT_VERSION\",
  \"changes\": [
    {
      \"change_type\": \"feature\",
      \"component\": \"update_system\",
      \"description\": \"Added GitHub API integration for version checking\",
      \"breaking\": false,
      \"migration_required\": false
    },
    {
      \"change_type\": \"feature\",
      \"component\": \"task_schema\",
      \"description\": \"Added priority field to tasks\",
      \"breaking\": false,
      \"migration_required\": true
    }
  ]
}"

if test "$DRY_RUN" = false
  echo $UPDATE_MANIFEST > ".rotd/update_manifest.json"
end
log_success "Update manifest created"

# Create periodic review schedule if it doesn't exist
if not test -f ".rotd/review_schedule.json"
  log_info "Creating periodic review schedule"
  
  # Get next Monday in a cross-platform way
  set -l NEXT_MONDAY ""
  
  # For macOS
  if test (uname) = "Darwin"
    set NEXT_MONDAY (date -v+mon "+%Y-%m-%d")
  else
    # For Linux/others
    set NEXT_MONDAY (date -d "next Monday" "+%Y-%m-%d" 2>/dev/null; or date -v+mon "+%Y-%m-%d" 2>/dev/null; or date "+%Y-%m-%d")
  end
  
  set -l REVIEW_SCHEDULE "{
    \"frequency\": \"weekly\",
    \"next_review\": \"$NEXT_MONDAY\",
    \"reviewers\": [\"team\"],
    \"created_at\": \"$CURRENT_TIMESTAMP\"
  }"
  
  if test "$DRY_RUN" = false
    echo $REVIEW_SCHEDULE > ".rotd/review_schedule.json"
  end
  log_success "Review schedule created"
end

# Verify the update
log_info "Verifying the update"
echo -e "$YELLOW""NOTE:""$NC For a complete verification, please run these commands manually:"
echo "  rotd validate --all --strict"
echo "  rotd check --strict"
echo "  rotd check --fix"

# Print final instructions
echo
log_success "ROTD Update Completed"
echo -e "$BOLD""Next Steps:""$NC"
echo "1. Run 'rotd validate --all --strict' to verify all schemas"
echo "2. Run 'rotd check --strict' to verify project health"
echo "3. Fix any issues with 'rotd check --fix'"
echo "4. Test GitHub integration with 'rotd update --check --verbose'"
echo "5. Review the updated documentation at $GITHUB_REPO_URL"
echo
echo -e "Your backup is available at: ""$CYAN""$BACKUP_DIR""$NC"
echo -e "If you encounter any issues, please restore from backup or report to the ROTD team."