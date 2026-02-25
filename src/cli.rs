use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Descargador de videitos en Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Descargar video sin especificar nada mas en .mp4
    Download {
        /// Url del video
        url: String,
        /// Carpeta de salida
        #[arg(short, long, default_value = ".")]
        output: String,
    },

    /// Descarga de solo audio
    DownloadAudio {
        /// Url del video
        url: String,
        /// Carpeta de salida
        #[arg(short, long, default_value = ".")]
        output: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
