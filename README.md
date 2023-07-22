# fullscreen_yt_video_player

This simple Tauri-App opens a YouTube video in full-screen mode.

## For Users:
You can add a `config.json` file to configure the video and start time.

### Example Configuration:
*Note that if the configuration file is missing or incorrectly formatted, the default configuration will be used instead.*

Filename: `config.json`
#### File Contents:
```json
{
    "video_id": "dQw4w9WgXcQ",
    "start_time": 0
}
```

## For Developers:
![Techstack](https://skillicons.dev/icons?i=rust,vue,ts)

Install Rust, NodeJS and Yarn and follow the [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites). 

Then install the Tauri CLI:
```sh
cargo install tauri-cli
```

### Developing and debugging the project:
```sh
cargo tauri dev
```

### Building the project:
```sh
cargo tauri build
```
