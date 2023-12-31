use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint, ValueEnum};

/// Program to format config file formats using serde.
/// Files supported are: Json and Toml
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Minify or Pretty the file
    #[command(subcommand)]
    pub mode: Mode,

    /// whether or not to use streaming. Enabling this is helpful if your
    /// file is too big to fit in RAM
    #[arg(short, long, default_value_t = false)]
    pub streaming: bool,

    /// Specify file format. If not specified, format is determined by extension
    #[arg(short, long, default_value = None)]
    pub format: Option<FileFormat>,
}

impl Cli {
    pub fn file(&self) -> &PathBuf {
        self.mode.file()
    }
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    Minify {
        /// Name of file to be formatted
		#[arg(value_hint = ValueHint::FilePath)]
        file: PathBuf,
    },
    Pretty {
        /// Name of file to be formatted
		#[arg(value_hint = ValueHint::FilePath)]
        file: PathBuf,
    },
}

impl Mode {
    pub fn file(&self) -> &PathBuf {
        match self {
            Mode::Minify { file } |
            Mode::Pretty { file } => file,
        }
    }

    pub fn pretty(&self) -> bool {
        match self {
            Mode::Pretty {..} => true,
            Mode::Minify {..} => false,
        }
    }
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum FileFormat {
    Json,
    Toml,
}
