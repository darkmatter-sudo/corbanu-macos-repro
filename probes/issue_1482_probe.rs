
/// #1482 probe (appended by issue-1482-probe.yml, never committed upstream).
/// Prints, for every #1482 example and for every wrapper line in the corpus
/// (`CORBANU_1482_CORPUS`, main's tainted_action_tests.rs), the command word
/// and the classifier's verdict, so main and a branch can be diffed.
#[cfg(unix)]
#[test]
#[ignore = "issue 1482 probe"]
fn issue_1482_probe() {
    let evil = "echo x >> ~/.zshenv\n";
    let words = |line: &str| line.split(' ').map(str::to_string).collect::<Vec<_>>();
    let x_evil: &[(&str, &str)] = &[("x", evil)];
    let x_ok: &[(&str, &str)] = &[("x", "true\n")];
    let sub_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("sub/run.sh", evil)];
    let three_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("3/run.sh", evil)];
    let issue: &[(&str, &[(&str, &str)], &str)] = &[
        ("1a", x_evil, "sudo -nC 3 sh x"),
        ("1b", sub_evil, "sudo -nD sub ./run.sh"),
        ("1c", sub_evil, "sudo --chdir sub ./run.sh"),
        ("1-note", x_evil, "sudo --chdir bash x"),
        ("1-note-control", x_evil, "sudo x"),
        ("1-note-control", x_evil, "sudo bash x"),
        ("2", sub_evil, "sudo -R >log -C sub ./run.sh"),
        ("3", three_evil, "eval 'sudo -C 3 ./run.sh'"),
        ("3-control", three_evil, "sudo -C 3 ./run.sh"),
        ("4", x_ok, "sudo -i ./x"),
        ("4", x_ok, "sudo -A ./x"),
        ("4", x_ok, "sudo -e ./x"),
        ("4", x_ok, "sudo -s ./x"),
    ];
    for (tag, files, line) in issue {
        let names: Vec<&str> = files.iter().map(|(name, _)| *name).collect();
        println!(
            "PROBE issue#{tag} {line:?} files={names:?} command_word={:?} verdict={:?}",
            indirect::command_word(&words(line)),
            in_workspace(files, line)
        );
    }
    // Item 5: one argv element holding a space.
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::write(root.path().join("x"), "true\n").expect("x");
    let cwd = root.path().to_string_lossy().into_owned();
    println!(
        "PROBE issue#5 argv=[\"true ./x\"] files=[\"x\"] verdict={:?}",
        classify_with(
            &shell_in(&cwd, &["true ./x"]),
            &fixture(HOME),
            Some(root.path())
        )
    );
    // Main-vs-branch sweep: every string literal in the corpus that runs a
    // wrapper with an option, judged with harmful scripts at ./x, ./sub and
    // ./3 and without a workspace.
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
    let wrappers: Vec<&str> = indirect::WRAPPERS.iter().map(|(name, _, _)| *name).collect();
    let all_evil: &[(&str, &str)] = &[
        ("run.sh", "true\n"),
        ("sub/run.sh", evil),
        ("3/run.sh", evil),
        ("x", evil),
    ];
    let mut count = 0;
    for literal in &literals {
        let wrapped = literal.split(|ch: char| ";&|(".contains(ch)).any(|part| {
            let mut parts = part.split_whitespace();
            matches!(
                (parts.next(), parts.next()),
                (Some(first), Some(second))
                    if wrappers.contains(&super::shell::basename(first))
                        && second.starts_with('-')
            )
        });
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
