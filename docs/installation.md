# Installation Guide

Machine is designed to be easy to install and portable across Linux distributions.

## Requirements

- **Linux**: Machine is a Linux-first utility.
- **Rust**: Only required if installing from source (stable 1.75+ recommended).
- **systemd**: Required for service analysis commands.

## Installing from Source

This is the recommended method for developers and advanced users.

```bash
git clone https://github.com/Feroan101/machine.git
cd machine
./install.sh
```

The `install.sh` script will detect your architecture, check for local builds, and install the binary to `/usr/local/bin/machine`.

### Manual Cargo Installation

If you prefer using `cargo` directly:

```bash
cargo install --path .
```

## Manual Download

You can download the latest static binary from the [GitHub Releases](https://github.com/Feroan101/machine/releases) page.

1. Download the `machine-x86_64-unknown-linux-musl.tar.gz` for your architecture.
2. Extract the archive.
3. Move the `machine` binary to your PATH (e.g., `/usr/local/bin/`).

## Updating Machine

To update to the latest version, simply run the installation script again:

```bash
cd machine
git pull
./install.sh
```

## Uninstallation

To remove Machine from your system:

```bash
sudo rm /usr/local/bin/machine
rm -rf ~/.local/share/machine/
```
