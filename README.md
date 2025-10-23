# scog (Shell COmpletion Generator) ⚙️

scog aims to generate shell completions for bash/zsh/fish given a spec that defines your CLI's usage.

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