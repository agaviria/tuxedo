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
                         $XDG_CONFIG_HOME/tuxedo/tracker.toml)'")
        .subcommand(SubCommand::with_name("list")
            .aliases(&["ls", "l"])
            .about("List ascribed task items")
            .subcommands(vec![
                         SubCommand::with_name("all").alias("a").about("show all task items"),
                         SubCommand::with_name("pending").alias("p").about("show all task items pending, alpha status"),
                         SubCommand::with_name("done").alias("d").about("show all task items completed, charlie status"),
                         SubCommand::with_name("in-transit").alias("t").about("show all task items in-transit, beta status"),
                         SubCommand::with_name("void").alias("v").about("show all task items revoked, delta status"),
            ])
        )
        .subcommand(SubCommand::with_name("add")
            .aliases(&["new", "n", "a"])
            .about("Add a new task item")
            .arg(Arg::from_usage("[pr] -p --priority=[classification] 'Set risk classification for task'")
                 .possible_values(&["c", "critical", "h", "high", "m", "moderate", "l", "low"])
                 .hide_possible_values(true)
                 )
            .args_from_usage(r#"
                -c, --critical 'Set risk priority classification to critical'
                -h, --high     'Set risk priority classification to high'
                -m, --moderate 'Set risk priority classification to moderate'
                -l, --low      'Set risk priority classification to low'
                <name>         'Task name'
                [details]      'Task details'
            "#)
            .group(ArgGroup::with_name("priority")
                   .args(&["pr", "critical", "high", "moderate", "low"])
                   )
        )
}
