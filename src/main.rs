// Simple test app for libbps
use libbps::{
    Errors,
    Patcher,
};

mod cli;

fn main() -> Result<(), Errors> {
    let args = cli::parse_args();

    // These are all required
    let bps_path = args.get_one::<String>("BPS_FILE_PATH").unwrap();
    let rom_path = args.get_one::<String>("ROM_FILE_PATH").unwrap();
    let output_path = args.get_one::<String>("OUTPUT_FILE_PATH").unwrap();

    let patcher = Patcher::new(
        &bps_path,
        &rom_path,
        &output_path,
    )?;

    println!("{patcher:#?}");

    Ok(())
}
