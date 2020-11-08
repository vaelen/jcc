JCC Tool
========

This tool downloads the current JCC / JCG / Ku data from the JARL website and outputs a CSV file containing all of the primary and secondary administrative regions for use in amateur radio applications.

The easiest way to run this application is like this:
```
cargo run --release
```

You can also build the application without running it like this:
```
cargo build --release
```

This places the binary at `target/release/jcc`.
