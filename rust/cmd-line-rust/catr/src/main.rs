use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com)")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    
    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        println!("{filename}");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
