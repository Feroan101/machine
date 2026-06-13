# Troubleshooting Guide

This document covers common issues encountered while building or using Machine.

## Build Failures

### Missing Dependencies
Ensure you have `pkg-config` and libssl-dev (or equivalent) installed for your distribution.

```bash
# Debian/Ubuntu
sudo apt install pkg-config libssl-dev

# Fedora
sudo dnf install pkgconf-pkg-config openssl-devel
```

### Rust Version
Machine requires Rust 2024 edition features (stable 1.75+). Update your toolchain:
```bash
rustup update stable
```

## Runtime Issues

### Permission Denied
If you see permission errors when running commands like `machine ports` or `machine security`, ensure you have read access to `/proc` and `/sys`. Some advanced security checks may require elevated privileges, though Machine defaults to read-only analysis.

### Database Issues
Machine stores history in `~/.local/share/machine/machine.db`.

**Database is locked**: This usually happens if multiple instances of Machine are trying to write to the history at the same time. Ensure no background processes are deadlocked.

**Resetting the database**: If the database becomes corrupted, you can safely delete it. Machine will recreate it on the next run.
```bash
rm ~/.local/share/machine/machine.db
```

## Command Specific Errors

### machine services
This command requires `systemctl`. It will not work on systems using OpenRC, SysVinit, or Runit.

### machine latency
Requires the `host` and `ping` commands to be available in your PATH.

## Reporting Issues

If your issue is not listed here, please open an issue on the [GitHub repository](https://github.com/Feroan101/machine/issues) with the output of `machine report --verbose`.
