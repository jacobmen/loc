use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
#[command(name = "loc")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run command
    #[command(visible_alias = "r", arg_required_else_help = true)]
    Run {
        /// Name of command to run
        command_name: String,
    },
    /// Add command for current working directory. Script can be
    /// specified either in the prompt or by copying from a file.
    #[command(visible_alias = "a", arg_required_else_help = true)]
    Add {
        /// Name of command to add
        command_name: String,
        /// Path of file to copy into loc
        #[clap(short, long, conflicts_with = "script")]
        file: Option<PathBuf>,
        /// Script to copy into loc
        script: Vec<OsString>,
    },
    /// Edit command in $EDITOR
    #[command(visible_alias = "e", arg_required_else_help = true)]
    Edit {
        /// Command to edit
        command_name: String,
    },
    /// Delete command for current working directory
    #[command(visible_alias = "d", arg_required_else_help = true)]
    Delete {
        /// Command to remove
        command_name: String,
    },
    /// List commands in current working directory
    #[command(visible_alias = "ls")]
    List {
        /// Instead, list all directories with commands
        #[clap(short, long, action)]
        dirs: bool,
    },
}
