<script>
  import logoIcon from '../../src-tauri/icons/icon_32x32.png'
  import { FolderOpen, Settings } from 'lucide-svelte'

  let { outputDir, onselectoutput, onopensettings } = $props()
</script>

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
      <button class="icon-button" onclick={onselectoutput} title="选择输出目录">
        <FolderOpen size={20} />
      </button>
      <button class="icon-button" title="设置" onclick={onopensettings}>
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

<style>
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
</style>
