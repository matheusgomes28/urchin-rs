use std::{io::Read, path::Path};

use clap::Parser;
use common::UrchinConfig;


#[derive(clap::Parser)]
struct ProgramArgs {
    // path to the config toml
    #[clap(long, short)]
    config_file: String,
}

fn read_file<T: AsRef<Path>>(path: T) -> anyhow::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn main() {
    // TODO : initialise logging here
    

    match try_main() {
        Ok(()) => {
            println!("program done");
        },
        Err(e) => {
            eprintln!("error: {}", e)
        }
    }
}

fn try_main() -> anyhow::Result<()> {
    // Read the config
    let args = ProgramArgs::parse();

    let config_contents = read_file(args.config_file)?;
    let config = toml::from_str::<UrchinConfig>(&config_contents)?;

    // TODO : we probably don't want to print secrets ;)
    println!("config");
    println!("{:#?}", config);

    Ok(())
}
