<script>
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'
  import { open } from '@tauri-apps/plugin-dialog'
  import { onMount } from 'svelte'
  import DropZone from './components/DropZone.svelte'
  import FileList from './components/FileList.svelte'
  import SettingsModal from './components/SettingsModal.svelte'
  import logoIcon from '../src-tauri/icons/icon_32x32.png'
  import { Settings, FolderOpen } from 'lucide-svelte'

  const settingsPointerKey = 'ncm-converter-output-dir'
  
  let files = $state([])
  let outputDir = $state('')
  let isProcessing = $state(false)
  let isSettingsOpen = $state(false)
  let settingsError = $state('')
  let settings = $state({
    inputDir: '',
    outputDir: '',
    autoConvertOnStart: false
  })
  let draftSettings = $state({
    inputDir: '',
    outputDir: '',
    autoConvertOnStart: false
  })
  let stats = $derived({
    total: files.length,
    success: files.filter(f => f.status === 'success').length,
    failed: files.filter(f => f.status === 'error').length,
    processing: files.filter(f => f.status === 'processing').length
  })

  let eventUnlisten = $state(null)

  onMount(async () => {
    eventUnlisten = await listen('file-progress', (event) => {
      const { path, status, progress, error } = event.payload
      const index = files.findIndex(f => f.path === path)
      if (index !== -1) {
        files[index] = {
          ...files[index],
          status,
          progress: progress || 0,
          error: error || null
        }
      }
    })

    await loadSettingsFromPointer()
  })

  function addFiles(newFiles) {
    for (const file of newFiles) {
      const exists = files.some(f => f.path === file.path)
      if (!exists) {
        files.push({
          ...file,
          status: 'pending',
          progress: 0,
          error: null
        })
      }
    }
  }

  async function selectFiles() {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'NCM Files',
        extensions: ['ncm']
      }]
    })
    
    if (selected) {
      const fileList = Array.isArray(selected) ? selected : [selected]
      const newFiles = fileList.map(path => ({
        path,
        name: path.split(/[/\\]/).pop()
      }))
      addFiles(newFiles)
    }
  }

  async function selectFolder() {
    const selected = await open({
      directory: true
    })
    
    if (selected) {
      const ncmFiles = await invoke('scan_ncm_files', { path: selected })
      addFiles(ncmFiles)
    }
  }

  async function selectOutputDir() {
    const selected = await open({
      directory: true
    })
    
    if (selected) {
      outputDir = selected
    }
  }

  async function convertFilePaths(paths, targetOutputDir) {
    if (paths.length === 0 || isProcessing) return
    
    isProcessing = true

    files.forEach(f => {
      if (paths.includes(f.path) && f.status !== 'success') {
        f.status = 'pending'
        f.progress = 0
        f.error = null
      }
    })
    
    try {
      await invoke('convert_files', {
        files: paths,
        outputDir: targetOutputDir || null
      })
    } catch (error) {
      console.error('Processing error:', error)
    } finally {
      isProcessing = false
    }
  }

  async function startProcessing() {
    const pendingPaths = files
      .filter(f => f.status !== 'success')
      .map(f => f.path)

    await convertFilePaths(pendingPaths, outputDir || settings.outputDir || null)
  }

  function clearCompleted() {
    files = files.filter(f => f.status !== 'success' && f.status !== 'error')
  }

  function removeFile(path) {
    files = files.filter(f => f.path !== path)
  }

  function openSettings() {
    draftSettings = { ...settings }
    settingsError = ''
    isSettingsOpen = true
  }

  async function selectSettingsInputDir() {
    const selected = await open({ directory: true })
    if (selected) {
      draftSettings.inputDir = selected
      if (!draftSettings.outputDir) {
        draftSettings.autoConvertOnStart = false
      }
    }
  }

  async function selectSettingsOutputDir() {
    const selected = await open({ directory: true })
    if (selected) {
      draftSettings.outputDir = selected
      if (!draftSettings.inputDir) {
        draftSettings.autoConvertOnStart = false
      }
    }
  }

  async function saveSettings() {
    settingsError = ''

    if (!draftSettings.outputDir) {
      settingsError = '请先选择默认输出文件夹。'
      return
    }

    if (draftSettings.autoConvertOnStart && !draftSettings.inputDir) {
      settingsError = '开启启动自动转换前，请先选择默认输入文件夹。'
      return
    }

    try {
      await invoke('save_settings', { settings: draftSettings })
      localStorage.setItem(settingsPointerKey, draftSettings.outputDir)
      settings = { ...draftSettings }
      outputDir = settings.outputDir
      isSettingsOpen = false
    } catch (error) {
      settingsError = String(error)
    }
  }

  async function loadSettingsFromPointer() {
    const savedOutputDir = localStorage.getItem(settingsPointerKey)
    if (!savedOutputDir) return

    try {
      const loadedSettings = await invoke('load_settings', { outputDir: savedOutputDir })
      if (!loadedSettings) return

      settings = loadedSettings
      outputDir = settings.outputDir

      if (settings.autoConvertOnStart && settings.inputDir && settings.outputDir) {
        await runStartupAutoConvert(settings)
      }
    } catch (error) {
      console.error('Failed to load settings:', error)
    }
  }

  async function runStartupAutoConvert(activeSettings) {
    if (isProcessing) return

    try {
      const pendingFiles = await invoke('scan_pending_ncm_files', {
        inputDir: activeSettings.inputDir,
        outputDir: activeSettings.outputDir
      })

      if (pendingFiles.length === 0) return

      addFiles(pendingFiles)
      await convertFilePaths(
        pendingFiles.map(file => file.path),
        activeSettings.outputDir
      )
    } catch (error) {
      console.error('Startup auto convert failed:', error)
    }
  }
