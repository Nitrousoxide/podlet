//! Podlet generates [podman](https://podman.io/)
//! [quadlet](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html)
//! (systemd-like) files from a podman command.
//!
//! # Usage
//!
//! ```shell
//! $ podlet podman run quay.io/podman/hello
//! [Container]
//! Image=quay.io/podman/hello
//! ```
//!
//! Run `podlet --help` for more information.

#![warn(clippy::pedantic, clippy::cargo)]
// zbus uses some old dependencies, this is ok for now
#![allow(clippy::multiple_crate_versions)]

mod cli;
mod quadlet;

use clap::Parser;
use color_eyre::eyre;

use self::cli::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    Cli::parse().print_or_write_files()
}
