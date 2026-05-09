# Contributing

Thanks for helping improve NCM Converter.

## Local Setup

```bash
npm install
npm run tauri:dev
```

## Before Opening a Pull Request

Run the core checks:

```bash
npm run build
cd src-tauri
cargo test
```

Keep generated build output out of commits. The repository should not include `node_modules`, `dist`, `target`, installers, converted audio files, or local settings.

## Scope

Please keep changes focused. For behavior changes, include a short explanation of the user-facing impact and add or update tests where practical.
