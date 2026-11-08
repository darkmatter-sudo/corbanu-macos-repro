# Issue #1772 / PR #1794 review 2 — proof run (main 7218a2d vs fix c23fec9) (Actions run 42471161404)

Unedited artifacts and job logs of
https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42471161404
(workflow `issue-1772-probe.yml`; jobs: probe (macos-15, aarch64-apple-darwin): success; probe (ubuntu-24.04, x86_64-unknown-linux-gnu): success).

- `commit` (before) = `7218a2db477a1025f0cd671796376c85b96d0eb1`, CorbanuCore/CorbanuTerminal main
- `fix_commit` (after) = `c23fec92915c0487dc6528b322b9be8b493385b5`, darkmatter-sudo/CorbanuTerminal branch `fix/1772-review2`
  (PR head ec5450e + merge of main 7218a2d + a7de709 + d7e2311 + c23fec9)
- `prev` = `ec5450e96c4bc09c56198650d84aa690c3f8dce5` (PR head reviewed in round 2)

Earlier round-2 runs (not published here, links only): 42464333519 (a7de709: quieter 2 -> `read_newly` read before
`Lexer::finish`, fixed in d7e2311) and 42468220904 (d7e2311: one failing case-list test, fixed in c23fec9).

Per OS folder: same files as `../1772-review1-run-42445589788/README.md` describes.

Results (both OSes): quieter on fix 0, louder 20; the 14 review-2 probe lines (`issue#r2-*`) ask (`UnseenCode`);
`issue_1772` 9 passed on the fix; reverted 2 passed / 3 failed (assertions); prev code 4 passed / 1 failed
(`issue_1772_review_2_lines_ask`, assertion); `security::` 816 passed / 0 failed / 5 ignored; clippy exit 0.

fmt exit 1 on c23fec9: one rustfmt layout diff in `tainted_action_reading_tests.rs` (see `40-fmt.log`). Applied
verbatim in `dbf58a8e5af59fa2d031b394e38b5dedd1c6c328` (whitespace only, 5+/3-); `rustfmt-check-run-42475374169.log`
(https://github.com/darkmatter-sudo/corbanu-macos-repro/actions/runs/42475374169) shows `RUSTFMT_OK` for that file at dbf58a8.
