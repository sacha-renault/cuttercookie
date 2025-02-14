I'll update the README with more detailed and precise documentation reflecting the new CLI structure:

# Cuttercookie

Transform existing projects into reusable templates - the reverse of cookiecutter.

## Overview

Cuttercookie is a powerful file system utility that converts real, existing projects into template structures. Unlike cookiecutter, which generates projects from templates, cuttercookie takes a concrete implementation and transforms it into a flexible, reusable template compatible with cookiecutter.

## Key Features

- Keep existing project structures
- Selectively exclude desired directories and files
- Generate cookiecutter-compatible templates
- Customize template generation with flexible options

## Installation

Install Cuttercookie using Cargo:

> Not available yet
```bash
cargo install cuttercookie
```

## Usage

### Basic Usage

To create a cookiecutter template, navigate to the directory where you want to generate the template and run the Cuttercookie command. Specify the path to the project you wish to templatize, which can be an absolute or relative path. This approach allows you to easily transform existing project structures into flexible, reusable templates that can be used with cookiecutter, all from a single, convenient command. The root must have a cookiecutter.json file that indicate the pattern for the regex to templatize the project.

```bash
cuttercookie /path/to/project
```

### How to make the `cookiecutter.json`

Exactly like a normal cookiecutter.json !

```json
{
    "project_name": "MySuperProject",
    "author_name": "the name of a super author"
}
```

In the project, it will replace all the `MySuperProject` into {{cookiecutter.project_name}} !

### Advanced Options

Exclude specific directories or files:

```bash
cuttercookie /path/to/project --excluded-items target,node_modules,.git
```

Exclude the root project directory:

```bash
cuttercookie /path/to/project --no-root
```

## Command Line Arguments

| Argument | Description | Required | Format | Example |
|----------|-------------|----------|--------|---------|
| `path` | Directory path to the project to be templatized | Yes | String | `./my-project` |
| `--excluded-items`, `-e` | Comma-separated list of directories or files to exclude | No | Comma-separated strings | `target/,node_modules/` |
| `--no-root`, `-n` | Exclude the main project directory from template generation | No | Flag | `--no-root` |

## Examples

### React Project Template

Convert a React project into a template, excluding build artifacts and dependencies:

```bash
cuttercookie ./my-react-app -e build,node_modules,coverage
```

### Python Project Template

Create a template from a Python project, excluding virtual environments and cache directories:

```bash
cuttercookie ./python-project -e venv/,__pycache__/,.pytest_cache/ --no-root
```

## How It Works

1. Analyze the provided project directory structure
2. Apply exclusion rules for specified directories and files
3. Identify common patterns and variables in the project
4. Generate a cookiecutter-compatible template structure

When using the `--no-root` flag, Cuttercookie will:
- Exclude the root project directory from the template
- Preserve the internal directory and file structure
- Useful for creating more granular or nested templates

## Use Cases

- Convert existing projects to reusable templates
- Standardize project structures across teams
- Create project scaffolding tools
- Facilitate code generation and project initialization

## Contributing

Contributions are welcome!

### How to Contribute
- Fork the repository
- Create a feature branch
- Implement your changes
- Submit a Pull Request

Please ensure your code follows Rust best practices and includes appropriate tests.


## Contact

[Insert contact information or project maintainer details]