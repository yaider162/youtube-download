use std::path::PathBuf;
use yt_dlp::Youtube;
use yt_dlp::client::deps::Libraries;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    let video_path = fetcher.download_video_from_url(url, "my-video.mp4").await?;
    Ok(())
}
