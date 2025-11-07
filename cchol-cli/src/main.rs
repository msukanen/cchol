use cchol_lib::{pc::PlayerCharacter, racial::Race, social::culture::Culture};
use clap::Parser;
use rpgassist::{gender::Gender, ext::IsNamed};

#[derive(Parser, Debug)]
#[command(
    name = "cchol-cli",
    version = "0.1.0",
    about = "CCHoL-CLI © 2025 Markku Sukanen.\nMIT / Apache-2.0 license applies.\n\nA simple CLI for fantasy/medieval RPG character background generation.")
]
struct Cli {
    name: String,
    #[arg(short, long)]
    gender: Option<String>,
    #[arg(short, long)]
    race: Option<String>,
    #[arg(short, long)]
    culture: Option<String>,
}

/// Simple CLI thingy.
fn main() {
    let _ = env_logger::try_init();
    let args = Cli::parse();
    let pc = PlayerCharacter::random(&args.name);

    println!("Generating stuff for {pc:?}");
    //let p = PlayerCharacter::new("MJS", Some(Gender::Male), None);
 }