# tokenizer-arena

Compare how different LLM tokenizers handle the same input text.

Understanding tokenization is fundamental to working with LLMs. Different models use different tokenizers, which means the same text costs different amounts of tokens depending on which model you use. `tokenizer-arena` lets you see exactly how each tokenizer breaks down your text, side by side.

## Example Output

```
$ tokenizer-arena "def fibonacci(n): return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2)"

Input: 73 bytes, 73 chars, 12 words

╭────────────────┬─────────────┬────────┬─────────────┬─────────────╮
│ Model          ┆ Encoding    ┆ Tokens ┆ Bytes/Token ┆ Tokens/Word │
╞════════════════╪═════════════╪════════╪═════════════╪═════════════╡
│ GPT-4 / Claude ┆ cl100k_base ┆ 23     ┆ 3.17        ┆ 1.92        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ GPT-4o         ┆ o200k_base  ┆ 23     ┆ 3.17        ┆ 1.92        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ GPT-3 / Codex  ┆ p50k_base   ┆ 31     ┆ 2.35        ┆ 2.58        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ GPT-3 (legacy) ┆ r50k_base   ┆ 31     ┆ 2.35        ┆ 2.58        │
╰────────────────┴─────────────┴────────┴─────────────┴─────────────╯
```

With `--show-tokens`, you can see exactly where each tokenizer splits the text, with color-coded boundaries in your terminal.

## Install

### From source

```bash
cargo install --git https://github.com/amaljithkuttamath/tokenizer-arena
```

### Pre-built binaries

Download from [Releases](https://github.com/amaljithkuttamath/tokenizer-arena/releases) for Linux, macOS, and Windows.

## Usage

```bash
# Tokenize a string
tokenizer-arena "Your text here"

# Read from a file
tokenizer-arena --file input.txt

# Pipe from stdin
cat prompt.txt | tokenizer-arena

# See token boundaries (color-coded in terminal)
tokenizer-arena --show-tokens "Hello, world!"

# JSON output for scripting
tokenizer-arena --json "Hello, world!"
```

## Tokenizers Included

| Model | Encoding | Vocabulary |
|-------|----------|------------|
| GPT-4, Claude | cl100k_base | 100K tokens |
| GPT-4o | o200k_base | 200K tokens |
| GPT-3, Codex | p50k_base | 50K tokens |
| GPT-3 (legacy) | r50k_base | 50K tokens |

## Why This Exists

Token counts directly affect cost, latency, and context window usage when working with LLMs. Different tokenizers handle the same text very differently, especially for code, multilingual text, and structured data. This tool makes those differences visible so you can make informed decisions about which model to use for your workload.

## License

MIT
