<script>
  import { Check, Loader2, Trash2, Music, FileWarning } from 'lucide-svelte'

  let { files, onremove } = $props()
</script>

<div class="file-list-container">
  <div class="file-list-header">
    <span class="header-label">文件名</span>
    <span class="header-label right">状态</span>
  </div>
  
  <div class="file-list-scroll">
    {#each files as file (file.path)}
      <div class="file-item" class:processing={file.status === 'processing'}>
        <div class="file-info">
          <div class="file-icon">
            {#if file.status === 'success'}
              <Check class="icon success" size={20} />
            {:else if file.status === 'error'}
              <FileWarning class="icon error" size={20} />
            {:else if file.status === 'processing'}
              <Loader2 class="icon loading" size={20} />
            {:else}
              <Music class="icon pending" size={20} />
            {/if}
          </div>
          
          <div class="file-details">
            <span class="file-name">{file.name}</span>
            {#if file.error}
              <span class="file-error">{file.error}</span>
            {/if}
          </div>
        </div>

        <div class="file-status">
          {#if file.status === 'processing'}
            <div class="progress-wrapper">
              <div class="progress-bar">
                <div 
                  class="progress-fill" 
                  style="width: {file.progress}%"
                ></div>
              </div>
              <span class="progress-text">{Math.round(file.progress)}%</span>
            </div>
          {:else if file.status === 'success'}
            <span class="status-badge success">已完成</span>
          {:else if file.status === 'error'}
            <span class="status-badge error">失败</span>
          {:else}
            <span class="status-badge pending">等待中</span>
          {/if}

          {#if file.status !== 'processing'}
            <button 
              class="remove-button" 
              onclick={() => onremove(file.path)}
              title="移除"
            >
              <Trash2 size={16} />
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .file-list-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 20px;
    overflow: hidden;
    backdrop-filter: blur(20px);
  }

  .file-list-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .header-label {
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .header-label.right {
    padding-right: 40px;
  }

  .file-list-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .file-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.04);
    border-radius: 14px;
    margin-bottom: 8px;
    transition: all 0.3s ease;
  }

  .file-item:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .file-item.processing {
    background: rgba(124, 58, 237, 0.08);
    border-color: rgba(124, 58, 237, 0.3);
    animation: pulse-soft 2s ease-in-out infinite;
  }

  @keyframes pulse-soft {
    0%, 100% {
      box-shadow: 0 0 0 0 rgba(124, 58, 237, 0.2);
    }
    50% {
      box-shadow: 0 0 20px 4px rgba(124, 58, 237, 0.15);
    }
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 14px;
    flex: 1;
    min-width: 0;
  }

  .file-icon {
    width: 44px;
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    flex-shrink: 0;
  }

  :global(.icon) {
    transition: all 0.3s ease;
  }

  :global(.icon.pending) {
    color: rgba(255, 255, 255, 0.4);
  }

  :global(.icon.success) {
    color: #34d399;
  }

  :global(.icon.error) {
    color: #f87171;
  }

  :global(.icon.loading) {
    color: #fbbf24;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .file-details {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .file-name {
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.85);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-error {
    font-size: 12px;
    color: #f87171;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-status {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
  }

  .progress-wrapper {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .progress-bar {
    width: 120px;
    height: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #7c3aed, #6366f1);
    border-radius: 4px;
    transition: width 0.3s ease;
    position: relative;
  }

  .progress-fill::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.3),
      transparent
    );
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }

  .progress-text {
    font-size: 12px;
    font-weight: 600;
    color: #a78bfa;
    min-width: 40px;
    text-align: right;
  }

  .status-badge {
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 20px;
    letter-spacing: 0.3px;
  }

  .status-badge.pending {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .status-badge.success {
    background: rgba(52, 211, 153, 0.15);
    color: #34d399;
    border: 1px solid rgba(52, 211, 153, 0.3);
  }

  .status-badge.error {
    background: rgba(248, 113, 113, 0.15);
    color: #f87171;
    border: 1px solid rgba(248, 113, 113, 0.3);
  }

  .remove-button {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.3);
    cursor: pointer;
    transition: all 0.2s ease;
    opacity: 0;
  }

  .file-item:hover .remove-button {
    opacity: 1;
  }

  .remove-button:hover {
    background: rgba(248, 113, 113, 0.15);
    color: #f87171;
  }
</style>
