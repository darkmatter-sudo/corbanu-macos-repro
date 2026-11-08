# Issue #1772 / PR #1794 review 1 — proof run (main e8dcdf6 vs fix ec5450e) (Actions run 42445589788)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42445589788
(workflow `issue-1772-probe.yml`; jobs: probe (macos-15, aarch64-apple-darwin): success; probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success).

- `commit` (before) = `e8dcdf6e2a3b91826b9002dca96c691edfd4cc5d`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `ec5450e96c4bc09c56198650d84aa690c3f8dce5`, darkmatter-sudo/CorbanuTerminal (review 1 fix 9d15845 + merge of main e8dcdf6 + rustfmt)
- `prev` = `bf389d4ba04601cf0f4fb3ada5d578a48938ba7f`, branch `prev/1772-b94f746-on-main`: the PR head b94f746 merged with the same main e8dcdf6, so prev and fix differ only by this round's change

Per OS folder: `10-probe-main.log` (before), `11-probe-fix.log` (after), `12-probe-diff.txt`,
`13-probe-prev.log` (prev), `14-quieter.txt` (louder/quieter verdicts, main vs fix),
`20-test-fix.log` (`issue_1772` tests on the fix), `21-test-reverted.log` (same tests with every
non-test file of `git diff --name-only main..fix` taken from main: the new tests must fail on their assertions),
`22-test-prev-code.log` (same tests with the sources of prev), `30-security-fix.log` (`security::` suite),
`40-fmt.log`/`41-clippy.log` (Linux only).

Results (both OSes): quieter on fix 0, louder 19; the 20 review-1 lines (`issue#r1-1..3`) ask on the fix and are
quiet on prev; `issue_1772` 5 passed on the fix; reverted 1 passed / 4 failed (assertions); prev code 4 passed /
1 failed (`issue_1772_review_lines_ask`, assertion); `security::` 812 passed / 0 failed / 5 ignored; fmt exit 0; clippy exit 0.
