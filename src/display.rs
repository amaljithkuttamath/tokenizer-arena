use crate::compare::ComparisonRow;
use crate::tokenize::TokenizerResult;
use colored::Colorize;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};

const COLORS: &[&str] = &["red", "green", "blue", "yellow", "magenta", "cyan"];

pub fn print_table(rows: &[ComparisonRow]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Model",
            "Encoding",
            "Tokens",
            "Bytes/Token",
            "Tokens/Word",
        ]);

    for row in rows {
        table.add_row(vec![
            row.model_name.clone(),
            row.encoding.clone(),
            row.token_count.to_string(),
            format!("{:.2}", row.bytes_per_token),
            format!("{:.2}", row.tokens_per_word),
        ]);
    }

    println!("{table}");
}

pub fn print_tokens(results: &[TokenizerResult]) {
    println!(
        "\n{}",
        "Token Boundaries".bold().underline()
    );

    for (i, result) in results.iter().enumerate() {
        let color = COLORS[i % COLORS.len()];
        println!(
            "\n  {} ({})",
            result.model_name.bold(),
            result.encoding_name
        );
        print!("  ");

        for (j, token) in result.tokens.iter().enumerate() {
            let display = token.replace('\n', "\\n").replace('\t', "\\t");
            let styled = if j % 2 == 0 {
                match color {
                    "red" => display.on_red().white().to_string(),
                    "green" => display.on_green().black().to_string(),
                    "blue" => display.on_blue().white().to_string(),
                    "yellow" => display.on_yellow().black().to_string(),
                    "magenta" => display.on_magenta().white().to_string(),
                    _ => display.on_cyan().black().to_string(),
                }
            } else {
                display.normal().to_string()
            };
            print!("{styled}");
        }
        println!();
    }
    println!();
}

pub fn print_input_summary(text: &str) {
    let bytes = text.len();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    println!(
        "\n{} {} bytes, {} chars, {} words\n",
        "Input:".bold(),
        bytes,
        chars,
        words
    );
}
