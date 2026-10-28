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

## Full-sequence stress run (`stress-full.yml`)

`stress.yml` ran only the #902 step (60/60 passed, run 40080772518). In the failing upstream
job the crash came in the sixth test step, after the `security::` and `pf_23_s01`/`pf_25_s01`
steps had already run on the same runner and home folder. `.github/workflows/stress-full.yml`
(manual `workflow_dispatch`, inputs `commit`, `iterations`) builds what
`rust-ci-core-security.yml` builds (`--lib` and `--test all` test binaries plus the
`codex-code-mode-host`/`codex-rmcp-client` helpers), then replays all six test steps of that
workflow in order, with the same `TMPDIR` settings, `iterations` times on one `macos-15` runner.
It writes one summary line per step, the full log of every failing step, and every new `*.ips`
crash report, as the `macos-stress-full-logs` artifact. No upstream code is changed and nothing
is posted upstream.

## request_compression stack probe (`request-compression-stack.yml`)

For the maintainers' follow-up (b) on
[#1396](https://github.com/CorbanuCore/CorbanuTerminal/pull/1396#issuecomment-6455141884)
(decision D201): `cargo test -p codex-core --test all request_compression` overflowing the
default test-thread stack in debug builds. Manual `workflow_dispatch` (input `commit`, optional
`fix_repo`/`fix_commit`) on `ubuntu-24.04` and `macos-15`:

1. runs that exact command with `RUST_MIN_STACK` unset, then at 4 and 8 MiB, one log per run
   (exit code, "has overflowed its stack" lines, test result);
2. on Linux, runs the overflowing case under gdb and prints the faulting thread's backtrace with
   the stack bytes each frame uses, to name the oversized future or frame before choosing a fix;
3. if a candidate fix commit is given, runs the same command on it with `RUST_MIN_STACK` unset,
   1.5 MiB and 1 MiB (headroom the fix leaves).

Logs are uploaded as `request-compression-stack-<runner>` artifacts. Nothing is posted upstream.
