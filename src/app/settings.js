export const settingsPointerKey = 'ncm-converter-output-dir'

export function createEmptySettings() {
  return {
    inputDir: '',
    outputDir: '',
    autoConvertOnStart: false
  }
}

export function loadSettingsOutputPointer() {
  return localStorage.getItem(settingsPointerKey)
}

export function saveSettingsOutputPointer(outputDir) {
  localStorage.setItem(settingsPointerKey, outputDir)
}

export function validateSettings(settings) {
  if (!settings.outputDir) {
    return '请先选择默认输出文件夹。'
  }

  if (settings.autoConvertOnStart && !settings.inputDir) {
    return '开启启动自动转换前，请先选择默认输入文件夹。'
  }

  return ''
}
