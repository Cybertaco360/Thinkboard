<script setup>
import { onMounted, onUnmounted } from 'vue';

const props = defineProps({
  x: {
    type: Number,
    required: true
  },
  y: {
    type: Number,
    required: true
  },
  actions: {
    type: Array,
    required: true
  }
});

const emit = defineEmits(['select', 'close']);

const handleClickOutside = (event) => {
  const menu = document.querySelector('.context-menu');
  if (menu && !menu.contains(event.target)) {
    emit('close');
  }
};

const handleAction = (action) => {
  if (!action.disabled) {
    emit('select', action.id);
  }
};

// Adjust menu position to stay within viewport
const getAdjustedPosition = () => {
  const menuWidth = 200;
  const menuHeight = props.actions.length * 36 + 16;
  
  let adjustedX = props.x;
  let adjustedY = props.y;
  
  if (adjustedX + menuWidth > window.innerWidth) {
    adjustedX = window.innerWidth - menuWidth - 10;
  }
  
  if (adjustedY + menuHeight > window.innerHeight) {
    adjustedY = window.innerHeight - menuHeight - 10;
  }
  
  return { x: adjustedX, y: adjustedY };
};

const position = getAdjustedPosition();

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside);
});
</script>

<template>
  <div class="context-menu" :style="{ top: `${position.y}px`, left: `${position.x}px` }">
    <div 
      v-for="action in actions" 
      :key="action.id"
      class="menu-item"
      :class="{ disabled: action.disabled }"
      @click="handleAction(action)"
    >
      <span class="icon">{{ action.icon }}</span>
      {{ action.label }}
    </div>
  </div>
</template>

<style scoped>
.context-menu {
  position: fixed;
  width: 200px;
  background-color: var(--panel-bg, white);
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  padding: 8px 0;
  z-index: 1000;
  animation: fadeIn 0.15s ease-out;
  border: 1px solid var(--border-color, #ddd);
}

.menu-item {
  padding: 8px 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  color: var(--text-color, #333);
  transition: background-color 0.2s;
}

.menu-item:hover {
  background-color: var(--hover-bg, #f0f0f0);
}

.menu-item.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.icon {
  display: inline-block;
  width: 24px;
  margin-right: 8px;
  text-align: center;
}

@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}
</style>
