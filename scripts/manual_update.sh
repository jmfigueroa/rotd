#!/bin/bash
# ROTD Manual Update Script
# This script helps update older ROTD repositories to the latest version
# Usage: ./manual_update.sh [--dry-run] [--force]

set -e

# Configuration
DRY_RUN=false
FORCE=false
CURRENT_DATE=$(date -I)
CURRENT_TIMESTAMP=$(date -Iseconds)
LATEST_VERSION="1.2.0"
GITHUB_REPO_URL="https://github.com/jmfigueroa/rotd"

# Process arguments
for arg in "$@"; do
  case $arg in
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --force)
      FORCE=true
      shift
      ;;
  esac
done

# Text formatting
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Helper functions
function log_info() {
  echo -e "${CYAN}INFO:${NC} $1"
}

function log_success() {
  echo -e "${GREEN}✓ SUCCESS:${NC} $1"
}

function log_warning() {
  echo -e "${YELLOW}⚠ WARNING:${NC} $1"
}

function log_error() {
  echo -e "${RED}✗ ERROR:${NC} $1"
  exit 1
}

function check_command() {
  if ! command -v $1 &> /dev/null; then
    log_error "Required command '$1' not found. Please install it first."
  fi
}

function run_command() {
  if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}DRY RUN:${NC} Would run: $1"
  else
    echo -e "${CYAN}RUNNING:${NC} $1"
    eval "$1"
  fi
}

function get_current_version() {
  local version_path=".rotd/version.json"
  if [ -f "$version_path" ]; then
    jq -r '.version' "$version_path" 2>/dev/null || echo "1.0.0"
  else
    echo "1.0.0"
  fi
}

# Check for required tools
check_command jq
check_command git

# Check if we're in a directory with .rotd
if [ ! -d ".rotd" ]; then
  log_error "No .rotd directory found. Please run this script from the root of a ROTD project."
fi

log_info "Starting ROTD manual update process"
echo -e "${BOLD}This script will update your ROTD project to version $LATEST_VERSION${NC}"
echo

# Get current version
CURRENT_VERSION=$(get_current_version)
log_info "Current ROTD version: $CURRENT_VERSION"
log_info "Target ROTD version: $LATEST_VERSION"

# Backup existing ROTD files
log_info "Creating backup of ROTD files"
BACKUP_DIR=".rotd/backup_$(date +%Y%m%d_%H%M%S)"
run_command "mkdir -p $BACKUP_DIR"

# Copy all important files to backup
ROTD_FILES=(
  "tasks.jsonl"
  "session_state.json"
  "coverage_history.json"
  "pss_scores.jsonl"
  "lessons_learned.jsonl"
)

for file in "${ROTD_FILES[@]}"; do
  if [ -f ".rotd/$file" ]; then
    run_command "cp .rotd/$file $BACKUP_DIR/"
  fi
done

if [ -d ".rotd/test_summaries" ]; then
  run_command "cp -r .rotd/test_summaries $BACKUP_DIR/"
fi

log_success "Backup created at $BACKUP_DIR"

# Update schema files based on version
log_info "Updating ROTD schemas"

# 1. Update tasks.jsonl (add priority field)
if [ -f ".rotd/tasks.jsonl" ]; then
  if grep -q "\"priority\":" ".rotd/tasks.jsonl"; then
    log_info "Tasks already have priority field"
  else
    log_info "Adding priority field to tasks"
    if [ "$DRY_RUN" = false ]; then
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
      if jq empty ".rotd/tasks.jsonl.new" 2>/dev/null; then
        mv ".rotd/tasks.jsonl.new" ".rotd/tasks.jsonl"
        log_success "Priority field added to tasks"
      else
        rm ".rotd/tasks.jsonl.new"
        log_error "Failed to update tasks.jsonl. Please check the file format."
      fi
    fi
  fi
fi

# Create or update version.json
log_info "Updating version tracking"
if [ "$DRY_RUN" = false ]; then
  echo "{\"version\":\"$LATEST_VERSION\"}" > ".rotd/version.json"
fi
log_success "Version updated to $LATEST_VERSION"

# Create update history entry
log_info "Adding update history entry"
UPDATE_HISTORY_ENTRY="{\"version\":\"$LATEST_VERSION\",\"updated_at\":\"$CURRENT_TIMESTAMP\",\"updated_by\":\"Manual Update Script\",\"status\":\"success\",\"changes_applied\":[\"github_integration\",\"task_priority\"]}"

if [ "$DRY_RUN" = false ]; then
  echo "$UPDATE_HISTORY_ENTRY" >> ".rotd/update_history.jsonl"
fi
log_success "Update history recorded"

# Create update manifest
log_info "Creating update manifest"
UPDATE_MANIFEST="{
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

if [ "$DRY_RUN" = false ]; then
  echo "$UPDATE_MANIFEST" > ".rotd/update_manifest.json"
fi
log_success "Update manifest created"

# Create periodic review schedule if it doesn't exist
if [ ! -f ".rotd/review_schedule.json" ]; then
  log_info "Creating periodic review schedule"
  NEXT_MONDAY=$(date -d "next Monday" +%Y-%m-%d)
  REVIEW_SCHEDULE="{
    \"frequency\": \"weekly\",
    \"next_review\": \"$NEXT_MONDAY\",
    \"reviewers\": [\"team\"],
    \"created_at\": \"$CURRENT_TIMESTAMP\"
  }"
  
  if [ "$DRY_RUN" = false ]; then
    echo "$REVIEW_SCHEDULE" > ".rotd/review_schedule.json"
  fi
  log_success "Review schedule created"
fi

# Verify the update
log_info "Verifying the update"
echo -e "${YELLOW}NOTE:${NC} For a complete verification, please run these commands manually:"
echo "  rotd validate --all --strict"
echo "  rotd check --strict"
echo "  rotd check --fix"

# Print final instructions
echo
log_success "ROTD Update Completed"
echo -e "${BOLD}Next Steps:${NC}"
echo "1. Run 'rotd validate --all --strict' to verify all schemas"
echo "2. Run 'rotd check --strict' to verify project health"
echo "3. Fix any issues with 'rotd check --fix'"
echo "4. Test GitHub integration with 'rotd update --check --verbose'"
echo "5. Review the updated documentation at $GITHUB_REPO_URL"
echo
echo -e "Your backup is available at: ${CYAN}$BACKUP_DIR${NC}"
echo -e "If you encounter any issues, please restore from backup or report to the ROTD team."