use std::path::PathBuf;
use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;

fn setup_env() {
    use std::env;
    unsafe {
        env::set_var("YT_DLP_NO_WARNINGS", "1");
        env::set_var("YT_DLP_UPDATE", "0");
    }
}

pub async fn basic_download(url: String) -> Result<(), Box<dyn std::error::Error>> {
    setup_env();
    let libraries = Libraries::new(
        PathBuf::from("libs/yt-dlp.exe"),
        PathBuf::from("libs/ffmpeg.exe"),
    );
    let downloader = Downloader::builder(libraries, "output").build().await?;

    // Aqui hago la magia
    let video = downloader.fetch_video_infos(url).await?;
    // es para darle extension al archivo sino da error
    let filename = format!("{}.mp4", video.title);
    let _video_path = downloader.download_video(&video, &filename).await?;
    Ok(())
}

pub async fn only_audio_download(url: String) -> Result<(), Box<dyn std::error::Error>> {
    setup_env();
    let libraries = Libraries::new(
        PathBuf::from("libs/yt-dlp.exe"),
        PathBuf::from("libs/ffmpeg.exe"),
    );
    let downloader = Downloader::builder(libraries, "output").build().await?;

    // Aqui se descarga el audio
    let video = downloader.fetch_video_infos(url).await?;
    downloader
        .download_audio_stream(&video, "audio.mp3")
        .await?;
    Ok(())
}
