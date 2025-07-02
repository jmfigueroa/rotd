# Runtime-Oriented Test Discipline (ROTD)

> Test-driven development optimized for LLM workflows, where runtime validation is the single source of truth.

## Quick Start

1. **Initialize** your project with a `.rotd/` directory
2. **Track tasks** in `.rotd/tasks.jsonl` 
3. **Write tests** before marking tasks complete
4. **Score progress** with the PSS system
5. **Learn from failures** in `.rotd/lessons_learned.jsonl`

## Philosophy

Traditional TDD: "Write the failing test first."  
**ROTD**: "Explore and implement, but nothing is done until tests prove it works."

ROTD is designed for LLM-assisted development where:
- Exploration and rapid iteration are encouraged
- Runtime validation is the ultimate truth
- Progress is measured systematically
- Failures become shared memory

## Core Files

- **[ROTD.md](./ROTD.md)** - Complete methodology documentation
- **[examples/](./examples/)** - Annotated JSON examples from real projects
- **[schema/](./schema/)** - JSON schemas for validation
- **[pss_score.py](./pss_score.py)** - Portable PSS scoring script

## Key Principles

1. **Test-Driven**: Every feature needs tests
2. **Runtime Truth**: 100% tests must pass before marking complete
3. **Clean Code**: No TODOs/stubs on main branch
4. **Systematic Progress**: Track everything in structured artifacts
5. **Continuous Learning**: Log failures for future reference

## Artifacts Structure

```
.rotd/                       # ROTD directory (hidden)
├── tasks.jsonl              # Append-only task log
├── test_summaries/          # Proof of completion
├── lessons_learned.jsonl    # Reusable failure patterns
├── pss_scores.jsonl         # Progress scoring results
├── session_state.json       # Delta prompting state
├── coverage_history.json    # Adaptive coverage tracking
└── audit.log                # Rule violations
```

## Progress Scoring System (PSS)

ROTD includes a 10-point scoring system evaluating:

1. **Execution Sanity (1-3)**: LLM engagement, compilation, implementation
2. **Testing Discipline (4-6)**: Tests written/passing, coverage quality  
3. **Cleanup Discipline (7-8)**: Stub-free code, documentation
4. **Historical Continuity (9-10)**: Artifacts maintained, lessons logged

Score tasks with: `python .rotd/pss_score.py <task_id>`

## For LLM Agents

ROTD provides structure without rigidity:
- **Explore freely** during implementation
- **Test thoroughly** before claiming completion  
- **Score systematically** for project health
- **Learn persistently** from failures

The methodology adapts to LLM workflows while maintaining engineering discipline.

## For Human Developers

ROTD complements traditional development:
- Preserves TDD benefits with LLM flexibility
- Creates auditable progress tracking
- Builds institutional memory through lessons learned
- Provides objective quality metrics

Perfect for hybrid human-AI development teams.

---

**Getting Started**: Read [ROTD.md](./ROTD.md) for complete documentation and implementation details.