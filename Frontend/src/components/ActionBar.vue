<script setup>
const props = defineProps({
  isDark: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits([
  'undo',
  'redo',
  'toggle-theme',
  'toggle-minimap',
  'show-keyboard-shortcuts',
  'logout' // Add logout emit
]);
</script>

<template>
  <div class="action-bar">
    <button class="action-button" @click="$emit('undo')" title="Undo (Ctrl+Z)">
      <span class="icon">↩️</span>
    </button>
    <button class="action-button" @click="$emit('redo')" title="Redo (Ctrl+Y)">
      <span class="icon">↪️</span>
    </button>
    <div class="divider"></div>
    <button class="action-button" @click="$emit('toggle-theme')" title="Toggle Theme (T)">
      <span class="icon">{{ isDark ? '☀️' : '🌙' }}</span>
    </button>
    <button class="action-button" @click="$emit('toggle-minimap')" title="Toggle Minimap (M)">
      <span class="icon">🗺️</span>
    </button>
    <div class="divider"></div>
    <button class="action-button" @click="$emit('show-keyboard-shortcuts')" title="Keyboard Shortcuts (?)">
      <span class="icon">⌨️</span>
    </button>
    <div class="divider"></div>
    <button class="action-button logout-button" @click="$emit('logout')" title="Logout">
      <span class="icon">🚪</span>
    </button>
  </div>
</template>

<style scoped>
.action-bar {
  position: absolute;
  top: 20px;
  left: 20px;
  display: flex;
  background-color: var(--panel-bg, white);
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  z-index: 100;
  padding: 4px;
  border: 1px solid var(--border-color, #ddd);
}

.action-button {
  background: none;
  border: none;
  border-radius: 4px;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  margin: 0 2px;
  transition: background-color 0.2s;
}

.action-button:hover {
  background-color: var(--hover-bg, #f0f0f0);
}

.divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color, #ddd);
  margin: 0 4px;
}

.icon {
  font-size: 18px;
}

.logout-button {
  background-color: var(--hover-bg, #f0f0f0);
}

.logout-button:hover {
  background-color: var(--error-bg, #ffebee);
  color: var(--error-color, #d32f2f);
}
</style>
