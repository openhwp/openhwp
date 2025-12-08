//! Integration tests using real HWP document samples.
//!
//! These tests use sample files from the `fixtures/` directory.
//! The fixtures directory is not committed to version control due to
//! licensing concerns with the sample documents.
//!
//! Tests are designed to:
//! - Dynamically discover all .hwp files in the fixtures directory
//! - Skip gracefully if fixture files are not present
//! - Automatically handle large files (50MB+) separately
//!
//! Run tests:
//! - Normal: `cargo test --test fixtures_test -- --nocapture`
//! - With large files: `cargo test --test fixtures_test -- --ignored --nocapture`

use std::collections::HashSet;
use std::path::PathBuf;

use hwp::HwpDocument;

/// Size threshold for considering a file "large" (50MB).
const LARGE_FILE_THRESHOLD: u64 = 50 * 1024 * 1024;

/// Returns the path to a fixture file if it exists.
fn get_fixture_path(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name);
    if path.exists() { Some(path) } else { None }
}

/// Discovers all .hwp files in the fixtures directory.
/// Returns an empty Vec if the directory doesn't exist.
fn discover_fixtures() -> Vec<PathBuf> {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");

    if !fixtures_dir.exists() {
        return Vec::new();
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(&fixtures_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map(|ext| ext.eq_ignore_ascii_case("hwp"))
                .unwrap_or(false)
        })
        .collect();

    // Sort for deterministic ordering
    files.sort();
    files
}

/// Test result for a fixture file.
#[derive(Debug)]
enum FixtureTestResult {
    /// Parsing succeeded.
    Success {
        version: String,
        sections: usize,
        paragraphs: usize,
        text_len: usize,
    },
    /// Parsing failed with error.
    ParseError(String),
}

/// Configuration for files that need special handling.
struct FixtureConfig {
    /// Files that should be skipped entirely (known broken, not HWP 5.0, etc.)
    skip_files: HashSet<String>,
    /// Files that are expected to fail (known parser limitations)
    expected_failures: HashSet<String>,
}

impl Default for FixtureConfig {
    fn default() -> Self {
        Self {
            skip_files: HashSet::new(),
            expected_failures: HashSet::new(),
        }
    }
}

/// Detailed test results for reporting.
struct TestResults {
    success: Vec<(String, FixtureTestResult)>,
    failed: Vec<(String, FixtureTestResult)>,
    skipped: Vec<(String, String)>, // (filename, reason)
    expected_failures: Vec<(String, FixtureTestResult)>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            success: Vec::new(),
            failed: Vec::new(),
            skipped: Vec::new(),
            expected_failures: Vec::new(),
        }
    }

    fn total(&self) -> usize {
        self.success.len() + self.failed.len() + self.skipped.len() + self.expected_failures.len()
    }

    fn success_rate(&self) -> f64 {
        let testable = self.success.len() + self.failed.len();
        if testable == 0 {
            100.0
        } else {
            self.success.len() as f64 / testable as f64 * 100.0
        }
    }
}

/// Tests a fixture file and returns the result.
fn test_fixture(path: &PathBuf) -> FixtureTestResult {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => return FixtureTestResult::ParseError(format!("IO error: {}", e)),
    };

    match HwpDocument::from_bytes(&data) {
        Ok(doc) => {
            let version = doc.version();
            let sections = doc.section_count();
            let paragraphs = doc.paragraph_count();
            let text = doc.extract_text();

            FixtureTestResult::Success {
                version: version.to_string(),
                sections,
                paragraphs,
                text_len: text.len(),
            }
        }
        Err(e) => FixtureTestResult::ParseError(format!("{:?}", e)),
    }
}

/// Prints the test summary.
fn print_test_summary(results: &TestResults) {
    eprintln!("\n=== Test Summary ===");
    eprintln!("Success:           {}", results.success.len());
    eprintln!("Failed:            {}", results.failed.len());
    eprintln!("Expected Failures: {}", results.expected_failures.len());
    eprintln!("Skipped:           {}", results.skipped.len());
    eprintln!("Total:             {}", results.total());
    eprintln!("Success Rate:      {:.1}%", results.success_rate());

    if !results.failed.is_empty() {
        eprintln!("\n--- Unexpected Failures ---");
        for (name, result) in &results.failed {
            if let FixtureTestResult::ParseError(e) = result {
                eprintln!("  {}: {}", name, e);
            }
        }
    }

    if !results.skipped.is_empty() {
        eprintln!("\n--- Skipped Files ---");
        for (name, reason) in &results.skipped {
            eprintln!("  {}: {}", name, reason);
        }
    }
}

