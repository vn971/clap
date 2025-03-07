use super::*;
use crate::Fig;

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
fn fig() {
    let mut cmd = build_app();
    common(Fig, &mut cmd, "myapp", FIG);
}

static FIG: &str = r#"const completion: Fig.Spec = {
  name: "myapp",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
      ],
      args: {
        name: "subcommand",
        isOptional: true,
      },
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;

#[test]
fn fig_with_special_commands() {
    let mut cmd = build_app_special_commands();
    common(Fig, &mut cmd, "my_app", FIG_SPECIAL_CMDS);
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

static FIG_SPECIAL_CMDS: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some_cmd",
      description: "tests other things",
      options: [
        {
          name: "--config",
          description: "the other case to test",
          args: {
            name: "config",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some-cmd-with-hyphens",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
      ],
      args: {
        name: "subcommand",
        isOptional: true,
      },
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;

#[test]
fn fig_with_special_help() {
    let mut cmd = build_app_special_help();
    common(Fig, &mut cmd, "my_app", FIG_SPECIAL_HELP);
}

fn build_app_special_help() -> Command<'static> {
    Command::new("my_app")
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .help("For more information see `echo test`"),
        )
        .arg(Arg::new("backslash").long("backslash").help("Avoid '\\n'"))
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .help("Execute the shell command with $SHELL"),
        )
}

static FIG_SPECIAL_HELP: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "",
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
    {
      name: "--single-quotes",
      description: "Can be 'always', 'auto', or 'never'",
    },
    {
      name: "--double-quotes",
      description: "Can be \"always\", \"auto\", or \"never\"",
    },
    {
      name: "--backticks",
      description: "For more information see `echo test`",
    },
    {
      name: "--backslash",
      description: "Avoid '\\n'",
    },
    {
      name: "--brackets",
      description: "List packages [filter]",
    },
    {
      name: "--expansions",
      description: "Execute the shell command with $SHELL",
    },
  ],
};

export default completion;
"#;

#[test]
fn fig_with_aliases() {
    let mut cmd = build_app_with_aliases();
    common(Fig, &mut cmd, "cmd", FIG_ALIASES);
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

static FIG_ALIASES: &str = r#"const completion: Fig.Spec = {
  name: "cmd",
  description: "testing bash completions",
  options: [
    {
      name: ["-o", "-O", "--option", "--opt"],
      description: "cmd option",
      args: {
        name: "option",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
    {
      name: ["-f", "-F", "--flag", "--flg"],
      description: "cmd flag",
    },
  ],
  args: {
    name: "positional",
    isOptional: true,
  },
};

export default completion;
"#;

#[test]
fn fig_with_sub_subcommands() {
    let mut cmd = build_app_sub_subcommands();
    common(Fig, &mut cmd, "my_app", FIG_SUB_SUBCMDS);
}

fn build_app_sub_subcommands() -> Command<'static> {
    build_app_with_name("my_app").subcommand(
        Command::new("some_cmd")
            .about("top level subcommand")
            .subcommand(
                Command::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("--config")
                        .takes_value(true)
                        .help("the other case to test"),
                ),
            ),
    )
}

static FIG_SUB_SUBCMDS: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some_cmd",
      description: "top level subcommand",
      subcommands: [
        {
          name: "sub_cmd",
          description: "sub-subcommand",
          options: [
            {
              name: "--config",
              description: "the other case to test",
              args: {
                name: "config",
                isOptional: true,
              },
            },
            {
              name: ["-h", "--help"],
              description: "Print help information",
            },
            {
              name: ["-V", "--version"],
              description: "Print version information",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help information",
            },
            {
              name: ["-V", "--version"],
              description: "Print version information",
            },
          ],
          args: {
            name: "subcommand",
            isOptional: true,
          },
        },
      ],
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
      ],
      args: {
        name: "subcommand",
        isOptional: true,
      },
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;
