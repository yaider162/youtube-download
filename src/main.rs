use cli::Commands;

mod cli;
mod videos_download;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();
    // println!("{:?}", args);

    match args.command {
        Commands::Download { url, output: _ } => {
            videos_download::basic_download(url).await?;
        }
        Commands::DownloadAudio { url, output: _ } => {
            videos_download::only_audio_download(url).await?;
        }
    }
    print!("funcciono esta monda");
    Ok(())
}
