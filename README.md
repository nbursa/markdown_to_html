# Markdown to HTML Converter

This is a simple command-line tool that converts Markdown files to HTML using Rust.

## Features

- Converts Markdown files to HTML.
- Command-line interface for easy usage.
- Utilizes the `pulldown-cmark` crate for Markdown parsing.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/nbursa/markdown_to_html.git
   cd markdown_to_html
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

## Usage

To convert a Markdown file to HTML, run the following command:

```bash
./target/release/markdown_to_html <input.md> <output.html>
```

Replace `<input.md>` with the path to your Markdown file and `<output.html>` with the desired path for the output HTML file.

## Example

```bash
./target/release/markdown_to_html example.md example.html
```

This will convert the contents of `example.md` to HTML and save it to `example.html`.

## Dependencies

- [clap](https://crates.io/crates/clap) for command-line argument parsing.
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) for Markdown parsing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
