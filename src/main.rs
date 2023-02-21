use std::io::Read;
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a scru128 ID (default)
    Generate {},

    /// Parse a scru128 ID to a UNIX timestamp
    Parse {
        #[clap(value_parser)]
        id: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    let command = args.command.unwrap_or(Commands::Generate {});

    match command {
        Commands::Generate {} => {
            let id = scru128::new();
            println!("{}", id);
        }

        Commands::Parse { id } => {
            let id = match id {
                Some(id) => id,
                None => {
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf).unwrap();
                    buf.trim().to_string()
                }
            };
            let id = scru128::Scru128Id::from_str(&id).unwrap();
            println!("{}", (id.timestamp() as f64) / 1000.0);
        }
    }
}
