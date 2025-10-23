# scog (Shell COmpletion Generator) ⚙️

scog aims to generate shell completions for bash/zsh/fish, in a language-agnostic manner.

![demo](https://github.com/user-attachments/assets/548cf1f5-2269-46c4-ae61-b07bf89ae922)

_⚠️ this software is in its infancy, the spec is likely incomplete and subject to change_

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
[...]
```

2. **Generate completions**:
```bash
scog myapp.yaml
```

You should see some helpful output such as:
<img width="794" height="653" alt="image" src="https://github.com/user-attachments/assets/0011712d-c8de-4d6c-b0c3-8cef4e56f23b" />

3. **Verify and install**

Validate if the generated completions look good, then give them a try!

## Spec format

The YAML spec defines your CLI's interface. See [the Example](#example) for a complete sample.

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
| `name` | string | yes | Argument identifier |
| `short` | char | no | Short flag (e.g., `v` for `-v`) |
| `long` | string | no| Long flag (e.g., `verbose` for `--verbose`) |
| `help` | string | no| Help text for the argument |
| `required` | bool | no| Whether the argument is required (default: `false`) |
| `takes_value` | bool |no | Whether argument takes a value. `false` makes it a boolean flag (default: `true`) |
| `value_hint` | string | no| Completion hint (see [this](https://docs.rs/clap/latest/clap/enum.ValueHint.html) for reference) |
| `possible_values` | array | no| List of allowed values for constrained options |

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

Subcommands can be nested in other subcommands.

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
# Build completions for all currently supported shells
scog spec.yaml

# Build completions only for bash
scog bash spec.yaml

# Build completions only for bash and zsh
scog bash zsh spec.yaml
```

## How it works

scog is merely some glue to use [clap](https://github.com/clap-rs/clap)'s own completion generation, but for utilities that are _not_ built using Clap, or even Rust itself. Your spec is parsed and converted into a `clap::Command` structure at runtime, then clap's proven completion generators produce completions.

## Example

Given the following spec:

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

Here's what's generated for each shell:

### Fish

<details>
  <summary>Click to expand/collapse</summary>

```fish
# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_myapp_global_optspecs
	string join \n c/config= v/verbose o/output= f/format= h/help
end

function __fish_myapp_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_myapp_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_myapp_using_subcommand
	set -l cmd (__fish_myapp_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c myapp -n "__fish_myapp_needs_command" -s c -l config -d 'Path to configuration file' -r -F
complete -c myapp -n "__fish_myapp_needs_command" -s o -l output -d 'Output directory' -r -f -a "(__fish_complete_directories)"
complete -c myapp -n "__fish_myapp_needs_command" -s f -l format -d 'Output format' -r -f -a "json\t''
yaml\t''
toml\t''"
complete -c myapp -n "__fish_myapp_needs_command" -s v -l verbose -d 'Enable verbose output'
complete -c myapp -n "__fish_myapp_needs_command" -s h -l help -d 'Print help'
complete -c myapp -n "__fish_myapp_needs_command" -f -a "init" -d 'Initialize a new project'
complete -c myapp -n "__fish_myapp_needs_command" -f -a "build" -d 'Build the project'
complete -c myapp -n "__fish_myapp_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myapp -n "__fish_myapp_using_subcommand init" -l template -d 'Project template to use' -r -f -a "basic\t''
advanced\t''
minimal\t''"
complete -c myapp -n "__fish_myapp_using_subcommand init" -s h -l help -d 'Print help'
complete -c myapp -n "__fish_myapp_using_subcommand build" -l target -d 'Build target' -r -f
complete -c myapp -n "__fish_myapp_using_subcommand build" -l release -d 'Build in release mode'
complete -c myapp -n "__fish_myapp_using_subcommand build" -s h -l help -d 'Print help'
complete -c myapp -n "__fish_myapp_using_subcommand help; and not __fish_seen_subcommand_from init build help" -f -a "init" -d 'Initialize a new project'
complete -c myapp -n "__fish_myapp_using_subcommand help; and not __fish_seen_subcommand_from init build help" -f -a "build" -d 'Build the project'
complete -c myapp -n "__fish_myapp_using_subcommand help; and not __fish_seen_subcommand_from init build help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
```

</details>


### Bash

<details>
  <summary>Click to expand/collapse</summary>

```bash
_myapp() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="myapp"
                ;;
            myapp,build)
                cmd="myapp__build"
                ;;
            myapp,help)
                cmd="myapp__help"
                ;;
            myapp,init)
                cmd="myapp__init"
                ;;
            myapp__help,build)
                cmd="myapp__help__build"
                ;;
            myapp__help,help)
                cmd="myapp__help__help"
                ;;
            myapp__help,init)
                cmd="myapp__help__init"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        myapp)
            opts="-c -v -o -f -h --config --verbose --output --format --help init build help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --config)
                    local oldifs
                    if [ -n "${IFS+x}" ]; then
                        oldifs="$IFS"
                    fi
                    IFS=$'\n'
                    COMPREPLY=($(compgen -f "${cur}"))
                    if [ -n "${oldifs+x}" ]; then
                        IFS="$oldifs"
                    fi
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o filenames
                    fi
                    return 0
                    ;;
                -c)
                    local oldifs
                    if [ -n "${IFS+x}" ]; then
                        oldifs="$IFS"
                    fi
                    IFS=$'\n'
                    COMPREPLY=($(compgen -f "${cur}"))
                    if [ -n "${oldifs+x}" ]; then
                        IFS="$oldifs"
                    fi
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o filenames
                    fi
                    return 0
                    ;;
                --output)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                -o)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json yaml toml" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json yaml toml" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__build)
            opts="-h --release --target --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --target)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__help)
            opts="init build help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__help__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__help__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        myapp__init)
            opts="-h --template --help <name>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --template)
                    COMPREPLY=($(compgen -W "basic advanced minimal" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _myapp -o nosort -o bashdefault -o default myapp
else
    complete -F _myapp -o bashdefault -o default myapp
fi
```

</details>

### Zsh

<details>
  <summary>Click to expand/collapse</summary>

```zsh
#compdef myapp

autoload -U is-at-least

_myapp() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration file]: :_files' \
'--config=[Path to configuration file]: :_files' \
'-o+[Output directory]: :_files -/' \
'--output=[Output directory]: :_files -/' \
'-f+[Output format]: :(json yaml toml)' \
'--format=[Output format]: :(json yaml toml)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_myapp_commands" \
"*::: :->myapp" \
&& ret=0
    case $state in
    (myapp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:myapp-command-$line[1]:"
        case $line[1] in
            (init)
_arguments "${_arguments_options[@]}" : \
'--template=[Project template to use]: :(basic advanced minimal)' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Project name:_default' \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
'--target=[Build target]: :' \
'--release[Build in release mode]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_myapp__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:myapp-help-command-$line[1]:"
        case $line[1] in
            (init)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_myapp_commands] )) ||
_myapp_commands() {
    local commands; commands=(
'init:Initialize a new project' \
'build:Build the project' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'myapp commands' commands "$@"
}
(( $+functions[_myapp__build_commands] )) ||
_myapp__build_commands() {
    local commands; commands=()
    _describe -t commands 'myapp build commands' commands "$@"
}
(( $+functions[_myapp__help_commands] )) ||
_myapp__help_commands() {
    local commands; commands=(
'init:Initialize a new project' \
'build:Build the project' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'myapp help commands' commands "$@"
}
(( $+functions[_myapp__help__build_commands] )) ||
_myapp__help__build_commands() {
    local commands; commands=()
    _describe -t commands 'myapp help build commands' commands "$@"
}
(( $+functions[_myapp__help__help_commands] )) ||
_myapp__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'myapp help help commands' commands "$@"
}
(( $+functions[_myapp__help__init_commands] )) ||
_myapp__help__init_commands() {
    local commands; commands=()
    _describe -t commands 'myapp help init commands' commands "$@"
}
(( $+functions[_myapp__init_commands] )) ||
_myapp__init_commands() {
    local commands; commands=()
    _describe -t commands 'myapp init commands' commands "$@"
}

if [ "$funcstack[1]" = "_myapp" ]; then
    _myapp "$@"
else
    compdef _myapp myapp
fi
```

</details>

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgments

Built with [clap](https://github.com/clap-rs/clap) and [clap_complete](https://crates.io/crates/clap-complete).