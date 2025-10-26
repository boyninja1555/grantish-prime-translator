use crate::grantish_prime::{english_grantish_prime, grantish_prime_english};
use clap::{Parser, Subcommand};
use copypasta::{ClipboardContext, ClipboardProvider};
mod grantish_prime;

#[derive(Parser)]
#[command(name = "Grantish Prime Translator")]
#[command(version = "v0.1.1")]
#[command(
    about = "Translator for Grantish Prime",
    long_about = "Translates to and from Grantish Prime"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Translate Grantish Prime to English")]
    ToEnglish {
        #[arg(num_args = 1..)]
        input: Vec<String>,
    },

    #[command(about = "Translate English to Grantish Prime")]
    FromEnglish {
        #[arg(num_args = 1..)]
        input: Vec<String>,
    },
}

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let cli = Cli::parse();
    let map_a = english_grantish_prime();
    let map_b = grantish_prime_english();

    match &cli.command {
        Commands::ToEnglish { input } => {
            let allowed_chars: Vec<char> = map_b.keys().copied().collect();
            let text: String = input
                .join("_")
                .trim()
                .to_lowercase()
                .chars()
                .filter(|ch| allowed_chars.contains(ch))
                .collect();
            println!("Translating to English... ({})", text);

            let mut output = String::new();
            for ch in text.chars() {
                let mapped = map_b.get(&ch).cloned().unwrap_or('☐');
                output.push(mapped);
            }

            println!("{}", output);
        }

        Commands::FromEnglish { input } => {
            let allowed_chars: Vec<char> = map_a.keys().copied().collect();
            let text: String = input
                .join(" ")
                .trim()
                .to_lowercase()
                .chars()
                .filter(|ch| allowed_chars.contains(ch))
                .collect();
            println!("Translating from English... ({})", text);

            let mut output = String::new();
            for ch in text.chars() {
                let mapped = map_a.get(&ch).cloned().unwrap_or('☐');
                output.push(mapped);
            }

            println!("{}", output);

            let _ = ctx.set_contents(output);
            println!("✅ Copied");
        }
    }
}
