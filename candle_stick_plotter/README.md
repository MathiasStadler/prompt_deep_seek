# Candle Stick Plotter

A Rust application for processing financial data and creating candlestick charts.

## Features

- CSV data processing
- Uppercase string conversion
- Candlestick chart visualization
- Comprehensive test coverage
- Error handling

## Installation

```bash
cargo build --release


./target/release/candle_stick_plotter "input string" [--csv-file FILE] [--output-dir DIR]

cargo test --verbose
cargo llvm-cov --html
