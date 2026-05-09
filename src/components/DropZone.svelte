<script>
  import { Music, Folder } from 'lucide-svelte'
  
  let { onselectfiles, onselectfolder } = $props()
</script>

<div 
  class="dropzone-container"
>
  <div class="dropzone-content">
    <div class="icon-wrapper">
      <span class="note-core">
        <Music size={50} strokeWidth={1.6} />
      </span>
      <span class="floating-note note-one">♪</span>
      <span class="floating-note note-two">♫</span>
    </div>
    
    <div class="text-content">
      <h2 class="title">选择 NCM 文件或文件夹</h2>
      <p class="subtitle">支持 .ncm 格式的网易云音乐加密文件</p>
    </div>

    <div class="action-buttons">
      <button class="action-button" onclick={() => onselectfiles()}>
        <Music size={20} />
        <span>选择文件</span>
      </button>
      
      <button class="action-button" onclick={() => onselectfolder()}>
        <Folder size={20} />
        <span>选择文件夹</span>
      </button>
    </div>

    <div class="features">
      <div class="feature-item">
        <div class="feature-dot"></div>
        <span>100% 离线处理，无网络依赖</span>
      </div>
      <div class="feature-item">
        <div class="feature-dot"></div>
        <span>急速解析，快如闪电</span>
      </div>
      <div class="feature-item">
        <div class="feature-dot"></div>
        <span>批量转换，多线程处理</span>
      </div>
    </div>
  </div>
</div>

<style>
  .dropzone-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 40px;
  }

  .dropzone-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 32px;
    padding: 60px 80px;
    background: rgba(255, 255, 255, 0.03);
    border: 2px dashed rgba(255, 255, 255, 0.15);
    border-radius: 32px;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }

  .dropzone-content::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: radial-gradient(ellipse at center, rgba(124, 58, 237, 0.08) 0%, transparent 70%);
    opacity: 0;
    transition: opacity 0.4s ease;
    pointer-events: none;
  }

  .icon-wrapper {
    width: 120px;
    height: 120px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #a78bfa;
    background: linear-gradient(135deg, rgba(124, 58, 237, 0.2) 0%, rgba(99, 102, 241, 0.2) 100%);
    border-radius: 50%;
    position: relative;
    transition: all 0.4s ease;
    animation: ring-breathe 2.8s ease-in-out infinite;
  }

  .icon-wrapper::before {
    content: '';
    position: absolute;
    inset: -8px;
    border-radius: 50%;
    border: 1px solid rgba(167, 139, 250, 0.22);
    animation: pulse-ring 2.8s ease-in-out infinite;
  }

  .icon-wrapper::after {
    content: '';
    position: absolute;
    inset: 4px;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
    z-index: 0;
  }

  .note-core {
    position: relative;
    z-index: 2;
    display: inline-flex;
    color: #ddd6fe;
    filter: drop-shadow(0 10px 22px rgba(124, 58, 237, 0.42));
    animation: note-bounce 1.35s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
  }

  .floating-note {
    position: absolute;
    z-index: 2;
    color: rgba(216, 180, 254, 0.78);
    font-size: 20px;
    line-height: 1;
    pointer-events: none;
    text-shadow: 0 8px 24px rgba(124, 58, 237, 0.45);
  }

  .note-one {
    top: 22px;
    right: 24px;
    animation: float-note 2.1s ease-in-out infinite;
  }

  .note-two {
    left: 24px;
    bottom: 24px;
    font-size: 18px;
    animation: float-note 2.1s ease-in-out 0.45s infinite;
  }

  @keyframes note-bounce {
    0%, 100% {
      transform: translateY(0) scale(1);
    }
    45% {
      transform: translateY(-9px) scale(1.06);
    }
  }

  @keyframes ring-breathe {
    0%, 100% {
      box-shadow: 0 0 0 rgba(124, 58, 237, 0);
    }
    50% {
      box-shadow: 0 20px 48px rgba(124, 58, 237, 0.18);
    }
  }

  @keyframes pulse-ring {
    0%, 100% {
      opacity: 0.35;
      transform: scale(0.96);
    }
    50% {
      opacity: 0.78;
      transform: scale(1.08);
    }
  }

  @keyframes float-note {
    0%, 100% {
      opacity: 0.45;
      transform: translateY(0);
    }
    50% {
      opacity: 0.95;
      transform: translateY(-8px);
    }
  }

  .text-content {
    text-align: center;
  }

  .title {
    font-size: 24px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 8px;
    letter-spacing: -0.3px;
  }

  .subtitle {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 400;
  }

  .action-buttons {
    display: flex;
    gap: 16px;
  }

  .action-button {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 28px;
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.85);
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
  }

  .action-button:hover {
    background: linear-gradient(135deg, rgba(124, 58, 237, 0.3) 0%, rgba(99, 102, 241, 0.3) 100%);
    border-color: rgba(124, 58, 237, 0.4);
    transform: translateY(-2px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
    color: #fff;
  }

  .action-button:active {
    transform: translateY(0);
  }

  .features {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 8px;
  }

  .feature-item {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.4);
  }

  .feature-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: linear-gradient(135deg, #7c3aed 0%, #6366f1 100%);
    box-shadow: 0 0 8px rgba(124, 58, 237, 0.5);
  }
</style>
