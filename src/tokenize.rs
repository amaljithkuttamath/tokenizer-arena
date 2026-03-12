use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, r50k_base, CoreBPE};

pub struct TokenizerResult {
    pub model_name: String,
    pub encoding_name: String,
    pub token_count: usize,
    pub tokens: Vec<String>,
}

pub struct Tokenizer {
    pub model_name: String,
    pub encoding_name: String,
    bpe: CoreBPE,
}

impl Tokenizer {
    pub fn tokenize(&self, text: &str) -> TokenizerResult {
        let token_ids = self.bpe.encode_with_special_tokens(text);
        let tokens: Vec<String> = token_ids
            .iter()
            .map(|&id| {
                self.bpe
                    .decode(vec![id])
                    .unwrap_or_else(|_| format!("<{id}>"))
            })
            .collect();

        TokenizerResult {
            model_name: self.model_name.clone(),
            encoding_name: self.encoding_name.clone(),
            token_count: token_ids.len(),
            tokens,
        }
    }
}

pub fn all_tokenizers() -> Vec<Tokenizer> {
    vec![
        Tokenizer {
            model_name: "GPT-4 / Claude".into(),
            encoding_name: "cl100k_base".into(),
            bpe: cl100k_base().expect("failed to load cl100k_base"),
        },
        Tokenizer {
            model_name: "GPT-4o".into(),
            encoding_name: "o200k_base".into(),
            bpe: o200k_base().expect("failed to load o200k_base"),
        },
        Tokenizer {
            model_name: "GPT-3 / Codex".into(),
            encoding_name: "p50k_base".into(),
            bpe: p50k_base().expect("failed to load p50k_base"),
        },
        Tokenizer {
            model_name: "GPT-3 (legacy)".into(),
            encoding_name: "r50k_base".into(),
            bpe: r50k_base().expect("failed to load r50k_base"),
        },
    ]
}
