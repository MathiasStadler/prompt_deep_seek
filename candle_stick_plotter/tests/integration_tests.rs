//! Integration tests for the candle stick plotter application

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;
// use std::fs;

/// Test the complete application workflow
#[test]
fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let output_dir = temp_dir.path().join("output");
    
    let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
    cmd.arg("test input")
       .arg("--output-dir")
       .arg(output_dir.to_str().unwrap())
       .assert()
       .success()
       .stdout(contains("TEST INPUT"));
    
    // Verify output directory was created
    assert!(output_dir.exists());
    
    Ok(())
}

/// Test with different input variations
#[test]
fn test_various_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("hello", "HELLO"),
        ("WORLD", "WORLD"),
        ("mixedCase", "MIXEDCASE"),
        ("123 numbers", "123 NUMBERS"),
        ("special!@#", "SPECIAL!@#"),
        ("", ""),
    ];
    
    for (input, expected) in test_cases {
        let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
        cmd.arg(input)
           .assert()
           .success()
           .stdout(contains(expected));
    }
    
    Ok(())
}

/// Test with custom CSV file (non-existent, should use sample data)
#[test]
fn test_with_custom_csv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
    cmd.arg("test")
       .arg("--csv-file")
       .arg("non_existent_file.csv")
       .assert()
       .success();
    
    Ok(())
}
