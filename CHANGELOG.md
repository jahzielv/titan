# Roadmap
- Error handling
    - [ ] `unwrap`s have to be replaced with appropriate error handling; this is a server, so we can't be panicking when something trivial breaks!
- TLS config
    - [ ] Add section to Titan.toml for TLS config (`pfx` file location, environment variable key for password)
- Documentation
    - [ ] Comments and examples on all functions
    - [ ] Instructions for setting up self-signed TLS
- Perf improvements
    - Will Tokio be better than a multithreaded approach?
- Distribution
    - [ ] Packaging binary and framework separately on crates.io
    - [ ] Maybe binary distribution through GH releases?