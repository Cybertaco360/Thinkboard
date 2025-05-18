<script setup>
const emit = defineEmits(['close']);
</script>

<template>
  <div class="shortcuts-overlay" @click="$emit('close')">
    <div class="shortcuts-modal" @click.stop>
      <h2>Keyboard Shortcuts</h2>
      
      <div class="shortcuts-section">
        <h3>Navigation</h3>
        <div class="shortcut"><kbd>Space</kbd> + <kbd>Drag</kbd> Reset zoom and pan</div>
        <div class="shortcut"><kbd>Mouse Wheel</kbd> Zoom in/out</div>
        <div class="shortcut"><kbd>Drag Canvas</kbd> Pan the view</div>
      </div>
      
      <div class="shortcuts-section">
        <h3>Nodes</h3>
        <div class="shortcut"><kbd>Right-Click</kbd> Open context menu</div>
        <div class="shortcut"><kbd>Ctrl</kbd> + <kbd>A</kbd> Select all nodes</div>
        <div class="shortcut"><kbd>Shift</kbd> + <kbd>Click</kbd> Multi-select nodes</div>
        <div class="shortcut"><kbd>Delete</kbd> Delete selected nodes</div>
      </div>
      
      <div class="shortcuts-section">
        <h3>Connections</h3>
        <div class="shortcut"><kbd>Alt</kbd> + <kbd>Drag</kbd> Draw connection from node</div>
        <div class="shortcut"><kbd>Ctrl</kbd> + <kbd>L</kbd> Connect selected nodes</div>
      </div>
      
      <div class="shortcuts-section">
        <h3>Interface</h3>
        <div class="shortcut"><kbd>T</kbd> Toggle dark/light theme</div>
        <div class="shortcut"><kbd>M</kbd> Toggle minimap</div>
        <div class="shortcut"><kbd>Ctrl</kbd> + <kbd>Z</kbd> Undo</div>
        <div class="shortcut"><kbd>Ctrl</kbd> + <kbd>Y</kbd> Redo</div>
        <div class="shortcut"><kbd>?</kbd> Show/hide this help</div>
        <div class="shortcut"><kbd>Esc</kbd> Close dialogs/selection</div>
      </div>
      
      <button class="close-button" @click="$emit('close')">Close</button>
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
