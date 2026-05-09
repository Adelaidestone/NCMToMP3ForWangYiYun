# NCMToMP3ForWangYiYun

一个把网易云音乐本地下载的 `.ncm` 文件解析为 MP3 文件的桌面工具，支持 ID3 标签提取，全程本地运行，快速解析。

本项目使用 Svelte、Vite、Tauri 2 和 Rust 构建。

## 功能

- 支持转换单个 `.ncm` 文件或整个文件夹。
- 在元数据可用时，将标题、歌手、专辑、封面和歌词写入输出 MP3 的 ID3 标签。
- 支持嵌入与源 `.ncm` 文件同名、位于同一目录下的 `.lrc` 或 `.irc` 歌词文件。
- 支持设置默认输入文件夹和输出文件夹。
- 支持启动时自动扫描并转换新增文件。

## 环境要求

- Node.js 18 或更新版本。
- Rust stable 工具链。
- Windows WebView2 Runtime。
- 目标平台所需的 Tauri 系统依赖。

## 本地开发

```bash
npm install
npm run tauri:dev
```

## 构建

```bash
npm run tauri:build
```

Windows 安装包会生成在：

```text
src-tauri/target/release/bundle/
```

## 项目结构

```text
src/                    Svelte 前端源码
src/components/         UI 组件
src-tauri/src/          Rust 转换逻辑和 Tauri 命令
src-tauri/icons/        应用图标
src-tauri/capabilities/ Tauri 权限配置
```

## 说明

本项目仅用于处理你有权处理的本地文件。请勿使用本项目重新分发受版权保护的音乐，也请遵守相关服务条款。本项目与网易云音乐没有关联。

## 许可证

MIT