/// Verifies the test infrastructure is working correctly.
#[test]
fn test_infrastructure() {
    // Test path helper
    assert!(get_fixture_path("nonexistent.hwp").is_none());

    // Test discovery function
    let fixtures = discover_fixtures();
    eprintln!("Discovered {} fixtures", fixtures.len());

    // Verify all discovered files have .hwp extension
    for path in &fixtures {
        assert!(
            path.extension()
                .map(|e| e.eq_ignore_ascii_case("hwp"))
                .unwrap_or(false),
            "Non-HWP file discovered: {:?}",
            path
        );
    }

    // Verify fixtures are sorted
    let is_sorted = fixtures.windows(2).all(|w| w[0] <= w[1]);
    assert!(is_sorted, "Fixtures should be sorted");
}

/// Dynamically discovers and tests all HWP fixtures.
///
/// This test:
/// - Automatically finds all .hwp files in fixtures/
/// - Tests each file without requiring code changes when files are added/removed
/// - Reports comprehensive statistics
/// - Fails only on unexpected failures
#[test]
fn test_all_fixtures() {
    let config = FixtureConfig::default();
    let fixtures = discover_fixtures();
    let mut results = TestResults::new();

    if fixtures.is_empty() {
        eprintln!("\n=== Fixture Test Results ===");
        eprintln!("No fixtures found in fixtures/ directory.");
        eprintln!("This is OK - fixtures may be missing due to licensing.");
        return;
    }

    eprintln!("\n=== Testing {} HWP Fixtures ===\n", fixtures.len());

    for path in &fixtures {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Check if file should be skipped
        if config.skip_files.contains(file_name) {
            results
                .skipped
                .push((file_name.to_string(), "In skip list".to_string()));
            eprintln!("[SKIP] {}: in skip list", file_name);
            continue;
        }

        // Check file size for large files
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > LARGE_FILE_THRESHOLD {
                results.skipped.push((
                    file_name.to_string(),
                    format!(
                        "Large file ({}MB) - run with --ignored",
                        metadata.len() / 1024 / 1024
                    ),
                ));
                eprintln!("[SKIP] {}: large file, run with --ignored", file_name);
                continue;
            }
        }

        // Run the test
        let result = test_fixture(path);
        let is_expected_failure = config.expected_failures.contains(file_name);

        match &result {
            FixtureTestResult::Success {
                version,
                sections,
                paragraphs,
                text_len,
            } => {
                eprintln!(
                    "[OK]   {}: v={}, sections={}, paragraphs={}, text_len={}",
                    file_name, version, sections, paragraphs, text_len
                );
                results.success.push((file_name.to_string(), result));
            }
            FixtureTestResult::ParseError(e) => {
                if is_expected_failure {
                    eprintln!("[XFAIL] {}: {} (expected)", file_name, e);
                    results
                        .expected_failures
                        .push((file_name.to_string(), result));
                } else {
                    eprintln!("[FAIL] {}: {}", file_name, e);
                    results.failed.push((file_name.to_string(), result));
                }
            }
        }
    }

    // Print summary
    print_test_summary(&results);

    // Fail the test if there are unexpected failures
    if !results.failed.is_empty() {
        panic!(
            "Test failed: {} unexpected failures out of {} fixtures",
            results.failed.len(),
            results.total()
        );
    }
}

/// Tests large fixture files (run with: cargo test --test fixtures_test -- --ignored)
#[test]
#[ignore]
fn test_large_fixtures() {
    let fixtures = discover_fixtures();

    eprintln!("\n=== Testing Large Fixtures ===\n");

    let large_fixtures: Vec<_> = fixtures
        .iter()
        .filter(|path| {
            std::fs::metadata(path)
                .map(|m| m.len() > LARGE_FILE_THRESHOLD)
                .unwrap_or(false)
        })
        .collect();

    if large_fixtures.is_empty() {
        eprintln!("No large fixtures found.");
        return;
    }

    for path in large_fixtures {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        eprintln!("Testing {}...", file_name);
        let start = std::time::Instant::now();
        let result = test_fixture(path);
        let elapsed = start.elapsed();

        match result {
            FixtureTestResult::Success {
                version,
                sections,
                paragraphs,
                text_len,
            } => {
                eprintln!(
                    "[OK]   {} ({:.2}s): v={}, sections={}, paragraphs={}, text_len={}",
                    file_name,
                    elapsed.as_secs_f64(),
                    version,
                    sections,
                    paragraphs,
                    text_len
                );
            }
            FixtureTestResult::ParseError(e) => {
                eprintln!(
                    "[FAIL] {} ({:.2}s): {}",
                    file_name,
                    elapsed.as_secs_f64(),
                    e
                );
            }
        }
    }
}
