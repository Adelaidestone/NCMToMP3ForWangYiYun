<script>
  import { onMount } from 'svelte'
  import {
    addUniqueQueuedFiles,
    clearFinishedFiles,
    pendingFilePaths,
    resetFilesForProcessing,
    updateFileProgress
  } from './app/fileQueue'
  import {
    convertFiles,
    loadSettings,
    saveSettings as persistSettings,
    scanPendingNcmFiles,
    selectDirectory,
    selectNcmFiles,
    selectNcmFolder,
    subscribeFileProgress
  } from './app/conversion'
  import {
    createEmptySettings,
    loadSettingsOutputPointer,
    saveSettingsOutputPointer,
    validateSettings
  } from './app/settings'
  import AppHeader from './components/AppHeader.svelte'
  import DropZone from './components/DropZone.svelte'
  import FileList from './components/FileList.svelte'
  import SettingsModal from './components/SettingsModal.svelte'
  import { FolderOpen } from 'lucide-svelte'
  
  let files = $state([])
  let outputDir = $state('')
  let isProcessing = $state(false)
  let isSettingsOpen = $state(false)
  let settingsError = $state('')
  let settings = $state(createEmptySettings())
  let draftSettings = $state(createEmptySettings())
  let stats = $derived({
    total: files.length,
    success: files.filter(f => f.status === 'success').length,
    failed: files.filter(f => f.status === 'error').length,
    processing: files.filter(f => f.status === 'processing').length
  })

  let eventUnlisten = $state(null)

  onMount(async () => {
    eventUnlisten = await subscribeFileProgress(payload => {
      files = updateFileProgress(files, payload)
    })

    await loadSettingsFromPointer()
  })

  function addFiles(newFiles) {
    files = addUniqueQueuedFiles(files, newFiles)
  }

  async function selectFiles() {
    addFiles(await selectNcmFiles())
  }

  async function selectFolder() {
    addFiles(await selectNcmFolder())
  }

  async function selectOutputDir() {
    const selected = await selectDirectory()
    if (selected) {
      outputDir = selected
    }
  }

  async function convertFilePaths(paths, targetOutputDir) {
    if (paths.length === 0 || isProcessing) return
    
    isProcessing = true
    files = resetFilesForProcessing(files, paths)
    
    try {
      await convertFiles(paths, targetOutputDir)
    } catch (error) {
      console.error('Processing error:', error)
    } finally {
      isProcessing = false
    }
  }

  async function startProcessing() {
    await convertFilePaths(pendingFilePaths(files), outputDir || settings.outputDir || null)
  }

  function clearCompleted() {
    files = clearFinishedFiles(files)
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
    const selected = await selectDirectory()
    if (selected) {
      draftSettings.inputDir = selected
      if (!draftSettings.outputDir) {
        draftSettings.autoConvertOnStart = false
      }
    }
  }

  async function selectSettingsOutputDir() {
    const selected = await selectDirectory()
    if (selected) {
      draftSettings.outputDir = selected
      if (!draftSettings.inputDir) {
        draftSettings.autoConvertOnStart = false
      }
    }
  }

  async function saveSettings() {
    settingsError = ''
    settingsError = validateSettings(draftSettings)
    if (settingsError) {
      return
    }

    try {
      await persistSettings(draftSettings)
      saveSettingsOutputPointer(draftSettings.outputDir)
      settings = { ...draftSettings }
      outputDir = settings.outputDir
      isSettingsOpen = false
    } catch (error) {
      settingsError = String(error)
    }
  }

  async function loadSettingsFromPointer() {
    const savedOutputDir = loadSettingsOutputPointer()
    if (!savedOutputDir) return

    try {
      const loadedSettings = await loadSettings(savedOutputDir)
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
      const pendingFiles = await scanPendingNcmFiles(
        activeSettings.inputDir,
        activeSettings.outputDir
      )

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
  <AppHeader
    {outputDir}
    onselectoutput={selectOutputDir}
    onopensettings={openSettings}
  />

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
