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

## Stress run (`stress.yml`)

`.github/workflows/stress.yml` (manual `workflow_dispatch`, inputs `commit`, `iterations`)
builds the codex-core `all` test binary once on `macos-15` and runs the exact #902 step
of `rust-ci-core-security.yml`
(`cargo test --locked --target aarch64-apple-darwin -p codex-core --test all -- suite::issue_506:: suite::issue_584:: suite::issue_620_622:: suite::issue_736_740:: suite::issue_747:: suite::issue_748::`)
`iterations` times, alternating `TMPDIR` unset and `TMPDIR=/tmp` as that workflow does.
It keeps a one-line summary per run, the full log of every failing run, and any macOS
crash report (`~/Library/Logs/DiagnosticReports/all-*.ips`) written for the test binary,
as the `macos-stress-logs` artifact. Same job-level environment as above. No upstream code
is changed and nothing is posted upstream.
