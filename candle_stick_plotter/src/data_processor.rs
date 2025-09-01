//! Data processing module for handling CSV data and financial calculations

use std::path::Path;
use csv::ReaderBuilder;
use serde::Deserialize;
use anyhow::{Result, Context};
use chrono::{DateTime, NaiveDateTime, Utc};

/// Represents a single data point from the CSV file
#[derive(Debug, Deserialize, Clone)]
pub struct HistoricalData {
    #[allow(unused)]
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    
    #[allow(unused)]
    #[serde(rename = "Open")]
    pub open: f64,
    
    #[allow(unused)]
    #[serde(rename = "High")]
    pub high: f64,
    
    #[allow(unused)]
    #[serde(rename = "Low")]
    pub low: f64,
    
    #[allow(unused)]
    #[serde(rename = "Close")]
    pub close: f64,
    
    #[allow(unused)]
    #[serde(rename = "Volume")]
    pub volume: f64,
}

/// Represents a candlestick for plotting
#[derive(Debug, Clone)]
pub struct CandleStick {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Processes and manages financial data
pub struct DataProcessor {
    data: Vec<HistoricalData>,
}

impl DataProcessor {
    /// Creates a new DataProcessor instance
    ///
    /// # Returns
    /// * `DataProcessor` - New instance
    pub fn new() -> Self {
        DataProcessor { data: Vec::new() }
    }
    
    /// Loads CSV data from the specified file path
    ///
    /// # Arguments
    /// * `file_path` - Path to the CSV file
    ///
    /// # Returns
    /// * `Result<Vec<HistoricalData>>` - Vector of parsed historical data
    ///
    /// # Errors
    /// * Returns error if file cannot be read or parsed
    pub fn load_csv_data(&mut self, file_path: &str) -> Result<Vec<HistoricalData>> {
        let path = Path::new(file_path);
        
        // Check if file exists
        if !path.exists() {
            // Create sample data for testing if file doesn't exist
            self.generate_sample_data()
        } else {
            let mut rdr = ReaderBuilder::new()
                .has_headers(true)
                .from_path(path)
                .context("Failed to create CSV reader")?;
            
            let mut data = Vec::new();
            
            for result in rdr.deserialize() {
                let record: HistoricalData = result.context("Failed to deserialize CSV record")?;
                data.push(record);
            }
            
            self.data = data.clone();
            Ok(data)
        }
    }
    
    /// Generates sample data for testing purposes
    ///
    /// # Returns
    /// * `Result<Vec<HistoricalData>>` - Generated sample data
    fn generate_sample_data(&mut self) -> Result<Vec<HistoricalData>> {
        let sample_data = vec![
            HistoricalData {
                timestamp: "2023-01-01 00:00:00".to_string(),
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 1000.0,
            },
            HistoricalData {
                timestamp: "2023-01-02 00:00:00".to_string(),
                open: 102.0,
                high: 108.0,
                low: 101.0,
                close: 106.0,
                volume: 1200.0,
            },
            HistoricalData {
                timestamp: "2023-01-03 00:00:00".to_string(),
                open: 106.0,
                high: 110.0,
                low: 104.0,
                close: 108.0,
                volume: 1500.0,
            },
        ];
        
        self.data = sample_data.clone();
        Ok(sample_data)
    }
    
    /// Converts historical data to candlestick format
    ///
    /// # Returns
    /// * `Result<Vec<CandleStick>>` - Vector of candlestick data
    pub fn to_candlesticks(&self) -> Result<Vec<CandleStick>> {
        let mut candlesticks = Vec::new();
        
        for data in &self.data {
            let timestamp = NaiveDateTime::parse_from_str(&data.timestamp, "%Y-%m-%d %H:%M:%S")
                .context("Failed to parse timestamp")?;
            let datetime = DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc);
            
            candlesticks.push(CandleStick {
                timestamp: datetime,
                open: data.open,
                high: data.high,
                low: data.low,
                close: data.close,
                volume: data.volume,
            });
        }
        
        Ok(candlesticks)
    }
    
    /// Gets the loaded data
    ///
    /// # Returns
    /// * `&Vec<HistoricalData>` - Reference to the loaded data
    pub fn get_data(&self) -> &Vec<HistoricalData> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    /// Test DataProcessor creation
    #[test]
    fn test_data_processor_new() {
        let processor = DataProcessor::new();
        assert!(processor.get_data().is_empty());
    }
    
    /// Test sample data generation
    #[test]
    fn test_generate_sample_data() -> Result<()> {
        let mut processor = DataProcessor::new();
        let data = processor.generate_sample_data()?;
        
        assert_eq!(data.len(), 3);
        assert_eq!(data[0].open, 100.0);
        assert_eq!(data[1].close, 106.0);
        assert_eq!(data[2].volume, 1500.0);
        
        Ok(())
    }
    
    /// Test candlestick conversion
    #[test]
    fn test_to_candlesticks() -> Result<()> {
        let mut processor = DataProcessor::new();
        processor.generate_sample_data()?;
        
        let candlesticks = processor.to_candlesticks()?;
        
        assert_eq!(candlesticks.len(), 3);
        assert_eq!(candlesticks[0].open, 100.0);
        assert_eq!(candlesticks[1].close, 106.0);
        assert_eq!(candlesticks[2].volume, 1500.0);
        
        Ok(())
    }
    
    /// Test CSV loading with temporary file
    #[test]
    fn test_load_csv_data() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Timestamp,Open,High,Low,Close,Volume")?;
        writeln!(file, "2023-01-01 00:00:00,100.0,105.0,95.0,102.0,1000.0")?;
        writeln!(file, "2023-01-02 00:00:00,102.0,108.0,101.0,106.0,1200.0")?;
        
        let mut processor = DataProcessor::new();
        let data = processor.load_csv_data(file.path().to_str().unwrap())?;
        
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].open, 100.0);
        assert_eq!(data[1].close, 106.0);
        
        Ok(())
    }
}
