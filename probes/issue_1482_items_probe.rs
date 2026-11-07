
/// #1482 items 2-5 probe (appended by issue-1482-items-probe.yml, never
/// committed upstream). Prints, for every #1482 item 2-5 example and for
/// every corpus line (`CORBANU_1482_CORPUS`, main's tainted_action_tests.rs)
/// that holds a redirection, `eval` or `sudo`, the command word and the
/// classifier's verdict, so main and a branch can be diffed.
#[cfg(unix)]
#[test]
#[ignore = "issue 1482 items probe"]
fn issue_1482_items_probe() {
    let evil = "echo x >> ~/.zshenv\n";
    let words = |line: &str| line.split(' ').map(str::to_string).collect::<Vec<_>>();
    let x_evil: &[(&str, &str)] = &[("x", evil)];
    let x_ok: &[(&str, &str)] = &[("x", "true\n")];
    let sub_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("sub/run.sh", evil)];
    let three_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("3/run.sh", evil)];
    // The test harness prints `test ... ` without a newline first.
    println!();
    let issue: &[(&str, &[(&str, &str)], &str)] = &[
        ("2", sub_evil, "sudo -R >log -C sub ./run.sh"),
        ("2", x_evil, ">log ./x"),
        ("2", x_evil, "sudo >log ./x"),
        ("2", x_evil, "env >log ./x"),
        ("2", x_evil, "2>log sh x"),
        ("2", x_evil, "<log sh x"),
        ("2-control", x_ok, "ls >log"),
        ("2-control", x_ok, "2>/dev/null ls"),
        ("2-control", x_ok, "sudo ./x >log"),
        ("2-control", x_evil, "sudo ./x >log"),
        ("3", three_evil, "eval 'sudo -C 3 ./run.sh'"),
        ("3", three_evil, "eval sudo -C 3 ./run.sh"),
        ("3", sub_evil, "eval 'sudo -D sub ./run.sh'"),
        ("3", x_evil, "eval ./x"),
        ("3-control", three_evil, "sudo -C 3 ./run.sh"),
        ("3-control", x_ok, "eval ./x"),
        ("3-control", x_ok, "eval true"),
        ("4", x_ok, "sudo -i ./x"),
        ("4", x_ok, "sudo -iu root ./x"),
        ("4", x_ok, "sudo -A ./x"),
        ("4", x_ok, "sudo -nA ./x"),
        ("4", x_ok, "sudo --askpass ./x"),
        ("4", x_ok, "sudo -e ./x"),
        ("4", x_ok, "sudo -u root -e ./x"),
        ("4", x_ok, "sudo --edit ./x"),
        ("4", x_ok, "sudoedit ./x"),
        ("4-s", x_ok, "sudo -s ./x"),
        ("4-control", x_ok, "sudo -u root ./x"),
        ("4-control", x_ok, "sudo -E ./x"),
        ("4-control", x_ok, "sudo -u e ./x"),
        ("4-control", x_ok, "sudo -p A ./x"),
    ];
    for (tag, files, line) in issue {
        let names: Vec<&str> = files.iter().map(|(name, _)| *name).collect();
        println!(
            "PROBE issue#{tag} {line:?} files={names:?} command_word={:?} verdict={:?}",
            indirect::command_word(&words(line)),
            in_workspace(files, line)
        );
    }
    // Item 5: argv elements.
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::write(root.path().join("x"), "true\n").expect("x");
    let cwd = root.path().to_string_lossy().into_owned();
    for (tag, argv) in [
        ("5", &["true ./x"][..]),
        ("5", &["env", "true ./x"]),
        ("5", &["sudo", "-u", "root", "true ./x"]),
        ("5", &["true;./x"]),
        ("5-control", &["true", "./x"]),
        ("5-control", &["./x"]),
        ("5-control", &["echo", "a b"]),
        ("5-control", &["bash", "-lc", "true ./x"]),
        ("5-control", &["sudo", "sh", "-c", "true ./x"]),
    ] {
        println!(
            "PROBE issue#{tag} argv={argv:?} files=[\"x\"] verdict={:?}",
            classify_with(&shell_in(&cwd, argv), &fixture(HOME), Some(root.path()))
        );
    }
    // Main-vs-branch sweep: every string literal in the corpus that holds a
    // redirection, `eval` or `sudo`, judged with harmful scripts at ./x,
    // ./sub and ./3 and without a workspace.
    let Ok(path) = std::env::var("CORBANU_1482_CORPUS") else {
        return;
    };
    let text = std::fs::read_to_string(path).expect("corpus");
    let mut literals = std::collections::BTreeSet::new();
    let mut chars = text.chars().peekable();
    let mut previous = ' ';
    while let Some(ch) = chars.next() {
        if ch == '"' && previous == 'r' {
            // A raw string: no escapes.
            let literal: String = chars.by_ref().take_while(|inner| *inner != '"').collect();
            literals.insert(literal);
            previous = '"';
            continue;
        }
        if ch == '"' && previous != '#' && previous != '\'' {
            let mut literal = String::new();
            let mut escaped = false;
            for inner in chars.by_ref() {
                if escaped {
                    literal.push(match inner {
                        'n' => '\n',
                        other => other,
                    });
                    escaped = false;
                } else if inner == '\\' {
                    escaped = true;
                } else if inner == '"' {
                    break;
                } else {
                    literal.push(inner);
                }
            }
            literals.insert(literal);
            previous = '"';
            continue;
        }
        previous = ch;
    }
    let all_evil: &[(&str, &str)] = &[
        ("run.sh", "true\n"),
        ("sub/run.sh", evil),
        ("3/run.sh", evil),
        ("x", evil),
    ];
    let mut count = 0;
    for literal in &literals {
        let wrapped = literal.contains(['<', '>']) || literal.contains("eval") || literal.contains("sudo");
        if !wrapped || literal.contains('\n') || literal.len() > 300 {
            continue;
        }
        count += 1;
        println!(
            "SWEEP {literal:?} command_word={:?} script={:?} workspace={:?}",
            indirect::command_word(&words(literal)),
            script(literal),
            in_workspace(all_evil, literal)
        );
    }
    println!("SWEEP-COUNT {count}");
}
