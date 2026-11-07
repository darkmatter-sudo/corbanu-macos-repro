# Issue #1482 items 2-5 — review 1, first attempt ("just ask"; 11 security:: failures) (Actions run 42380715959)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42380715959
(workflow `issue-1482-items-probe.yml`; jobs: probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success; probe (macos-15, aarch64-apple-darwin): success).

- `commit` (before) = `b06efb2d6a552099073f8101dbbc93e2606dbb7d`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `9c872e7c5804703b9f7b42a7fc0135e6dde64f91`, branch `fix/1482-items-2-5` on darkmatter-sudo/CorbanuTerminal
- `prev` = `130d26e56a965a96de549e9ebf644cc1416e7015` (the head reviewed in review 1 of #1719)

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`13-probe-prev.log` (reviewed head), `14-quieter.txt` (louder/quieter verdicts, main vs fix),
`20-test-fix.log` (`issue_1482` tests on the fix), `21-test-reverted.log` (same tests with the
security sources from main: the new test must fail), `22-test-prev-code.log` (same tests with the
sources of the reviewed head), `30-security-fix.log` (`security::` suite), `40-fmt.log`/`41-clippy.log` (Linux only).
