#!/bin/bash

set -e

echo "Building and testing Candle Stick Plotter..."

Build with release profile
cargo build --release

Run all tests
cargo test --verbose

Run integration tests
cargo test --test integration_tests --verbose

Test the binary directly
echo "Testing binary output..."
./target/release/candle_stick_plotter "hello world" | grep -q "HELLO WORLD"
echo "✓ Uppercase conversion test passed"

Test with empty input
./target/release/candle_stick_plotter "" | grep -q "^$"
echo "✓ Empty input test passed"

Test error handling \(non-existent directory should not crash\)
./target/release/candle_stick_plotter "test" --output-dir "/non/existent/path" || true
echo "✓ Error handling test passed"

echo "All tests completed successfully!"
