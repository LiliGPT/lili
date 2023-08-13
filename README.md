# Lili CLI

Lili is a command line interface (CLI) with AI that allows developers to ask GPT-4
to generate code more efficiently with contextual awareness.

It is different from Github Copilot in that it generates code for the project itself
rather than generating code based on the current cursor position.

## Installation

### Ubuntu / Debian

```bash
curl -s https://raw.githubusercontent.com/LiliGPT/lili/main/scripts/install.sh | bash
```

### Windows

Installation in Windows is still experimental, please open an issue if you have 
any problems.

[Download the Windows Installer](https://github.com/LiliGPT/lili/releases/download/v0.0.1/liliv0.0.1.exe)

## Complete Uninstall

### Ubuntu / Debian

Lili's install script only creates a binary file in `/usr/local/bin/lili` and a
configuration directory in `~/.lili`. To completely uninstall Lili, run the
following commands:

To remove the binary file:

```bash
sudo rm -rf /usr/local/bin/lili
```

To remove the configuration directory:

```bash
rm -rf ~/.lili
```

### Windows

Go to Uninstall Programs in Windows and remove the program.

This is not working properly for now, so you can delete the directory manually 
**after uninstalling**.

* Remove the directory (defaults to `C:\Program Files (x86)\Lili`).
* Remove the Path variable that points to Lili

## Supported Languages

Lili Project supports these languages:

- Node.js
- Javascript
- Rust
- Lua

## Getting Started

In order to start using Lili, you need to create a project. Lili does not create
any project for you, it just edit files inside a project.

You can create any project you want than open lili running `lili` from the terminal.

If you prefer you can also give lili a path, for example: `lili ~/my-project`.
The path is optional.
