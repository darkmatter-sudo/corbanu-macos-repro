# PR #1794 round 2 — Windows clippy fix (Actions run 42476775622)

PR CI on head `dbf58a8e5af59fa2d031b394e38b5dedd1c6c328`: `rust-ci / Lint/Build — windows-2022 - x86_64-pc-windows-msvc /
cargo clippy` failed (https://github.com/CorbanuCore/CorbanuTerminal/actions/runs/42475505368/job/128715935892) with 5
errors in the two new test files: `tainted_action_redirect_tests.rs` holds only `#[cfg(unix)]` items since round 2, so its
two imports are unused on Windows, and `new_verdict` in `tainted_action_reading_tests.rs` (used only by a unix test)
imported the unix-only helpers `HOME`, `action_in`, `workspace`.

Fix `21b238bd1b3d3797fff359057e8e42bbbae91856` (parent dbf58a8): `#[cfg(unix)]` on those two imports, on `new_verdict`
and on `REVIEW_QUIET_ON_MAIN`. No test or code change; same tests on unix.

Unedited job logs:

- `windows-clippy-run-42476775622.log` — workflow `windows-clippy-check.yml` at
  1ec82f20078ce91fec63f576aeb2446fd570a0cd, windows-2022, rustc 1.99.1 (repo toolchain),
  `cargo clippy -p codex-core --target x86_64-pc-windows-msvc --tests -- -D warnings` per commit:
  - `=== RESULT 21b238bd1b3d3797fff359057e8e42bbbae91856 clippy exit 0`
  - `dbf58a8` (fail-before): the same 5 errors as the PR CI job, `could not compile codex-core (lib test) due to 5 previous errors`.
  - The run is marked failed and has no RESULT line for dbf58a8: Actions' bash runs with `-e`, so the step ended at that
    clippy's exit 101. Fixed in the workflow afterwards (`set +e`, 3f45b77df6cfb20d52e17789df8c738786348879).
  - 9 `Filename too long` checkout errors are long paths under `qa/` (none under `codex-rs/`).
- `rustfmt-check-run-42476777945.log`, `rustfmt-check-run-42476780509.log` — `RUSTFMT_OK` for both changed files at 21b238b.
