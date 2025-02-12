use std::fmt::{self, Display, Formatter};

use clap::Args;

// Common systemd unit options
// From [systemd.unit](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)
#[derive(Args, Default, Debug, Clone, PartialEq)]
pub struct Unit {
    /// Add a description to the unit
    ///
    /// A description should be a short, human readable title of the unit
    ///
    /// Converts to "Description=DESCRIPTION"
    #[arg(short, long)]
    description: Option<String>,

    /// Add (weak) requirement dependencies to the unit
    ///
    /// Converts to "Wants=WANTS[ ...]"
    ///
    /// Can be specified multiple times
    #[arg(long)]
    wants: Vec<String>,

    /// Similar to --wants, but adds stronger requirement dependencies
    ///
    /// Converts to "Requires=REQUIRES[ ...]"
    ///
    /// Can be specified multiple times
    #[arg(long)]
    requires: Vec<String>,

    /// Configure ordering dependency between units
    ///
    /// Converts to "Before=BEFORE[ ...]"
    ///
    /// Can be specified multiple times
    #[arg(long)]
    before: Vec<String>,

    /// Configure ordering dependency between units
    ///
    /// Converts to "After=AFTER[ ...]"
    ///
    /// Can be specified multiple times
    #[arg(long)]
    after: Vec<String>,
}

impl Unit {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    pub fn add_dependencies(&mut self, depends_on: docker_compose_types::DependsOnOptions) {
        let depends_on = match depends_on {
            docker_compose_types::DependsOnOptions::Simple(vec) => vec,
            docker_compose_types::DependsOnOptions::Conditional(map) => map.into_keys().collect(),
        };

        self.requires.extend(
            depends_on
                .into_iter()
                .map(|dependency| dependency + ".service"),
        );
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "[Unit]")?;

        if let Some(description) = &self.description {
            writeln!(f, "Description={description}")?;
        }

        if !self.wants.is_empty() {
            writeln!(f, "Wants={}", self.wants.join(" "))?;
        }

        if !self.requires.is_empty() {
            writeln!(f, "Requires={}", self.requires.join(" "))?;
        }

        if !self.before.is_empty() {
            writeln!(f, "Before={}", self.before.join(" "))?;
        }

        if !self.before.is_empty() {
            writeln!(f, "After={}", self.after.join(" "))?;
        }

        Ok(())
    }
}
