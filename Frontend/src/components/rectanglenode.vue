<template>
  <div class="node-wrapper">
    <div 
      class="node-container"
      :class="{ selected, hovered: isHovered }"
      :data-node-id="node.node_id"
      @mouseenter="isHovered = true"
      @mouseleave="isHovered = false"
    >
      <div class="category-indicator" :style="{ backgroundColor: color }">
        {{ category || 'Unknown' }}
      </div>
      <div class="node-title">{{ displayText }}</div>
      <div class="node-info-preview" @click.stop="toggleInfo">
        {{ displayInfo.length > 60 ? displayInfo.substring(0, 60) + '...' : displayInfo }}
        <button class="info-toggle" :class="{ 'is-open': showFullInfo }">
          {{ showFullInfo ? '▲' : '▼' }}
        </button>
      </div>
      <div v-if="showFullInfo" class="node-full-info" @click.stop>
        {{ displayInfo }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  node: {
    type: Object,
    required: true,
    validator: (value) => {
      if (typeof value !== 'object' || value === null) {
        console.error('Invalid node prop:', value);
        return false;
      }
      
      if (!value.hasOwnProperty('node_id') || !value.hasOwnProperty('text')) {
        console.error('Node missing required properties:', value);
        return false;
      }
      
      return true;
    }
  },
  text: {
    type: String,
    required: true,
    default: 'Untitled Node'
  },
  information: {
    type: String,
    default: ''
  },
  selected: {
    type: Boolean,
    default: false
  },
  category: {
    type: String,
    default: 'Task'
  },
  color: {
    type: String,
    default: '#666'
  }
});

// Add a computed property to handle possible undefined text
const displayText = computed(() => props.text || 'Untitled Node');
const displayInfo = computed(() => props.information || 'No additional information available');

// You can also add a fallback computed property for safe access
const nodeText = computed(() => {
  return props.node && typeof props.node === 'object' ? props.node.text : 'Invalid Node';
});

const isHovered = ref(false);
const showFullInfo = ref(false);

const toggleInfo = () => {
  showFullInfo.value = !showFullInfo.value;
};
</script>

<style scoped>
.node-wrapper {
  position: relative;
  width: 270px;
}

.node-container {
  border: 2px solid #666;
  border-radius: 10px;
  padding: 12px;
  width: 100%;
  background-color: var(--panel-bg, white);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  transition: transform 0.2s, box-shadow 0.2s, border-color 0.2s;
  overflow: hidden;
}

.node-container.hovered {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  transform: translateY(-3px);
}

.node-container.selected {
  box-shadow: 0 0 0 3px var(--primary-color, #1976d2);
  border-color: var(--primary-color, #1976d2);
}

.category-indicator {
  display: inline-block;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: bold;
  color: white;
  margin-bottom: 8px;
  text-shadow: 0 1px 1px rgba(0, 0, 0, 0.2);
}

.node-title {
  font-weight: bold;
  font-size: 16px;
  margin-bottom: 8px;
  color: var(--text-color, #333);
}

.node-info-preview {
  font-size: 13px;
  color: var(--text-color, #666);
  position: relative;
  padding-right: 20px;
  cursor: pointer;
  white-space: pre-wrap;
}

.info-toggle {
  position: absolute;
  right: 0;
  top: 0;
  background: none;
  border: none;
  font-size: 10px;
  color: var(--text-color, #666);
  cursor: pointer;
  opacity: 0.7;
  transition: transform 0.2s;
}

.info-toggle:hover {
  opacity: 1;
}

.info-toggle.is-open {
  transform: rotateX(180deg);
}

.node-full-info {
  margin-top: 8px;
  padding: 8px;
  background-color: var(--hover-bg, #f5f5f5);
  border-radius: 4px;
  font-size: 13px;
  color: var(--text-color, #333);
  white-space: pre-wrap;
  max-height: 200px;
  overflow-y: auto;
}
</style>