// Command line parsing for rbps
use clap::{
    crate_description,
    crate_name,
    crate_version,
    Arg,
    ArgAction,
    ArgMatches,
    Command,
};

fn create_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .term_width(80)
        .arg(
            Arg::new("BPS_FILE_PATH")
                .action(ArgAction::Set)
                .help("Path to BPS patch file")
                .long("bps-file")
                .required(true)
                .short('b')
                .value_name("BPSFILE")
        )
        .arg(
            Arg::new("ROM_FILE_PATH")
                .action(ArgAction::Set)
                .help("Path to original ROM file")
                .long("rom-file")
                .required(true)
                .short('r')
                .value_name("ROMFILE")
        )
        .arg(
            Arg::new("OUTPUT_FILE_PATH")
                .action(ArgAction::Set)
                .help("Path to write patched ROM to")
                .long("output-file")
                .required(true)
                .short('o')
                .value_name("OUTPUTFILE")
        )
}

pub fn parse_args() -> ArgMatches {
    create_app().get_matches()
}
