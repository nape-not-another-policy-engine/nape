# AGENTS.md

This file is the entry point for agent work inside `nape`.

## Start Here

When resuming `nape` work:

1. read [docs/1-plan/roadmap.md](docs/1-plan/roadmap.md)
2. open the active plan under `docs/1-plan/plans/`
3. open the matching handoff under `docs/1-plan/handoffs/`
4. re-ground on the current `nape` architecture and evaluator boundary docs

If the work depends on `nape-evaluator`, also read the sibling repo planning entry points before changing the CLI integration seam.

## Planning Rules

- keep `nape`-specific planning documents under `docs/1-plan/`
- do not create new `TEMP-` planning files at the root of `docs/`
- treat `docs/1-plan/roadmap.md` as the status index for active `nape` workstreams
- each active workstream should have:
  - one plan document in `docs/1-plan/plans/`
  - one handoff document in `docs/1-plan/handoffs/`
- when a workstream changes materially, update the roadmap and the relevant handoff before stopping

## Baseline Rules

- `nape` owns orchestration, workspace state, report generation, and evaluator process integration
- `nape-evaluator` owns test execution against evidence
- the current evaluator seam is documented in `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
- current `nape` planning must treat evaluator output integration as dependent on the current and proposed contracts in the sibling `nape-evaluator` repo

## Working Order

Use this order when making `nape` changes:

1. roadmap
2. active plan
3. active handoff
4. `docs/maintainers/architecture.md`
5. `docs/reference/evaluation-report-traceability.md`
6. `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
7. code and tests

## Completion Rule

When a plan is completed:

- move durable conclusions into permanent docs or ADRs where appropriate
- update the roadmap status
- delete or archive planning docs only after their information has been disseminated
