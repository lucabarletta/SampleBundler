# SampleBundler

A command-line tool to organize audio samples based on predefined patterns to a 
specific destination folder (for exporting to Digitakt for example) 

## Commands

### `tree`

This command displays the directory structure of a given path.

**Usage:**
```bash
cargo run -- tree --source <path/to/directory>
```

**Arguments:**
- `--source` or `-s`: The directory to display.
- `--folders-only`: An optional flag to only display folders in the tree structure. [optional]
- `--run-discover`: Add an optional flag to show filename patterns and their counts. [optional]

**Example:**
```bash
cargo run -- tree --source ./samples --folders-only
```


### `organize`

This command categorizes and copies `.wav` files from a source directory to a destination directory based on regex patterns defined in a `config.toml` file.

**Usage:**
```bash
cargo run -- organize --source <path/to/source> --dest <path/to/destination> --config <path/to/config.toml>
```

**Arguments:**
- `--source` or `-s`: The source directory containing the `.wav` files.
- `--dest` or `-d`: The destination directory where the organized files will be copied.
- `--config` or `-c`: The path to the configuration file (defaults to `config.toml`).

