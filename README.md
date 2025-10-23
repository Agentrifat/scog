# scog (Shell COmpletion Generator) ⚙️

scog aims to generate shell completions for bash/zsh/fish, in a language-agnostic manner.

## Quick start

1. **Create a spec file**:

Suppose that the YAML above defines the interface for the `myapp` binary (more details on how this should look like later):

```yaml
name: myapp
about: A fictional CLI tool for managing projects

args:
  - name: verbose
    short: v
    long: verbose
    help: Enable verbose output
    takes_value: false    
```

2. **Generate completions**:
```bash
scog myapp.yaml
```

You should see some helpful output such as:
<img width="794" height="653" alt="image" src="https://github.com/user-attachments/assets/0011712d-c8de-4d6c-b0c3-8cef4e56f23b" />

3. **Verify and install**

Validate if the generated completions look good, give them a try!

## Spec format

The YAML spec defines your CLI's interface. Here's a complete example:

```yaml
name: myapp
about: A fictional CLI tool for managing projects
version: 1.0.0

args:
  - name: config
    short: c
    long: config
    help: Path to configuration file
    required: false
    value_hint: FilePath
  
  - name: verbose
    short: v
    long: verbose
    help: Enable verbose output
    required: false
    takes_value: false  # This is a flag, not an option
  
  - name: output
    short: o
    long: output
    help: Output directory
    required: false
    value_hint: DirPath
  
  - name: format
    short: f
    long: format
    help: Output format
    required: false
    possible_values:
      - json
      - yaml
      - toml

subcommands:
  - name: init
    about: Initialize a new project
    args:
      - name: name
        help: Project name
        required: true
      
      - name: template
        long: template
        help: Project template to use
        possible_values:
          - basic
          - advanced
          - minimal
  
  - name: build
    about: Build the project
    args:
      - name: release
        long: release
        help: Build in release mode
        takes_value: false
      
      - name: target
        long: target
        help: Build target
        value_hint: Other
```

### Reference

#### Top-level fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Name of your binary |
| `about` | string | No | Short description of what it does |
| `version` | string |No | Version string |
| `args` | array | No | List of arguments/options |
| `subcommands` | array | No | List of subcommands |

#### Argument fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✓ | Argument identifier |
| `short` | char | | Short flag (e.g., `v` for `-v`) |
| `long` | string | | Long flag (e.g., `verbose` for `--verbose`) |
| `help` | string | | Help text for the argument |
| `required` | bool | | Whether the argument is required (default: `false`) |
| `takes_value` | bool | | Whether argument takes a value. `false` makes it a boolean flag (default: `true`) |
| `value_hint` | string | | Completion hint (see [this] for reference) |
| `possible_values` | array | | List of allowed values for constrained options |

### Subcommands

Subcommands follow the same structure as the top-level command:

```yaml
subcommands:
  - name: init
    about: Initialize something
    args:
      - name: force
        short: f
        long: force
        help: Force initialization
        takes_value: false
```

Subcommands can be nested in another subcommand.

## Installation
```bash
cargo install scog
```

Or build from source:
```bash
git clone https://github.com/vrmiguel/scog
cd scog
cargo build --release
```

## Usage
```bash

```

## How it works

scog is merely some glue to use [clap](https://github.com/clap-rs/clap)'s own completion generation, but for utilities that are _not_ built using Clap, or even Rust itself. Your spec is parsed and converted into a `clap::Command` structure at runtime, then clap's proven completion generators produce completions.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgments

Built with [clap](https://github.com/clap-rs/clap) and [clap_complete](https://crates.io/crates/clap-complete).