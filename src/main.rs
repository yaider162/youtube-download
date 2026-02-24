use std::path::PathBuf;
use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    unsafe {
        env::set_var("YT_DLP_NO_WARNINGS", "1");
        env::set_var("YT_DLP_UPDATE", "0");
    }

    let libraries = Libraries::new(
        PathBuf::from("libs/yt-dlp.exe"),
        PathBuf::from("libs/ffmpeg.exe"),
    );
    let downloader = Downloader::builder(libraries, "output").build().await?;

    // Aqui hago la magia
    let url = String::from(
        "https://www.youtube.com/watch?v=WXIfT-dQtCo&pp=ygUXcHJvbWVzYXMgc29icmUgZWwgYmlkZXQ%3D",
    );
    let video = downloader.fetch_video_infos(url).await?;
    let _video_path = downloader
        .download_video(&video, "Promesas_sobre_el_bidet.mp4")
        .await?;
    println!("OK");
    Ok(())
}
