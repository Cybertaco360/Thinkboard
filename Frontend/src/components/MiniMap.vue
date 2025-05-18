<script setup>
import { computed, ref, watch } from 'vue';

const props = defineProps({
  nodes: {
    type: Array,
    required: true
  },
  viewport: {
    type: Object,
    required: true
  },
  panOffset: {
    type: Object,
    required: true
  },
  zoomLevel: {
    type: Number,
    required: true
  }
});

const emit = defineEmits(['pan-to']);

// Minimap constants
const MINIMAP_WIDTH = 200;
const MINIMAP_HEIGHT = 150;
const PADDING = 20;

// Calculate the bounds of all nodes
const nodeBounds = computed(() => {
  if (props.nodes.length === 0) {
    return { minX: 0, maxX: 3000, minY: 0, maxY: 2000 };
  }
  
  return props.nodes.reduce((bounds, node) => {
    bounds.minX = Math.min(bounds.minX, node.x);
    bounds.maxX = Math.max(bounds.maxX, node.x + 270); // Node width
    bounds.minY = Math.min(bounds.minY, node.y);
    bounds.maxY = Math.max(bounds.maxY, node.y + 100); // Node height
    return bounds;
  }, { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity });
});

// Calculate the scale factor for the minimap
const scale = computed(() => {
  const boundsWidth = nodeBounds.value.maxX - nodeBounds.value.minX + 2 * PADDING;
  const boundsHeight = nodeBounds.value.maxY - nodeBounds.value.minY + 2 * PADDING;
  
  const scaleX = MINIMAP_WIDTH / boundsWidth;
  const scaleY = MINIMAP_HEIGHT / boundsHeight;
  
  return Math.min(scaleX, scaleY);
});

// Calculate the minimap viewport position
const viewportRect = computed(() => {
  const boundsMinX = nodeBounds.value.minX - PADDING;
  const boundsMinY = nodeBounds.value.minY - PADDING;
  
  const viewX = (-props.panOffset.x / props.zoomLevel - boundsMinX) * scale.value;
  const viewY = (-props.panOffset.y / props.zoomLevel - boundsMinY) * scale.value;
  const viewWidth = props.viewport.width / props.zoomLevel * scale.value;
  const viewHeight = props.viewport.height / props.zoomLevel * scale.value;
  
  return {
    x: viewX,
    y: viewY,
    width: viewWidth,
    height: viewHeight
  };
});

// Calculate node positions on minimap
const nodePositions = computed(() => {
  const boundsMinX = nodeBounds.value.minX - PADDING;
  const boundsMinY = nodeBounds.value.minY - PADDING;
  
  return props.nodes.map(node => {
    return {
      id: node.node_id,
      x: (node.x - boundsMinX) * scale.value,
      y: (node.y - boundsMinY) * scale.value,
      width: 270 * scale.value,
      height: 100 * scale.value,
      category: node.category
    };
  });
});

// Handle clicks on the minimap
const handleMinimapClick = (event) => {
  const rect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  
  const boundsMinX = nodeBounds.value.minX - PADDING;
  const boundsMinY = nodeBounds.value.minY - PADDING;
  
  // Calculate the center position in the graph coordinates
  const graphX = x / scale.value + boundsMinX;
  const graphY = y / scale.value + boundsMinY;
  
  // Calculate the pan offset
  const centerX = -graphX * props.zoomLevel + props.viewport.width / 2;
  const centerY = -graphY * props.zoomLevel + props.viewport.height / 2;
  
  emit('pan-to', centerX, centerY);
};

// Dragging the viewport on minimap
const isDragging = ref(false);
const startDragPos = ref({ x: 0, y: 0 });

const startDrag = (event) => {
  isDragging.value = true;
  startDragPos.value = {
    x: event.clientX - viewportRect.value.x,
    y: event.clientY - viewportRect.value.y
  };
  
  window.addEventListener('mousemove', onDrag);
  window.addEventListener('mouseup', stopDrag);
};

const onDrag = (event) => {
  if (!isDragging.value) return;
  
  const rect = document.querySelector('.minimap').getBoundingClientRect();
  const x = event.clientX - rect.left - startDragPos.value.x;
  const y = event.clientY - rect.top - startDragPos.value.y;
  
  const boundsMinX = nodeBounds.value.minX - PADDING;
  const boundsMinY = nodeBounds.value.minY - PADDING;
  
  // Calculate the center position in the graph coordinates
  const graphX = x / scale.value + boundsMinX;
  const graphY = y / scale.value + boundsMinY;
  
  // Calculate the pan offset
  const centerX = -graphX * props.zoomLevel + props.viewport.width / 2;
  const centerY = -graphY * props.zoomLevel + props.viewport.height / 2;
  
  emit('pan-to', centerX, centerY);
};

const stopDrag = () => {
  isDragging.value = false;
  window.removeEventListener('mousemove', onDrag);
  window.removeEventListener('mouseup', stopDrag);
};
</script>

<template>
  <div class="minimap" @mousedown="handleMinimapClick">
    <!-- Node representations -->
    <div
      v-for="node in nodePositions"
      :key="`minimap-node-${node.id}`"
      class="minimap-node"
      :style="{
        left: `${node.x}px`,
        top: `${node.y}px`,
        width: `${node.width}px`,
        height: `${node.height}px`,
        backgroundColor: node.category === 1 ? '#4CAF50' : 
                         node.category === 2 ? '#2196F3' : 
                         node.category === 3 ? '#FF9800' : 
                         node.category === 4 ? '#9C27B0' : '#666'
      }"
    ></div>
    
    <!-- Viewport indicator -->
    <div 
      class="minimap-viewport"
      :style="{
        left: `${viewportRect.x}px`,
        top: `${viewportRect.y}px`,
        width: `${viewportRect.width}px`,
        height: `${viewportRect.height}px`,
      }"
      @mousedown.stop="startDrag"
    ></div>
  </div>
</template>

<style scoped>
.minimap {
  position: absolute;
  bottom: 20px;
  left: 20px;
  width: 200px;
  height: 150px;
  background-color: rgba(255, 255, 255, 0.8);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  overflow: hidden;
  z-index: 100;
}

.minimap-node {
  position: absolute;
  background-color: rgba(0, 0, 0, 0.6);
  border-radius: 2px;
}

.minimap-viewport {
  position: absolute;
  border: 2px solid #ff3e00;
  background-color: rgba(255, 62, 0, 0.1);
  cursor: move;
}

:deep(.dark-theme) .minimap {
  background-color: rgba(40, 40, 40, 0.8);
  border-color: #444;
}
</style>
