#!/bin/bash

# Script to clean up failed GitHub Actions workflow runs
# Requires GitHub CLI (gh) to be installed and authenticated

set -e

REPO="jmfigueroa/rotd"
WORKFLOW_NAME="Release"

echo "🧹 Cleaning up failed workflow runs for $REPO"
echo "Workflow: $WORKFLOW_NAME"
echo

# Get workflow ID
WORKFLOW_ID=$(gh api repos/$REPO/actions/workflows --jq ".workflows[] | select(.name==\"$WORKFLOW_NAME\") | .id")

if [ -z "$WORKFLOW_ID" ]; then
    echo "❌ Could not find workflow '$WORKFLOW_NAME'"
    exit 1
fi

echo "Found workflow ID: $WORKFLOW_ID"
echo

# Get failed runs
echo "📋 Fetching failed workflow runs..."
FAILED_RUNS=$(gh api repos/$REPO/actions/workflows/$WORKFLOW_ID/runs --paginate --jq '.workflow_runs[] | select(.conclusion=="failure") | .id')

if [ -z "$FAILED_RUNS" ]; then
    echo "✅ No failed runs found!"
    exit 0
fi

# Count runs
RUN_COUNT=$(echo "$FAILED_RUNS" | wc -l | tr -d ' ')
echo "Found $RUN_COUNT failed runs to delete"
echo

# Confirm deletion
read -p "❓ Do you want to delete all $RUN_COUNT failed runs? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Deletion cancelled"
    exit 0
fi

echo
echo "🗑️  Deleting failed workflow runs..."

# Delete each run
COUNT=0
for RUN_ID in $FAILED_RUNS; do
    COUNT=$((COUNT + 1))
    echo "[$COUNT/$RUN_COUNT] Deleting run $RUN_ID..."
    
    if gh api repos/$REPO/actions/runs/$RUN_ID -X DELETE &>/dev/null; then
        echo "  ✅ Deleted"
    else
        echo "  ❌ Failed to delete (may require admin permissions)"
    fi
    
    # Rate limiting - pause briefly between deletions
    sleep 0.5
done

echo
echo "🎉 Cleanup complete!"
echo "Note: Some runs may require repository admin permissions to delete."