// tests/bot_detector_tests.rs

use my_bot_checker::{is_bot, is_bot_match};
use std::fs;
use tempfile::NamedTempFile;

fn create_temp_patterns_file(patterns: &[&str]) -> NamedTempFile {
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let pattern_list = serde_json::to_string(&patterns).expect("Failed to serialize patterns");

    fs::write(file.path(), pattern_list).expect("Failed to write to temp file");
    file
}

#[test]
fn test_is_bot_integration() {
    let bot_user_agent = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    let temp_file = create_temp_patterns_file(&["Googlebot"]);

    assert!(is_bot(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap());
}

#[test]
fn test_is_bot_match_integration() {
    let bot_user_agent = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    let temp_file = create_temp_patterns_file(&["Googlebot"]);

    assert_eq!(
        is_bot_match(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap(),
        Some("Googlebot".to_string())
    );
}

#[test]
fn test_invalid_inputs_integration() {
    let temp_file = create_temp_patterns_file(&["Googlebot"]);

    assert!(!is_bot("", temp_file.path().to_str().unwrap()).unwrap());
    assert_eq!(
        is_bot_match("", temp_file.path().to_str().unwrap()).unwrap(),
        None
    );
}
