# corbanu-macos-repro

Five-run reproduction attempt for the macOS SIGSEGV in
[CorbanuCore/CorbanuTerminal#1367](https://github.com/CorbanuCore/CorbanuTerminal/issues/1367).

`.github/workflows/repro.yml` (manual `workflow_dispatch`) runs on a GitHub-hosted
`macos-15` runner, the same runner label as the crashed CI job. It:

1. clones CorbanuCore/CorbanuTerminal and checks out `1e90ae8def`, recording
   `git rev-parse HEAD`, `sw_vers`, `uname -m` and the CPU brand string;
2. runs, five times in sequence from `codex-rs/` (where the Cargo workspace lives),
   `cargo clean` followed by
   `cargo test -p codex-core --test all -- suite::issue_506 suite::issue_584 suite::issue_620_622 suite::issue_736_740 suite::issue_747`;
3. records start/end time (UTC), PASS/FAIL and exit code per run and uploads all logs
   as the `macos-repro-logs` artifact.

The job uses the same job-level environment as the upstream
`rust-ci-core-security.yml` (`RUST_MIN_STACK=8388608`, `RUST_BACKTRACE=1`,
`CARGO_INCREMENTAL=0`, `CORBANU_TEST_NO_NATIVE_KEYRING=1`) and `TMPDIR=/tmp` as in the
failing macOS step. No upstream code is changed.
