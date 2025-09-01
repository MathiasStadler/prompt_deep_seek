//! Candle Stick Plotter - A program for visualizing financial data as candlestick charts
//! 
//! This program processes CSV data, converts input strings to uppercase,
//! and displays candlestick plots using egui/eframe.

use std::collections::HashMap;
use std::io;
use clap::Parser;
use anyhow::{Result, Context};
use thiserror::Error;

mod data_processor;
mod plotter;
mod utils;

use data_processor::DataProcessor;
use plotter::Plotter;
use utils::file_utils;

/// Command line arguments structure
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input string to convert to uppercase
    input_string: String,
    
    /// Path to CSV file (default: )
    #[arg(short, long, default_value = "HistoricalData_1756580762948.csv")]
    csv_file: String,
    
    /// Output directory for generated files
    #[arg(short, long, default_value = "output")]
    output_dir: String,
}

/// Custom error types for the application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Data processing error: {0}")]
    DataProcessing(String),
    
    #[error("Plotting error: {0}")]
    Plotting(String),
}

/// Main application entry point
///
/// # Returns
/// * `Result<()>` - Ok if successful, Err if any error occurs
///
/// # Examples
/// ```
/// // This would run the main function (not typically tested directly)
/// ```
fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    // Process input string and output in uppercase
    let uppercase_output = args.input_string.to_uppercase();
    println!("{}", uppercase_output);
    
    // Check if output directory exists and create if not
    file_utils::ensure_directory_exists(&args.output_dir)
        .context("Failed to create output directory")?;
    
    // Process CSV data
    let mut processor = DataProcessor::new();
    let data = processor.load_csv_data(&args.csv_file)
        .context("Failed to load CSV data")?;
    
    // Store data in HashMap for easy access
    let mut data_map = HashMap::new();
    data_map.insert("historical_data".to_string(), data);
    
    // Create and display plot
    let mut plotter = Plotter::new();
    plotter.create_candlestick_plot(&data_map, &args.output_dir)
        .context("Failed to create candlestick plot")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::str::contains;
    use tempfile::TempDir;
    
    /// Test the main function with valid input
    #[test]
    fn test_main_with_valid_input() -> Result<()> {
        let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
        cmd.arg("hello world")
           .assert()
           .success()
           .stdout(contains("HELLO WORLD"));
        Ok(())
    }
    
    /// Test the main function with empty input
    #[test]
    fn test_main_with_empty_input() -> Result<()> {
        let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
        cmd.arg("")
           .assert()
           .success()
           .stdout(contains(""));
        Ok(())
    }
    
    /// Test the main function with special characters
    #[test]
    fn test_main_with_special_chars() -> Result<()> {
        let mut cmd = Command::cargo_bin("candle_stick_plotter")?;
        cmd.arg("hello@world#123")
           .assert()
           .success()
           .stdout(contains("HELLO@WORLD#123"));
        Ok(())
    }
}
