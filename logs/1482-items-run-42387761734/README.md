# Issue #1482 items 2-5 — review 1, final proof (Actions run 42387761734)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42387761734
(workflow `issue-1482-items-probe.yml`; jobs: probe (macos-15, aarch64-apple-darwin): success; probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success).

- `commit` (before) = `dfeb26a1225d5eed0b22d58c6396dda2ec7748a4`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `c79d6ce49984ffa450c4f0abe60a421706b354b9`, branch `fix/1482-items-2-5` on darkmatter-sudo/CorbanuTerminal
- `prev` = `130d26e56a965a96de549e9ebf644cc1416e7015` (the head reviewed in review 1 of #1719)

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`13-probe-prev.log` (reviewed head), `14-quieter.txt` (louder/quieter verdicts, main vs fix),
`20-test-fix.log` (`issue_1482` tests on the fix), `21-test-reverted.log` (same tests with the
security sources from main: the new test must fail), `22-test-prev-code.log` (same tests with the
sources of the reviewed head), `30-security-fix.log` (`security::` suite), `40-fmt.log`/`41-clippy.log` (Linux only).
