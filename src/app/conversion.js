import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import { toFileInfo } from './fileQueue'

export function subscribeFileProgress(handler) {
  return listen('file-progress', event => handler(event.payload))
}

export async function selectNcmFiles() {
  const selected = await open({
    multiple: true,
    filters: [{
      name: 'NCM Files',
      extensions: ['ncm']
    }]
  })

  if (!selected) return []

  const fileList = Array.isArray(selected) ? selected : [selected]
  return fileList.map(toFileInfo)
}

export async function selectNcmFolder() {
  const selected = await open({ directory: true })
  if (!selected) return []

  return invoke('scan_ncm_files', { path: selected })
}

export async function selectDirectory() {
  return open({ directory: true })
}

export async function convertFiles(paths, outputDir) {
  return invoke('convert_files', {
    files: paths,
    outputDir: outputDir || null
  })
}

export async function scanPendingNcmFiles(inputDir, outputDir) {
  return invoke('scan_pending_ncm_files', { inputDir, outputDir })
}

export async function loadSettings(outputDir) {
  return invoke('load_settings', { outputDir })
}

export async function saveSettings(settings) {
  return invoke('save_settings', { settings })
}
