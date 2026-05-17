# thirdpass-ansible

Ansible package extension for Thirdpass.

This repo contains the Thirdpass extension that understands Ansible Galaxy
collections and Ansible dependency files. It can be used by the Thirdpass CLI to
discover Ansible dependencies and fetch package metadata from Ansible Galaxy.

## Install

Install the extension as a normal Cargo binary:

```bash
cargo install thirdpass-ansible
```

Ensure Cargo's binary directory, usually `~/.cargo/bin`, is on `PATH`, then
verify Thirdpass can discover the extension:

```bash
thirdpass extension list
```
