use super::*;

fn build_app() -> Command<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> Command<'static> {
    Command::new(s)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .subcommand(
            Command::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .help("the case to test"),
            ),
        )
}

#[test]
fn elvish() {
    let mut cmd = build_app();
    common(Elvish, &mut cmd, "my_app", ELVISH);
}

static ELVISH: &str = r#"
use builtin;
use str;

set edit:completion:arg-completer[my_app] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'my_app'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'my_app'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand test 'tests things'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
        &'my_app;help'= {
        }
    ]
    $completions[$command]
}
"#;

#[test]
fn elvish_with_special_commands() {
    let mut cmd = build_app_special_commands();
    common(Elvish, &mut cmd, "my_app", ELVISH_SPECIAL_CMDS);
}

fn build_app_special_commands() -> Command<'static> {
    build_app_with_name("my_app")
        .subcommand(
            Command::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .help("the other case to test"),
            ),
        )
        .subcommand(Command::new("some-cmd-with-hyphens").alias("hyphen"))
}

static ELVISH_SPECIAL_CMDS: &str = r#"
use builtin;
use str;

set edit:completion:arg-completer[my_app] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'my_app'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'my_app'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand test 'tests things'
            cand some_cmd 'tests other things'
            cand some-cmd-with-hyphens 'some-cmd-with-hyphens'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
        &'my_app;some_cmd'= {
            cand --config 'the other case to test'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
        &'my_app;some-cmd-with-hyphens'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
        &'my_app;help'= {
        }
    ]
    $completions[$command]
}
"#;

#[test]
fn elvish_with_aliases() {
    let mut cmd = build_app_with_aliases();
    common(Elvish, &mut cmd, "cmd", ELVISH_ALIASES);
}

fn build_app_with_aliases() -> Command<'static> {
    Command::new("cmd")
        .version("3.0")
        .about("testing bash completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .takes_value(true),
        )
        .arg(Arg::new("positional"))
}

static ELVISH_ALIASES: &str = r#"
use builtin;
use str;

set edit:completion:arg-completer[cmd] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'cmd'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'cmd'= {
            cand -o 'cmd option'
            cand -O 'cmd option'
            cand --option 'cmd option'
            cand --opt 'cmd option'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -f 'cmd flag'
            cand -F 'cmd flag'
            cand --flag 'cmd flag'
            cand --flg 'cmd flag'
        }
    ]
    $completions[$command]
}
"#;
