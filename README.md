# NCMToMP3ForWangYiYun

一个把网易云音乐本地下载的本地 `.ncm` 文件解析为 MP3 文件的桌面工具，支持 ID3 标签提取，全程本地运行，快速解析。

The app is built with Svelte, Vite, Tauri 2, and Rust.

## Features

- Convert individual `.ncm` files or folders.
- Write title, artist, album, cover art, and matching lyrics into the output MP3 ID3 tag when metadata is available.
- Embed same-name `.lrc` or `.irc` lyrics files placed next to the source `.ncm` file.
- Configure default input and output folders.
- Optionally scan and convert new files automatically on startup.

## Requirements

- Node.js 18 or newer.
- Rust stable toolchain.
- Windows WebView2 Runtime.
- Tauri system dependencies for your target platform.

## Development

```bash
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

Windows installers are generated under:

```text
src-tauri/target/release/bundle/
```

## Project Structure

```text
src/                    Svelte frontend
src/components/         UI components
src-tauri/src/          Rust conversion and Tauri commands
src-tauri/icons/        Application icons
src-tauri/capabilities/ Tauri permissions
```

## Notes

This project is intended for working with local files you have the right to process. Do not use it to redistribute copyrighted music or bypass any service terms. This project is not affiliated with NetEase Cloud Music.

## License

MIT
