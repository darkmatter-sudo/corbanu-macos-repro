# Issue #1482 items 2-5 — review 2 proof run (main 9361108 vs fix 633e35b) (Actions run 42400145608)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42400145608
(workflow `issue-1482-items-probe.yml`; jobs: probe (macos-15, aarch64-apple-darwin): success; probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success).

- `commit` (before) = `9361108dab21d738e5495bb6f9aba9f9fb07b56e`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `633e35bc8ded44ef148a2e4b9765533686b9e312`, branch `fix/1482-items-2-5` on darkmatter-sudo/CorbanuTerminal
- `prev` = `6bf195dba2712ffd6f93e7d88d4adb45fd6ac964` (the review 1 code merged with main, before the review 2 fixes)

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`13-probe-prev.log` (prev), `14-quieter.txt` (louder/quieter verdicts, main vs fix),
`20-test-fix.log` (`issue_1482` tests on the fix), `21-test-reverted.log` (same tests with the
security sources from main: the new test must fail), `22-test-prev-code.log` (same tests with the
sources of prev), `30-security-fix.log` (`security::` suite), `40-fmt.log`/`41-clippy.log` (Linux only).
