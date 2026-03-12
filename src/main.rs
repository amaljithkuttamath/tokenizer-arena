mod compare;
mod display;
mod tokenize;

use clap::Parser;
use compare::{compute_stats, JsonOutput};
use std::io::{self, Read};

#[derive(Parser)]
#[command(
    name = "tokenizer-arena",
    about = "Compare how different LLM tokenizers handle the same input text",
    version
)]
struct Cli {
    /// Input text to tokenize
    text: Option<String>,

    /// Read input from a file
    #[arg(short, long)]
    file: Option<String>,

    /// Show token boundaries with color coding
    #[arg(short, long)]
    show_tokens: bool,

    /// Output results as JSON
    #[arg(short, long)]
    json: bool,
}

fn get_input(cli: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(ref path) = cli.file {
        Ok(std::fs::read_to_string(path)?)
    } else if let Some(ref text) = cli.text {
        Ok(text.clone())
    } else if atty::is(atty::Stream::Stdin) {
        Err("No input provided. Pass text as an argument, use --file, or pipe via stdin.".into())
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    }
}

fn main() {
    let cli = Cli::parse();

    let text = match get_input(&cli) {
        Ok(t) if t.is_empty() => {
            eprintln!("Error: input is empty.");
            std::process::exit(1);
        }
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let tokenizers = tokenize::all_tokenizers();
    let results: Vec<_> = tokenizers.iter().map(|t| t.tokenize(&text)).collect();
    let rows: Vec<_> = results.iter().map(|r| compute_stats(r, &text)).collect();

    if cli.json {
        let output = JsonOutput {
            input_bytes: text.len(),
            input_words: text.split_whitespace().count(),
            results: rows,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&output).expect("failed to serialize JSON")
        );
        return;
    }

    display::print_input_summary(&text);
    display::print_table(&rows);

    if cli.show_tokens {
        display::print_tokens(&results);
    }
}
