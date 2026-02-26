# YaiderTube

Descargador de videos y audio construido en Rust.

## Instalación

```bash
git clone https://github.com/yourusername/youtube-download.git
cd youtube-download
cargo build --release
```
tambien se puede usar el binario y ejecutar **hay que tener una carpeta ./libs/ con los binarios de yt-dlp y ffmpeg**:
```bash
./nombre_del_archivo.exe comando url_del_video
```
## Uso

### Descargar un video

```bash
cargo run -- download "https://www.youtube.com/watch?v=VIDEO_ID"
```

Con directorio personalizado:

```bash
cargo run -- download "https://www.youtube.com/watch?v=VIDEO_ID" -o ./videos
```

### Descargar solo audio

```bash
cargo run -- download-audio "https://www.youtube.com/watch?v=VIDEO_ID" -o ./audio
```

## Comandos

```
download           Descargar video en formato MP4
download-audio     Descargar solo audio del video

Opciones:
  -o, --output <PATH>    Carpeta de salida
  -h, --help             Mostrar ayuda
  -V, --version          Mostrar versión
```

Es una primera versión funcional que solo descarga video y audio de youtube en su máxima calidad, está planeado agregar mas funcionalidades.