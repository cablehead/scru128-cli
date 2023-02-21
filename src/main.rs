use std::str::FromStr;
use std::io::Read;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse {
        #[clap(long, value_parser)]
        id: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Parse { id }) => {
            println!("id: {:?}", id);
        }
        None => {
            let id = scru128::new();
            println!("{}", id);
        }
    }


    /*
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let id = scru128::Scru128Id::from_str(&buf).unwrap();
    println!("{}", (id.timestamp() as f64)/ 1000.0);
    */
}
