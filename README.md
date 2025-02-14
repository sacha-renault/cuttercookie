# Cuttercookie

Transform existing projects into reusable templates - the reverse of cookiecutter.

## Overview

Cuttercookie is a file system utility that converts real, existing projects into template structures. While cookiecutter generates projects from templates, cuttercookie does the opposite by taking a concrete implementation and turning it into a template that can be used with cookiecutter.

## Installation

```bash
cargo install cuttercookie
```

## Usage

Basic usage:
```bash
cuttercookie /path/to/project
```

Exclude specific directories or files:
```bash
cuttercookie /path/to/project --excluded-items target/,node_modules/,.git/
```

## Command Line Arguments

| Argument | Description | Required | Format |
|----------|-------------|----------|---------|
| `path` | Directory path to the project that will be templatized | Yes | String |
| `--excluded-items`, `-e` | Comma-separated list of directories or files to exclude from template generation | No | Comma-separated strings |

## Examples

Convert a React project into a template, excluding build artifacts and dependencies:
```bash
cuttercookie ./my-react-app -e build/,node_modules/,coverage/
```

Create a template from a Python project:
```bash
cuttercookie ./python-project -e venv/,__pycache__/,.pytest_cache/
```

## How It Works

1. Cuttercookie analyzes the provided project directory structure
2. Excludes specified directories and files
3. Identifies common patterns and variables in the project
4. Generates a cookiecutter-compatible template structure

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.