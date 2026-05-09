# 贡献指南

感谢你帮助改进 NCMToMP3ForWangYiYun。

## 本地环境

```bash
npm install
npm run tauri:dev
```

## 提交 Pull Request 前

请先运行核心检查：

```bash
npm run build
cd src-tauri
cargo test
```

请不要提交生成产物。仓库中不应包含 `node_modules`、`dist`、`target`、安装包、转换后的音频文件或本地设置文件。

## 变更范围

请保持改动聚焦。对于行为变更，请简要说明对用户可见的影响，并在可行时添加或更新测试。
