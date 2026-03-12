use std::process::Command;

fn arena_cmd() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_tokenizer-arena"));
    // Disable color output for predictable parsing
    cmd.env("NO_COLOR", "1");
    cmd
}

fn fixture_path(name: &str) -> String {
    format!(
        "{}/tests/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

// ── Token count sanity checks ────────────────────────────────────────

#[test]
fn known_input_produces_expected_token_counts() {
    let output = arena_cmd()
        .arg("--json")
        .arg("Hello, world!")
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    let results = json["results"].as_array().unwrap();

    // Every encoding must produce at least 1 token
    for r in results {
        let count = r["token_count"].as_u64().unwrap();
        assert!(count >= 1, "token count should be >= 1, got {count}");
    }

    // cl100k_base should tokenize "Hello, world!" into 4 tokens
    let cl100k = results
        .iter()
        .find(|r| r["encoding"] == "cl100k_base")
        .unwrap();
    assert_eq!(cl100k["token_count"].as_u64().unwrap(), 4);
}

#[test]
fn each_encoding_present_in_output() {
    let output = arena_cmd()
        .arg("--json")
        .arg("test")
        .output()
        .expect("failed to run binary");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    let results = json["results"].as_array().unwrap();
    let encodings: Vec<&str> = results
        .iter()
        .map(|r| r["encoding"].as_str().unwrap())
        .collect();

    assert!(encodings.contains(&"cl100k_base"));
    assert!(encodings.contains(&"o200k_base"));
    assert!(encodings.contains(&"p50k_base"));
    assert!(encodings.contains(&"r50k_base"));
}

// ── Stats calculation ────────────────────────────────────────────────

#[test]
fn bytes_per_token_is_reasonable() {
    let input = "The quick brown fox jumps over the lazy dog.";
    let output = arena_cmd()
        .arg("--json")
        .arg(input)
        .output()
        .expect("failed to run binary");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    for r in json["results"].as_array().unwrap() {
        let bpt = r["bytes_per_token"].as_f64().unwrap();
        // Bytes per token should be between 1 and the full input length
        assert!(bpt >= 1.0, "bytes_per_token too low: {bpt}");
        assert!(
            bpt <= input.len() as f64,
            "bytes_per_token too high: {bpt}"
        );
    }
}

#[test]
fn tokens_per_word_is_reasonable() {
    let input = "The quick brown fox jumps over the lazy dog.";
    let output = arena_cmd()
        .arg("--json")
        .arg(input)
        .output()
        .expect("failed to run binary");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    for r in json["results"].as_array().unwrap() {
        let tpw = r["tokens_per_word"].as_f64().unwrap();
        // Tokens per word should be positive and less than 10
        assert!(tpw > 0.0, "tokens_per_word too low: {tpw}");
        assert!(tpw < 10.0, "tokens_per_word too high: {tpw}");
    }
}

// ── JSON output structure ────────────────────────────────────────────

#[test]
fn json_output_contains_expected_fields() {
    let output = arena_cmd()
        .arg("--json")
        .arg("hello")
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    // Top-level fields
    assert!(json["input_bytes"].is_u64());
    assert!(json["input_words"].is_u64());
    assert!(json["results"].is_array());

    // Each result row
    for r in json["results"].as_array().unwrap() {
        assert!(r["model_name"].is_string());
        assert!(r["encoding"].is_string());
        assert!(r["token_count"].is_u64());
        assert!(r["bytes_per_token"].is_f64());
        assert!(r["tokens_per_word"].is_f64());
    }
}

#[test]
fn json_input_metadata_is_correct() {
    let input = "one two three";
    let output = arena_cmd()
        .arg("--json")
        .arg(input)
        .output()
        .expect("failed to run binary");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    assert_eq!(json["input_bytes"].as_u64().unwrap(), 13);
    assert_eq!(json["input_words"].as_u64().unwrap(), 3);
}

// ── Code input: cl100k vs p50k differences ───────────────────────────

#[test]
fn code_input_shows_encoding_differences() {
    let code = "def fibonacci(n): return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2)";

    let output = arena_cmd()
        .arg("--json")
        .arg(code)
        .output()
        .expect("failed to run binary");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    let results = json["results"].as_array().unwrap();

    let cl100k_count = results
        .iter()
        .find(|r| r["encoding"] == "cl100k_base")
        .unwrap()["token_count"]
        .as_u64()
        .unwrap();

    let p50k_count = results
        .iter()
        .find(|r| r["encoding"] == "p50k_base")
        .unwrap()["token_count"]
        .as_u64()
        .unwrap();

    // cl100k should be more efficient than p50k for code
    assert!(
        cl100k_count <= p50k_count,
        "expected cl100k ({cl100k_count}) <= p50k ({p50k_count}) for code"
    );

    // They should actually differ for this input
    assert!(
        cl100k_count < p50k_count,
        "expected cl100k ({cl100k_count}) < p50k ({p50k_count}) for code input"
    );
}

// ── File input mode ──────────────────────────────────────────────────

#[test]
fn file_input_mode_works() {
    let path = fixture_path("sample_code.py");

    let output = arena_cmd()
        .arg("--json")
        .arg("--file")
        .arg(&path)
        .output()
        .expect("failed to run binary");

    assert!(
        output.status.success(),
        "file input failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    let results = json["results"].as_array().unwrap();
    assert_eq!(results.len(), 4);

    // File has real content, so token counts should be non-trivial
    for r in results {
        let count = r["token_count"].as_u64().unwrap();
        assert!(count > 10, "expected > 10 tokens for code file, got {count}");
    }
}

#[test]
fn file_input_multilingual() {
    let path = fixture_path("multilingual.txt");

    let output = arena_cmd()
        .arg("--json")
        .arg("--file")
        .arg(&path)
        .output()
        .expect("failed to run binary");

    assert!(
        output.status.success(),
        "multilingual file failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    // Multilingual text should produce more tokens than its word count
    let words = json["input_words"].as_u64().unwrap();
    let cl100k = json["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["encoding"] == "cl100k_base")
        .unwrap();
    let tokens = cl100k["token_count"].as_u64().unwrap();
    assert!(
        tokens > words,
        "expected tokens ({tokens}) > words ({words}) for multilingual text"
    );
}

// ── Stdin input mode ─────────────────────────────────────────────────

#[test]
fn stdin_input_mode_works() {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = arena_cmd()
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"Hello from stdin!")
        .expect("failed to write stdin");

    let output = child.wait_with_output().expect("failed to wait");

    assert!(
        output.status.success(),
        "stdin mode failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");

    assert_eq!(json["input_bytes"].as_u64().unwrap(), 17);
    assert!(json["results"].as_array().unwrap().len() == 4);
}

// ── Table output (non-JSON) ──────────────────────────────────────────

#[test]
fn table_output_contains_all_encodings() {
    let output = arena_cmd()
        .arg("Hello, world!")
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cl100k_base"), "missing cl100k_base in table");
    assert!(stdout.contains("o200k_base"), "missing o200k_base in table");
    assert!(stdout.contains("p50k_base"), "missing p50k_base in table");
    assert!(stdout.contains("r50k_base"), "missing r50k_base in table");
    assert!(stdout.contains("Input:"), "missing Input: summary");
}

// ── Error cases ──────────────────────────────────────────────────────

#[test]
fn missing_file_returns_error() {
    let output = arena_cmd()
        .arg("--file")
        .arg("/nonexistent/file.txt")
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());
}
