<script setup>
const emit = defineEmits(['close']);

const shortcuts = [
  { keys: ['Ctrl', 'Z'], description: 'Undo last action' },
  { keys: ['Ctrl', 'Y'], description: 'Redo last undone action' },
  { keys: ['Ctrl', 'A'], description: 'Select all nodes' },
  { keys: ['Delete'], description: 'Delete selected node(s)' },
  { keys: ['Escape'], description: 'Clear selection / Close menus' },
  { keys: ['T'], description: 'Toggle light/dark theme' },
  { keys: ['M'], description: 'Toggle minimap' },
  { keys: ['?'], description: 'Show keyboard shortcuts' },
  { keys: ['Ctrl', 'Space'], description: 'Reset zoom and pan' },
  { keys: ['Shift', 'Click'], description: 'Add to selection' },
  { keys: ['Right-click'], description: 'Open context menu' },
  { keys: ['Mouse wheel'], description: 'Zoom in/out' },
];
</script>

<template>
  <div class="shortcuts-overlay" @click="$emit('close')">
    <div class="shortcuts-modal" @click.stop>
      <div class="modal-header">
        <h2>Keyboard Shortcuts</h2>
        <button class="close-button" @click="$emit('close')">✕</button>
      </div>
      <div class="shortcuts-list">
        <div v-for="(shortcut, index) in shortcuts" :key="index" class="shortcut-item">
          <div class="keys">
            <span v-for="(key, keyIndex) in shortcut.keys" :key="keyIndex" class="key">
              {{ key }}
              <span v-if="keyIndex < shortcut.keys.length - 1" class="key-plus">+</span>
            </span>
          </div>
          <div class="description">{{ shortcut.description }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.shortcuts-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 2000;
  display: flex;
  justify-content: center;
  align-items: center;
}

.shortcuts-modal {
  background-color: var(--panel-bg, white);
  border-radius: 10px;
  width: 500px;
  max-width: 90vw;
  max-height: 80vh;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  animation: modalAppear 0.3s ease-out;
}

.modal-header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #ddd);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  margin: 0;
  color: var(--text-color, #333);
  font-size: 18px;
  font-weight: 600;
}

.close-button {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: var(--text-color, #666);
}

.shortcuts-list {
  padding: 16px;
  max-height: 60vh;
  overflow-y: auto;
}

.shortcut-item {
  display: flex;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color, #eee);
  align-items: center;
}

.shortcut-item:last-child {
  border-bottom: none;
}

.keys {
  display: flex;
  align-items: center;
  width: 150px;
  flex-shrink: 0;
}

.key {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  background-color: var(--hover-bg, #f0f0f0);
  border: 1px solid var(--border-color, #ddd);
  border-radius: 4px;
  font-family: monospace;
  font-weight: bold;
  margin-right: 4px;
}

.key-plus {
  margin: 0 4px;
  font-weight: normal;
}

.description {
  color: var(--text-color, #333);
}

@keyframes modalAppear {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
