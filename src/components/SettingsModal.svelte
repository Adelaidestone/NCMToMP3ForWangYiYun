<script>
  import { FolderOpen, Save, X } from 'lucide-svelte'

  let {
    settings,
    error = '',
    onclose,
    onsave,
    onselectinput,
    onselectoutput
  } = $props()
</script>

<div class="modal-backdrop" role="presentation" onclick={onclose}>
  <div
    class="settings-modal"
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-title"
    tabindex="-1"
    onclick={(event) => event.stopPropagation()}
    onkeydown={(event) => event.stopPropagation()}
  >
    <header class="modal-header">
      <div>
        <h2 id="settings-title">设置</h2>
        <p>配置默认文件夹和启动自动转换。</p>
      </div>
      <button class="icon-button" onclick={onclose} aria-label="关闭设置">
        <X size={18} />
      </button>
    </header>

    <div class="settings-body">
      <div class="field-group">
        <div class="field-label">默认输入文件夹</div>
        <div class="path-row">
          <span class:empty={!settings.inputDir}>{settings.inputDir || '未选择'}</span>
          <button class="path-button" onclick={onselectinput}>
            <FolderOpen size={16} />
            选择
          </button>
        </div>
      </div>

      <div class="field-group">
        <div class="field-label">默认输出文件夹</div>
        <div class="path-row">
          <span class:empty={!settings.outputDir}>{settings.outputDir || '未选择'}</span>
          <button class="path-button" onclick={onselectoutput}>
            <FolderOpen size={16} />
            选择
          </button>
        </div>
      </div>

      <label class="toggle-row" class:disabled={!settings.inputDir || !settings.outputDir}>
        <div>
          <span>启动自动转换</span>
          <small>启动时检查默认输入文件夹，只转换输出目录中还没有同名 MP3 的 NCM。</small>
        </div>
        <input
          type="checkbox"
          checked={settings.autoConvertOnStart}
          disabled={!settings.inputDir || !settings.outputDir}
          onchange={(event) => settings.autoConvertOnStart = event.currentTarget.checked}
        />
      </label>

      {#if error}
        <p class="error-message">{error}</p>
      {/if}
    </div>

    <footer class="modal-actions">
      <button class="secondary-button" onclick={onclose}>取消</button>
      <button class="primary-button" onclick={onsave}>
        <Save size={16} />
        保存设置
      </button>
    </footer>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 20;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background: rgba(6, 8, 18, 0.72);
    backdrop-filter: blur(16px);
  }

  .settings-modal {
    width: min(620px, 100%);
    max-height: min(720px, calc(100vh - 48px));
    display: flex;
    flex-direction: column;
    overflow: hidden;
    color: rgba(255, 255, 255, 0.9);
    background: rgba(18, 20, 38, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 16px;
    box-shadow: 0 28px 90px rgba(0, 0, 0, 0.48);
  }

  .modal-header,
  .modal-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 20px 24px;
    border-color: rgba(255, 255, 255, 0.08);
  }

  .modal-header {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .modal-header p {
    margin: 4px 0 0;
    color: rgba(255, 255, 255, 0.48);
    font-size: 13px;
  }

  .icon-button {
    width: 34px;
    height: 34px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.7);
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    cursor: pointer;
  }

  .settings-body {
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding: 24px;
    overflow: auto;
  }

  .field-group {
    display: grid;
    gap: 8px;
  }

  .field-label,
  .toggle-row span {
    font-size: 13px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.78);
  }

  .path-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    min-height: 48px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }

  .path-row span {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.78);
  }

  .path-row span.empty {
    font-family: inherit;
    color: rgba(255, 255, 255, 0.38);
  }

  .path-button,
  .primary-button,
  .secondary-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-height: 36px;
    padding: 0 14px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .path-button,
  .secondary-button {
    color: rgba(255, 255, 255, 0.76);
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
  }

  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 18px;
    padding: 14px 16px;
    background: rgba(124, 58, 237, 0.1);
    border: 1px solid rgba(124, 58, 237, 0.18);
    border-radius: 10px;
  }

  .toggle-row.disabled {
    opacity: 0.58;
  }

  .toggle-row small {
    display: block;
    margin-top: 4px;
    color: rgba(255, 255, 255, 0.46);
    line-height: 1.45;
  }

  .toggle-row input {
    width: 18px;
    height: 18px;
    accent-color: #8b5cf6;
  }

  .error-message {
    margin: 0;
    padding: 10px 12px;
    color: #fecaca;
    background: rgba(248, 113, 113, 0.12);
    border: 1px solid rgba(248, 113, 113, 0.24);
    border-radius: 8px;
    font-size: 13px;
  }

  .modal-actions {
    justify-content: flex-end;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
  }

  .primary-button {
    color: #fff;
    background: linear-gradient(135deg, #7c3aed 0%, #6366f1 100%);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }
</style>
