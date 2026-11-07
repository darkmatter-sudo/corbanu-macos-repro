# Issue #1482 items 2-5 — final proof (Actions run 42371229508)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42371229508
(workflow `issue-1482-items-probe.yml`, ubuntu-24.04 + macos-15, both jobs success).

- `commit` (before) = `f3ae287bec86fa729037786abbcb82c9c2e62b00`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `130d26e56a965a96de549e9ebf644cc1416e7015`, branch `fix/1482-items-2-5` on darkmatter-sudo/CorbanuTerminal
  (based on main `18fa372aaa`; the main commits after it change no file under `codex-rs/core/src/security/`)

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`14-quieter.txt` (louder/quieter verdicts, main vs fix), `20-test-fix.log` (`issue_1482` tests on the fix),
`21-test-reverted.log` (same tests with `tainted_action.rs`/`indirect.rs` from main: the new test must fail),
`30-security-fix.log` (`security::` suite), `40-fmt.log`/`41-clippy.log` (Linux only).
