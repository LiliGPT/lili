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
