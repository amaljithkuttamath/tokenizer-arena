use crate::tokenize::TokenizerResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct ComparisonRow {
    pub model_name: String,
    pub encoding: String,
    pub token_count: usize,
    pub bytes_per_token: f64,
    pub tokens_per_word: f64,
}

pub fn compute_stats(result: &TokenizerResult, text: &str) -> ComparisonRow {
    let byte_len = text.len() as f64;
    let word_count = text.split_whitespace().count().max(1) as f64;
    let token_count = result.token_count.max(1) as f64;

    ComparisonRow {
        model_name: result.model_name.clone(),
        encoding: result.encoding_name.clone(),
        token_count: result.token_count,
        bytes_per_token: byte_len / token_count,
        tokens_per_word: result.token_count as f64 / word_count,
    }
}

#[derive(Serialize)]
pub struct JsonOutput {
    pub input_bytes: usize,
    pub input_words: usize,
    pub results: Vec<ComparisonRow>,
}
