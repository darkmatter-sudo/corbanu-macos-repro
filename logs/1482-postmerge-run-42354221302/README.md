# Issue #1482 — post-merge probe (Actions run 42354221302)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42354221302
(workflow `issue-1482-probe.yml`, ubuntu-24.04 + macos-15, both jobs success).

- `commit` (before) = `781b935ff783178f146d02815f8fc01332ae21a4`, CorbanuCore/CorbanuTerminal main just before the merge
- `fix_commit` (after) = `84b3f3765c5c0b75d53589a606bd0f542c7fd4c3`, merge commit of PR #1575 on main

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`14-quieter.txt`, `20-test-fix.log` (regression tests on merged main), `21-test-reverted.log`
(same tests with `indirect.rs`/`git_config.rs` reverted to pre-merge main: must fail),
`30-security-fix.log` (`security::` suite), `40-fmt.log`/`41-clippy.log` (Linux only).
