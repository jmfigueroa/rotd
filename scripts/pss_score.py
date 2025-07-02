#!/usr/bin/env python3
"""
ROTD Progress Scoring System (PSS) - Portable Version

Generates PSS scores for tasks based on project artifacts and provides
a framework for consistent evaluation across ROTD projects.
"""

import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, Optional

class PSSScorer:
    def __init__(self, rotd_dir: str = ".rotd"):
        self.rotd_dir = Path(rotd_dir)
        self.scores_file = self.rotd_dir / "pss_scores.jsonl"
        
    def load_test_summary(self, task_id: str) -> Optional[Dict[str, Any]]:
        """Load test summary for a task"""
        summary_file = self.rotd_dir / "test_summaries" / f"{task_id}.json"
        if summary_file.exists():
            with open(summary_file) as f:
                return json.load(f)
        return None
    
    def load_tasks(self) -> list:
        """Load tasks.jsonl"""
        tasks_file = self.rotd_dir / "tasks.jsonl"
        tasks = []
        if tasks_file.exists():
            with open(tasks_file) as f:
                for line in f:
                    if line.strip():
                        tasks.append(json.loads(line))
        return tasks
    
    def load_coverage_history(self) -> Optional[Dict[str, Any]]:
        """Load coverage history"""
        coverage_file = self.rotd_dir / "coverage_history.json"
        if coverage_file.exists():
            with open(coverage_file) as f:
                return json.load(f)
        return None
    
    def check_compiles(self) -> bool:
        """Check if project compiles - extensible for different languages"""
        # Check for package.json (Node.js/TypeScript)
        if Path("package.json").exists():
            return os.system("npm run typecheck > /dev/null 2>&1") == 0
        
        # Check for Cargo.toml (Rust)
        if Path("Cargo.toml").exists():
            return os.system("cargo check > /dev/null 2>&1") == 0
        
        # Default assume compilation passes
        return True
    
    def check_stubs_remaining(self) -> bool:
        """Check for remaining stubs in codebase"""
        # Search for common stub patterns
        stub_patterns = [
            "#[rotd_stub]",
            "TODO(",
            "unimplemented!",
            "todo!",
            "throw new Error(\"TODO\")"
        ]
        
        for pattern in stub_patterns:
            if os.system(f"grep -r '{pattern}' src/ > /dev/null 2>&1") == 0:
                return True
        return False
    
    def generate_criteria_scores(self, task_id: str) -> Dict[str, Dict[str, Any]]:
        """Generate criteria scores for a task"""
        criteria = {}
        
        # Load relevant data
        test_summary = self.load_test_summary(task_id)
        tasks = self.load_tasks()
        coverage_history = self.load_coverage_history()
        
        # Find the task
        task = next((t for t in tasks if t.get("id") == task_id), None)
        
        # 1. LLM Engagement
        criteria["llm_engaged"] = {
            "score": 1 if task and task.get("status") in ["in_progress", "complete"] else 0,
            "rationale": f"Task {task_id} found with status: {task.get('status') if task else 'not found'}"
        }
        
        # 2. Compiles
        compiles = self.check_compiles()
        criteria["compiles"] = {
            "score": 1 if compiles else 0,
            "rationale": "Project compiles cleanly" if compiles else "Compilation errors detected"
        }
        
        # 3. Core Implementation Complete
        criteria["core_impl"] = {
            "score": 1 if task and task.get("status") == "complete" else 0,
            "rationale": f"Task status is {task.get('status') if task else 'unknown'}"
        }
        
        # 4. Tests Written
        tests_written = test_summary is not None and test_summary.get("total", 0) > 0
        criteria["tests_written"] = {
            "score": 1 if tests_written else 0,
            "rationale": f"Test summary shows {test_summary.get('total', 0) if test_summary else 0} tests"
        }
        
        # 5. Tests Pass (Threshold Met)
        if test_summary:
            pass_rate = test_summary.get("passed", 0) / max(test_summary.get("total", 1), 1)
            tests_pass = pass_rate >= 0.7  # 70% threshold
            criteria["tests_pass"] = {
                "score": 1 if tests_pass else 0,
                "rationale": f"Pass rate: {pass_rate*100:.1f}% ({'meets' if tests_pass else 'below'} 70% threshold)"
            }
        else:
            criteria["tests_pass"] = {
                "score": 0,
                "rationale": "No test summary available"
            }
        
        # 6. Documentation Maintenance
        criteria["doc_maintained"] = {
            "score": 1,  # Placeholder - would check for lint/format passes
            "rationale": "Documentation maintained (assuming lint/format passes)"
        }
        
        # 7. Stub-Free
        stubs_remaining = self.check_stubs_remaining()
        criteria["stub_free"] = {
            "score": 0 if stubs_remaining else 1,
            "rationale": "Stubs remaining" if stubs_remaining else "No stubs detected"
        }
        
        # 8. History Maintained
        has_test_summary = test_summary is not None
        task_in_jsonl = task is not None
        criteria["history_maintained"] = {
            "score": 1 if has_test_summary and task_in_jsonl else 0,
            "rationale": f"Test summary: {'✓' if has_test_summary else '✗'}, Task in jsonl: {'✓' if task_in_jsonl else '✗'}"
        }
        
        # 9. QTS Floor Met
        if coverage_history and test_summary:
            current_coverage = test_summary.get("coverage", 0) * 100
            floor = coverage_history.get("floor", 70)
            floor_met = current_coverage >= floor
            criteria["qts_floor"] = {
                "score": 1 if floor_met else 0,
                "rationale": f"Coverage {current_coverage:.1f}% vs floor {floor}%"
            }
        else:
            criteria["qts_floor"] = {
                "score": 0,
                "rationale": "Coverage data not available"
            }
        
        # 10. QTS Ratchet
        if coverage_history and test_summary:
            current_coverage = test_summary.get("coverage", 0) * 100
            floor = coverage_history.get("floor", 70)
            ratchet_threshold = coverage_history.get("ratchet_threshold", 3)
            headroom = current_coverage - floor
            ratchet_triggered = headroom > ratchet_threshold
            criteria["qts_ratchet"] = {
                "score": 1 if ratchet_triggered else 0,
                "rationale": f"Headroom {headroom:.1f}% {'triggers' if ratchet_triggered else 'below'} {ratchet_threshold}% threshold"
            }
        else:
            criteria["qts_ratchet"] = {
                "score": 0,
                "rationale": "Coverage data not available for ratchet calculation"
            }
        
        return criteria
    
    def score_task(self, task_id: str) -> Dict[str, Any]:
        """Generate complete PSS score for a task"""
        criteria = self.generate_criteria_scores(task_id)
        
        # Calculate total score
        total_score = sum(c["score"] for c in criteria.values())
        
        score_entry = {
            "task_id": task_id,
            "score": total_score,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "criteria": criteria
        }
        
        return score_entry
    
    def save_score(self, score_entry: Dict[str, Any]) -> None:
        """Save score to pss_scores.jsonl"""
        self.rotd_dir.mkdir(exist_ok=True)
        
        with open(self.scores_file, "a") as f:
            f.write(json.dumps(score_entry) + "\n")
        
        task_id = score_entry["task_id"]
        score = score_entry["score"]
        print(f"ROTD scoring complete for task {task_id} (Score: {score}/10)")
        
        if score < 6:
            print("⚠️  Score below 6/10 - consider remediation:")
            for criterion, data in score_entry["criteria"].items():
                if data["score"] == 0:
                    print(f"  - {criterion}: {data['rationale']}")

def main():
    if len(sys.argv) != 2:
        print("Usage: python pss_score.py <task_id>")
        print("\nExample: python pss_score.py 6.1")
        sys.exit(1)
    
    task_id = sys.argv[1]
    scorer = PSSScorer()
    
    score_entry = scorer.score_task(task_id)
    scorer.save_score(score_entry)

if __name__ == "__main__":
    main()