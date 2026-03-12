#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$REPO_DIR/target/release/tokenizer-arena"
RESULTS_DIR="$SCRIPT_DIR/results"
INPUTS_DIR="$SCRIPT_DIR/inputs"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found at $BINARY. Building release..."
    (cd "$REPO_DIR" && cargo build --release)
fi

mkdir -p "$RESULTS_DIR"

run_benchmark() {
    local name="$1"
    local input_file="$2"
    local output_file="$RESULTS_DIR/${name}.md"

    echo "Running benchmark: $name"

    echo "# $name" > "$output_file"
    echo "" >> "$output_file"
    echo "Input file: \`$(basename "$input_file")\`" >> "$output_file"
    echo "" >> "$output_file"

    # Get file size info
    local bytes
    bytes=$(wc -c < "$input_file" | tr -d ' ')
    local words
    words=$(wc -w < "$input_file" | tr -d ' ')
    echo "Input: ${bytes} bytes, ${words} words" >> "$output_file"
    echo "" >> "$output_file"

    # Run with JSON output for structured data
    local json_output
    json_output=$("$BINARY" --json --file "$input_file")

    # Build markdown table from JSON
    echo "| Model | Encoding | Tokens | Bytes/Token | Tokens/Word |" >> "$output_file"
    echo "|-------|----------|--------|-------------|-------------|" >> "$output_file"

    echo "$json_output" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for r in data['results']:
    print(f\"| {r['model_name']} | {r['encoding']} | {r['token_count']} | {r['bytes_per_token']:.2f} | {r['tokens_per_word']:.2f} |\")
" >> "$output_file"

    echo "" >> "$output_file"
    echo "---" >> "$output_file"
    echo "" >> "$output_file"
}

# Run benchmarks for each input type
run_benchmark "english_prose" "$INPUTS_DIR/english_prose.txt"
run_benchmark "python_code" "$INPUTS_DIR/python_code.py"
run_benchmark "json_payload" "$INPUTS_DIR/json_payload.json"
run_benchmark "multilingual" "$INPUTS_DIR/multilingual.txt"

echo ""
echo "Benchmark results saved to $RESULTS_DIR/"
ls -la "$RESULTS_DIR/"
