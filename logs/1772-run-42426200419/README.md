# Issue #1772 — before/after proof run (main 29530d2 vs fix b94f746) (Actions run 42426200419)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42426200419
(workflow `issue-1772-probe.yml`; jobs: probe (macos-15, aarch64-apple-darwin): success; probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success).

- `commit` (before) = `29530d2cb9986dd3a4d05a9c707444f06c3ab22e`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `b94f746254d5ba29a23b8eeb85608cee051b9e18`, branch `fix/1772-redirections` on darkmatter-sudo/CorbanuTerminal
- `prev` = `abd0445416e1a47a05bf42d1368fdc75dd520115` (the fix before its last commit)

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`13-probe-prev.log` (prev), `14-quieter.txt` (louder/quieter verdicts, main vs fix),
`20-test-fix.log` (`issue_1772` tests on the fix), `21-test-reverted.log` (same tests with the
security sources from main: the new tests must fail on their assertions), `22-test-prev-code.log`
(same tests with the sources of prev), `30-security-fix.log` (`security::` suite),
`40-fmt.log`/`41-clippy.log` (Linux only).
