# MCP960x async

This is an async driver for the MCP960x digital thermocouple converter. The I2C communication is based on `embassy-rs` async traits, which are not yet on crates.io

Currently rust nightly is needed for the feature [inherent associated types](<https://github.com/rust-lang/rust/issues/8995>)
