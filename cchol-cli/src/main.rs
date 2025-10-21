use cchol_lib::player::PlayerCharacter;
use clap::Parser;
use rpgassist::gender::Gender;

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
}

/// Simple CLI thingy.
fn main() {
    let args = Cli::parse();
    let gender = Gender::from(args.gender);

    println!("Generating stuff for {}, to be {:?}", args.name, gender);
    //let p = PlayerCharacter::new("MJS", Some(Gender::Male), None);
}