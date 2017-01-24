extern crate clap;

use self::clap::{App, Arg, AppSettings, SubCommand};

pub fn gen_cli() -> App<'static, 'static> {
    // scan_task validates task arguments at compile time
    let scan_task = Arg::with_name("task")
        .required(true)
        .takes_value(true);

    App::new("tracker")
        .author("Alejandro R. Gaviria")
        .version("v0.1.0")
        .about("Productivity tool for tracking task duties")
        .setting(AppSettings::GlobalVersion)
        .arg_from_usage("[file] -f --file=<FILE> 'task file to use (default: \
                         $XDG_CONFIG_HOME/rust-todo/tasks.toml)'")
        .subcommand(SubCommand::with_name("list")
            .aliases(&["ls", "l"])
            .about("List ascribed task items")
            .subcommands(vec![
                         SubCommand::with_name("all").alias("a").about("show all task items"),
                         SubCommand::with_name("pending").alias("p").about("show all pending task items"),
                         SubCommand::with_name("completed").alias("c").about("show all completed task items"),
            ])
        )

}