</script>

<div class="app-container">
  <header class="app-header">
    <div class="header-content">
      <div class="logo">
        <img class="logo-icon" src={logoIcon} alt="" aria-hidden="true" />
        <div class="logo-text">
          <h1>NCM Converter</h1>
          <span class="subtitle">网易云音乐格式转换</span>
        </div>
      </div>
      
      <div class="header-actions">
        <button class="icon-button" onclick={selectOutputDir} title="选择输出目录">
          <FolderOpen size={20} />
        </button>
        <button class="icon-button" title="设置" onclick={openSettings}>
          <Settings size={20} />
        </button>
      </div>
    </div>
    
    {#if outputDir}
      <div class="output-path">
        <span class="output-label">输出目录:</span>
        <span class="output-value">{outputDir}</span>
      </div>
    {/if}
  </header>

  <main class="app-main">
    {#if files.length === 0}
      <DropZone 
        onselectfiles={selectFiles}
        onselectfolder={selectFolder}
      />
    {:else}
      <div class="content-wrapper">
        <div class="action-bar">
          <div class="action-left">
            <button class="glass-button" onclick={selectFiles} disabled={isProcessing}>
              <span class="button-icon">+</span>
              添加文件
            </button>
            <button class="glass-button" onclick={selectFolder} disabled={isProcessing}>
              <FolderOpen size={16} />
              添加文件夹
            </button>
          </div>
          
          <div class="action-right">
            <button class="glass-button secondary" onclick={clearCompleted} disabled={isProcessing}>
              清除已完成
            </button>
            <button 
              class="primary-button" 
              onclick={startProcessing}
              disabled={isProcessing || files.every(f => f.status === 'success')}
            >
              {#if isProcessing}
                <span class="spinner"></span>
                处理中...
              {:else}
                开始转换
              {/if}
            </button>
          </div>
        </div>

        <FileList 
          {files}
          onremove={removeFile}
        />
      </div>
    {/if}
  </main>

  {#if files.length > 0}
    <footer class="app-footer">
      <div class="stats">
        <span class="stat-item">
          <span class="stat-label">总计:</span>
          <span class="stat-value">{stats.total}</span>
        </span>
        <span class="stat-item success">
          <span class="stat-label">成功:</span>
          <span class="stat-value">{stats.success}</span>
        </span>
        <span class="stat-item failed">
          <span class="stat-label">失败:</span>
          <span class="stat-value">{stats.failed}</span>
        </span>
        {#if isProcessing}
          <span class="stat-item processing">
            <span class="stat-label">处理中:</span>
            <span class="stat-value">{stats.processing}</span>
          </span>
        {/if}
      </div>
    </footer>
  {/if}

  {#if isSettingsOpen}
    <SettingsModal
      settings={draftSettings}
      error={settingsError}
      onclose={() => isSettingsOpen = false}
      onsave={saveSettings}
      onselectinput={selectSettingsInputDir}
      onselectoutput={selectSettingsOutputDir}
    />
  {/if}
</div>

<style>
  .app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f0f23 100%);
    position: relative;
    overflow: hidden;
  }

  .app-container::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: radial-gradient(ellipse at 30% 20%, rgba(120, 119, 198, 0.15) 0%, transparent 50%),
                radial-gradient(ellipse at 70% 80%, rgba(255, 119, 198, 0.1) 0%, transparent 50%);
    pointer-events: none;
    z-index: 0;
  }

  .app-header {
    position: relative;
    z-index: 1;
    padding: 20px 32px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(20px);
  }

  .header-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .logo-icon {
    width: 28px;
    height: 28px;
    display: block;
    object-fit: contain;
    filter: drop-shadow(0 0 14px rgba(59, 130, 246, 0.35));
  }

  .logo-text h1 {
    font-size: 22px;
    font-weight: 600;
    background: linear-gradient(135deg, #fff 0%, #a78bfa 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 400;
  }

  .header-actions {
    display: flex;
    gap: 8px;
  }

  .icon-button {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .icon-button:hover {
    background: rgba(255, 255, 255, 0.15);
    color: #fff;
    transform: translateY(-2px);
  }

  .output-path {
    margin-top: 12px;
    padding: 10px 16px;
    background: rgba(124, 58, 237, 0.1);
    border: 1px solid rgba(124, 58, 237, 0.2);
    border-radius: 10px;
    font-size: 13px;
  }

  .output-label {
    color: rgba(255, 255, 255, 0.5);
    margin-right: 8px;
  }

  .output-value {
    color: #a78bfa;
    font-family: 'SF Mono', 'Fira Code', monospace;
  }

  .app-main {
    flex: 1;
    position: relative;
    z-index: 1;
    padding: 24px 32px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .content-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 20px;
  }

  .action-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    flex-wrap: wrap;
  }

  .action-left,
  .action-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .glass-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.8);
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .glass-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.15);
    transform: translateY(-2px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .glass-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .glass-button.secondary {
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.15);
  }

  .button-icon {
    font-size: 18px;
    font-weight: 600;
  }

  .primary-button {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 28px;
    font-size: 14px;
    font-weight: 600;
    color: #fff;
    background: linear-gradient(135deg, #7c3aed 0%, #6366f1 100%);
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 20px rgba(124, 58, 237, 0.4);
  }

  .primary-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 32px rgba(124, 58, 237, 0.5);
  }

  .primary-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .app-footer {
    position: relative;
    z-index: 1;
    padding: 16px 32px;
    background: rgba(255, 255, 255, 0.03);
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(20px);
  }

  .stats {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
  }

  .stat-label {
    color: rgba(255, 255, 255, 0.5);
  }

  .stat-value {
    font-weight: 600;
    color: rgba(255, 255, 255, 0.8);
  }

  .stat-item.success .stat-value {
    color: #34d399;
  }

  .stat-item.failed .stat-value {
    color: #f87171;
  }

  .stat-item.processing .stat-value {
    color: #fbbf24;
  }
</style>
