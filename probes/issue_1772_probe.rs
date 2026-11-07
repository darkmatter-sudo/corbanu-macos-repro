
/// #1772 probe (appended by issue-1772-probe.yml, never committed upstream).
/// Prints, for every CorbanuCore/CorbanuTerminal#1772 example (plus
/// controls), the words and redirection positions the classifier's own
/// lexer reads (`shell::simple_commands`, not a naive split) and the
/// classifier's verdict in a workspace whose ./x, sub/run.sh are harmful.
/// Then sweeps every corpus line (`CORBANU_1772_CORPUS`, main's
/// tainted_action_tests.rs) that holds `<` or `>`, so main and a branch can
/// be diffed.
#[cfg(unix)]
#[test]
#[ignore = "issue 1772 probe"]
fn issue_1772_probe() {
    let evil = "echo x >> ~/.zshenv\n";
    let x_evil: &[(&str, &str)] = &[("x", evil)];
    let x_ok: &[(&str, &str)] = &[("x", "true\n")];
    let sub_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("sub/run.sh", evil)];
    let lexed = |line: &str| -> String {
        let argv = ["bash".to_string(), "-lc".to_string(), line.to_string()];
        shell::simple_commands(&argv)
            .commands
            .iter()
            .map(|c| {
                format!(
                    "{:?} stdin={:?} here={:?} writes={:?} held={:?}",
                    c.words, c.stdin_from, c.here_string, c.writes_to, c.redirect_words
                )
            })
            .collect::<Vec<_>>()
            .join(" | ")
    };
    // The test harness prints `test ... ` without a newline first.
    println!();
    let issue: &[(&str, &[(&str, &str)], &str)] = &[
        // Item 1: a command behind a leading redirection, judged by name.
        ("1", sub_evil, "2>/dev/null cd sub && ./run.sh"),
        ("1", sub_evil, ">log cd sub; ./run.sh"),
        ("1", sub_evil, ">log pushd sub && ./run.sh"),
        ("1", x_evil, ">log $(cat x)"),
        ("1", x_evil, ">log `cat x`"),
        ("1", x_evil, ">log PATH=.:$PATH x"),
        ("1", x_evil, ">log export PATH=.:$PATH && x"),
        ("1", x_evil, "2>/dev/null find . -exec ./x \\;"),
        ("1", x_evil, ">log git -c core.pager=./x log"),
        ("1-control", sub_evil, "cd sub && ./run.sh"),
        ("1-control", sub_evil, "cd sub; ./run.sh"),
        ("1-control", sub_evil, "pushd sub && ./run.sh"),
        ("1-control", x_evil, "$(cat x)"),
        ("1-control", x_evil, "`cat x`"),
        ("1-control", x_evil, "PATH=.:$PATH x"),
        ("1-control", x_evil, "export PATH=.:$PATH && x"),
        ("1-control", x_evil, "find . -exec ./x \\;"),
        ("1-control", x_evil, "git -c core.pager=./x log"),
        ("1-control", sub_evil, "cd sub 2>/dev/null && ./run.sh"),
        ("1-control", x_ok, "2>/dev/null git status"),
        ("1-control", x_ok, ">log ls"),
        ("1-control", x_ok, "2>/dev/null cd sub && ls"),
        // Item 2: a redirect file name with a quoted or escaped space.
        ("2", x_evil, ">'a b' ./x"),
        ("2", x_evil, ">\"a b\" ./x"),
        ("2", x_evil, ">a\\ b ./x"),
        ("2", x_evil, "2>'a b' ./x"),
        ("2-control", x_evil, ">log ./x"),
        ("2-control", x_evil, ">'ab' ./x"),
        ("2-control", x_ok, "ls >'a b'"),
        ("2-control", x_ok, "echo hi >'a b'"),
        // Item 3: repeated stdin or here-string redirections.
        ("3", x_evil, "<a <b ./x"),
        ("3", x_ok, "sudo <a <b -A true"),
        ("3", x_ok, "<a <b sudo -A true"),
        ("3", x_ok, "sudo 3<a <b -A true"),
        ("3", x_ok, "sudo <<<a <<<b -A true"),
        ("3-control", x_evil, "<a ./x"),
        ("3-control", x_ok, "sudo <a -A true"),
        ("3-control", x_ok, "<a <b cat"),
        ("3-control", x_ok, "cat <a <b"),
        ("3-control", x_ok, "cat <<<a <<<b"),
        // Item 4: `&>` lexed as `&` plus `>`.
        ("4", x_evil, "&>log ./x"),
        ("4", x_evil, "sudo &>log -u root ./x"),
        ("4", x_evil, "&>log sh x"),
        ("4", sub_evil, "&>log cd sub && ./run.sh"),
        ("4", x_evil, "&>>log ./x"),
        ("4-control", x_ok, "ls &>log"),
        ("4-control", x_ok, "ls &>>log"),
        ("4-control", x_ok, "ls & >log"),
        ("4-control", x_ok, "sleep 1 & ls"),
    ];
    for (tag, files, line) in issue {
        let names: Vec<&str> = files.iter().map(|(name, _)| *name).collect();
        println!(
            "PROBE issue#{tag} {line:?} files={names:?} lexed=[{}] verdict={:?}",
            lexed(line),
            in_workspace(files, line)
        );
    }
    // Main-vs-branch sweep: every string literal in the corpus that holds a
    // redirection character, judged with harmful scripts at ./x and ./sub
    // and without a workspace.
    let Ok(path) = std::env::var("CORBANU_1772_CORPUS") else {
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
    let all_evil: &[(&str, &str)] = &[("run.sh", "true\n"), ("sub/run.sh", evil), ("x", evil)];
    let mut count = 0;
    for literal in &literals {
        if !literal.contains(['<', '>']) || literal.contains('\n') || literal.len() > 300 {
            continue;
        }
        count += 1;
        println!(
            "SWEEP {literal:?} script={:?} workspace={:?}",
            script(literal),
            in_workspace(all_evil, literal)
        );
    }
    println!("SWEEP-COUNT {count}");
}
