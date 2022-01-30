use std::collections::HashMap;
use clap::{AppSettings, Parser, Subcommand};
use cube_shuffle_core::distribution_shuffle::{Pile, shuffle};
use parse_display::{Display, FromStr};
use rand::prelude::StdRng;
use rand::SeedableRng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short = 'r', long)]
    #[clap(value_name = "seed number")]
    seed: Option<u64>,

    #[clap(short = 'f', long)]
    #[clap(value_name = "output format")]
    #[clap(default_value = "PrettyDebug")]
    format: Formats,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Distribution {
        #[clap(short, long)]
        #[clap(value_name = "piles")]
        piles: Vec<PileInput>,

        #[clap(short = 's', long)]
        #[clap(value_name = "pack size")]
        #[clap(default_value_t = 15)]
        pack_size: u32,
    }
}

#[derive(Debug, FromStr)]
enum Formats {
    Debug,
    PrettyDebug,
}

#[derive(Debug, Display, FromStr)]
#[display("{name}={definition}")]
struct PileInput {
    name: String,
    definition: Pile,
}

fn main() {
    let cli: Cli = Cli::parse();
    let mut rng = match cli.seed {
        None => { StdRng::from_entropy() }
        Some(s) => { StdRng::seed_from_u64(s) }
    };
    match &cli.command {
        Commands::Distribution { piles, pack_size } => {
            let piles_map: HashMap<_, _> = piles
                .iter()
                .map(|p| (&p.name, p.definition))
                .collect();
            let shuffled = shuffle(&piles_map, *pack_size, &mut rng);
            match cli.format {
                Formats::Debug => { println!("{:?}", shuffled)}
                Formats::PrettyDebug => { println!("{:#?}", shuffled) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cli;

    #[test]
    fn verify_app() {
        use clap::IntoApp;
        Cli::into_app().debug_assert();
    }
}