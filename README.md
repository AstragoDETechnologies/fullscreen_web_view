# fullscreen_yt_video_player

This simple Tauri-App opens a YouTube video in full-screen mode.

## For Users:
You can add an `id.txt` file containing only the YouTube Video ID to the same directory that contains the executable to play a certain video.
To start the video at a specific time create a `start_time.txt` file.

### Example (Video ID):
Filename: `id.txt`

#### Contents:
```txt
f8mL0_4GeV0
```

### Example (Start Time):
Filename: `start_time.txt`

#### Contents:
```txt
17
```

## For Developers:
![Techstack](https://skillicons.dev/icons?i=rust,vue,ts)

Install Rust, NodeJS and Yarn and follow the [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites). 

Developing and debugging the project:
```sh
cargo tauri dev
```

Building the project:
```sh
cargo tauri build
```
