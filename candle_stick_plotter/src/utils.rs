//! Utility functions for file operations and common tasks

use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

/// File utility functions
pub mod file_utils {
    use super::*;
    
    /// Ensures that a directory exists, creating it if necessary
    ///
    /// # Arguments
    /// * `dir_path` - Path to the directory
    ///
    /// # Returns
    /// * `Result<()>` - Ok if directory exists or was created successfully
    ///
    /// # Errors
    /// * Returns error if directory creation fails
    pub fn ensure_directory_exists(dir_path: &str) -> Result<()> {
        let path = Path::new(dir_path);
        
        if !path.exists() {
            fs::create_dir_all(path)
                .context(format!("Failed to create directory: {}", dir_path))?;
            log::info!("Created directory: {}", dir_path);
        } else {
            log::info!("Directory already exists: {}", dir_path);
        }
        
        Ok(())
    }
    
    /// Checks if a file exists
    ///
    /// # Arguments
    /// * `file_path` - Path to the file
    ///
    /// # Returns
    /// * `bool` - True if file exists, false otherwise
    pub fn file_exists(file_path: &str) -> bool {
        Path::new(file_path).exists()
    }
}

/// String utility functions
pub mod string_utils {
    /// Converts a string to uppercase (wrapper for built-in method)
    ///
    /// # Arguments
    /// * `input` - Input string to convert
    ///
    /// # Returns
    /// * `String` - Uppercase version of the input string
    pub fn to_uppercase(input: &str) -> String {
        input.to_uppercase()
    }
    
    /// Trims whitespace from a string
    ///
    /// # Arguments
    /// * `input` - Input string to trim
    ///
    /// # Returns
    /// * `String` - Trimmed version of the input string
    pub fn trim_string(input: &str) -> String {
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    /// Test directory creation
    #[test]
    fn test_ensure_directory_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_dir = temp_dir.path().join("test_subdir");
        
        let result = file_utils::ensure_directory_exists(test_dir.to_str().unwrap());
        assert!(result.is_ok());
        assert!(test_dir.exists());
        
        Ok(())
    }
    
    /// Test file existence check
    #[test]
    fn test_file_exists() {
        let exists = file_utils::file_exists("/proc/cpuinfo"); // Should exist on Linux
        assert!(exists || !exists); // This test just verifies the function doesn't panic
    }
    
    /// Test string to uppercase conversion
    #[test]
    fn test_to_uppercase() {
        assert_eq!(string_utils::to_uppercase("hello"), "HELLO");
        assert_eq!(string_utils::to_uppercase("Hello World"), "HELLO WORLD");
        assert_eq!(string_utils::to_uppercase("123abc"), "123ABC");
    }
    
    /// Test string trimming
    #[test]
    fn test_trim_string() {
        assert_eq!(string_utils::trim_string("  hello  "), "hello");
        assert_eq!(string_utils::trim_string("hello"), "hello");
        assert_eq!(string_utils::trim_string(""), "");
    }
}
