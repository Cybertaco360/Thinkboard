<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  }
});

const emit = defineEmits(['update:modelValue']);

const searchInput = ref(props.modelValue);

watch(() => props.modelValue, (newVal) => {
  searchInput.value = newVal;
});

const updateSearch = () => {
  emit('update:modelValue', searchInput.value);
};

const clearSearch = () => {
  searchInput.value = '';
  emit('update:modelValue', '');
};
</script>

<template>
  <div class="search-container">
    <div class="search-bar">
      <span class="search-icon">🔍</span>
      <input 
        type="text"
        v-model="searchInput"
        @input="updateSearch"
        placeholder="Search nodes..."
        class="search-input"
      />
      <button 
        v-if="searchInput" 
        @click="clearSearch" 
        class="clear-button"
      >
        ✕
      </button>
    </div>
  </div>
</template>

<style scoped>
.search-container {
  position: absolute;
  top: 20px;
  right: 20px;
  z-index: 100;
  width: 300px;
}

.search-bar {
  display: flex;
  align-items: center;
  background-color: var(--panel-bg, white);
  border: 1px solid var(--border-color, #ddd);
  border-radius: 20px;
  padding: 8px 12px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  transition: all 0.3s;
}

.search-bar:focus-within {
  box-shadow: 0 0 0 2px var(--primary-color, #1976d2);
}

.search-icon {
  margin-right: 8px;
  color: #666;
}

.search-input {
  flex-grow: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: 14px;
  color: var(--text-color, #333);
}

.clear-button {
  background: none;
  border: none;
  cursor: pointer;
  color: #999;
  padding: 0 4px;
  font-size: 14px;
}

.clear-button:hover {
  color: #666;
}
</style>
