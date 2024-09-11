# pafciggy

pafciggy is a Rust-based command-line tool designed to fix and validate PAF (Pairwise Alignment Format) files, specifically focusing on correcting endpoint coordinates based on CIGAR strings.

## Features

- Reads PAF files from stdin or a specified input file
- Fixes query and target endpoint coordinates based on CIGAR string calculations
- Provides statistics on fixed records, including:
  - Total number of fixed records
  - Number of query fixes
  - Number of target fixes
  - Average query displacement
  - Average target displacement
- Handles potential errors in input files

## Usage

To use pafciggy, run the following command:

```
cargo run -- <input_file or '-' for stdin>
```

Examples:

1. Process a PAF file:
   ```
   cargo run -- input_file.paf
   ```

2. Process input from stdin:
   ```
   cat input_file.paf | cargo run -- -
   ```

## Output

pafciggy will output the corrected PAF records to stdout. Additionally, it will print statistics to stderr, including:

- Number of fixed records
- Number of query fixes
- Number of target fixes
- Average query displacement
- Average target displacement
- Number of records with errors (if any)

## Development

To run tests:

```
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Just make a PR!

## Author

Erik Garrison
