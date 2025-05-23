<script setup>
import { ref, reactive, onMounted, computed, watch } from 'vue';
import Cloud from './cloud.vue';
import allancolumn1 from "./components/allancolumn1.vue";
import SidePan from './sidepan.vue';
import LogSign from './logsign.vue';
import Rectangular from './components/rectanglenode.vue';
import LineConnector from './components/LineConnector.vue';
import MiniMap from './components/MiniMap.vue';
import ContextMenu from './components/ContextMenu.vue';
import SearchBar from './components/SearchBar.vue';
import ActionBar from './components/ActionBar.vue';
import KeyboardShortcutsHelp from './components/KeyboardShortcutsHelp.vue';
import nodeService from './services/node.service';
import { login, signup } from './services/api.service';
// UI state
const showSidePan = ref(false);
const type = ref('signup');
const isLoggedIn = ref(false);
const isDarkTheme = ref(false);
const showContextMenu = ref(false);
const contextMenuPosition = reactive({ x: 0, y: 0 });
const showKeyboardShortcuts = ref(false);
const zoomLevel = ref(1);
const panOffset = reactive({ x: 0, y: 0 });
const isDraggingCanvas = ref(false);
const lastMousePosition = reactive({ x: 0, y: 0 });
const searchQuery = ref('');
const selectedNodeIds = ref([]);
const showMiniMap = ref(false);
const errorMessage = ref('');
const showError = ref(false);

// New state variables for line drawing
const isDrawingLine = ref(false);
const drawingLineSource = ref(null);
const drawingLineTarget = ref({ x: 0, y: 0 });

// Viewport dimensions
const viewportWidth = ref(0);
const viewportHeight = ref(0);

// Current node categories with colors
const nodeCategories = reactive([
  { id: 1, name: 'Task', color: '#4CAF50' },
  { id: 2, name: 'Milestone', color: '#2196F3' },
  { id: 3, name: 'Decision', color: '#FF9800' },
  { id: 4, name: 'Note', color: '#9C27B0' },
]);

// Access nodes from service
const nodes = nodeService.nodes;

// Access nodes from service
const nodes = nodeService.nodes;

// Computed properties
const filteredNodes = computed(() => {
  if (!searchQuery.value) return nodes;
  return nodes.filter(node => 
    node.text.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    (node.information && node.information.toLowerCase().includes(searchQuery.value.toLowerCase()))
  );
});

const transformStyle = computed(() => {
  return `transform: translate(${panOffset.x}px, ${panOffset.y}px) scale(${zoomLevel.value})`;
});

// Theme management
watch(isDarkTheme, (newValue) => {
  document.body.classList.toggle('dark-theme', newValue);
  localStorage.setItem('darkTheme', newValue);
});

// Authentication handlers
const handleLogin = async (credentials) => {
  try {
    const result = await login(credentials.email, credentials.password);
    if (result.success) {
      isLoggedIn.value = true;
      showSidePan.value = false;
      nodeService.addSampleNodes();
    } else {
      showError.value = true;
      errorMessage.value = result.message;
    }
  } catch (error) {
    showError.value = true;
    errorMessage.value = error.message;
  }
};

const handleSignup = async (userData) => {
  try {
    const result = await signup(userData.name, userData.email, userData.password);
    if (result.success) {
      isLoggedIn.value = true;
      showSidePan.value = false;
      nodeService.addSampleNodes();
    } else {
      showError.value = true;
      errorMessage.value = result.message;
    }
  } catch (error) {
    showError.value = true;
    errorMessage.value = error.message;
  }
};

// Mouse event handlers
const onMouseDown = (e, node) => {
  if (!node || typeof node !== 'object' || !node.node_id) {
    console.error('Invalid node object:', node);
    return;
  }

  if (e.button !== 0) return; // Only handle left clicks
  e.preventDefault();
  
  // If Alt key is pressed, start drawing a line from this node
  if (e.altKey) {
    isDrawingLine.value = true;
    drawingLineSource.value = node;
    drawingLineTarget.value = { 
      x: e.clientX - panOffset.x, 
      y: e.clientY - panOffset.y 
    };
    
    // Handle mouse move for line drawing
    const onMouseMove = (moveEvent) => {
      drawingLineTarget.value = { 
        x: moveEvent.clientX - panOffset.x, 
        y: moveEvent.clientY - panOffset.y 
      };
    };
    
    // Handle mouse up to complete line drawing
    const onMouseUp = (upEvent) => {
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
      
      // Find if we're over another node
      const targetElement = document.elementFromPoint(upEvent.clientX, upEvent.clientY);
      const targetNodeElement = targetElement?.closest('.node-container');
      
      if (targetNodeElement) {
        // Get the node ID from the element
        const targetNodeId = parseInt(targetNodeElement.dataset.nodeId);
        const targetNode = nodes.find(n => n.node_id === targetNodeId);
        
        if (targetNode && targetNode !== drawingLineSource.value) {
          // Add the connection
          if (!drawingLineSource.value.connected.includes(targetNode.node_id)) {
            nodeService.saveState(); // Save current state for undo
            drawingLineSource.value.connected.push(targetNode.node_id);
          }
        }
      }
      
      isDrawingLine.value = false;
      drawingLineSource.value = null;
    };
    
    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
    return;
  }
  
  // Add selection with shift key
  if (e.shiftKey) {
    toggleNodeSelection(node.node_id);
    return;
  }
  
  // Clear selection if clicking on a non-selected node
  if (!selectedNodeIds.value.includes(node.node_id)) {
    clearSelection();
  }
  
  // Select the current node
  if (!selectedNodeIds.value.includes(node.node_id)) {
    selectedNodeIds.value.push(node.node_id);
  }
  
  const startX = e.clientX;
  const startY = e.clientY;
  const initialPositions = selectedNodeIds.value.map(id => {
    const node = nodes.find(n => n.node_id === id);
    return { id, x: node.x, y: node.y };
  });
  
  const onMouseMove = (moveEvent) => {
    const dx = (moveEvent.clientX - startX) / zoomLevel.value;
    const dy = (moveEvent.clientY - startY) / zoomLevel.value;
    
    // Move all selected nodes
    initialPositions.forEach(pos => {
      const node = nodes.find(n => n.node_id === pos.id);
      node.x = pos.x + dx;
      node.y = pos.y + dy;
    });
  };
  
  const onMouseUp = () => {
    document.removeEventListener('mousemove', onMouseMove);
    document.removeEventListener('mouseup', onMouseUp);
    
    // Save state for undo/redo
    nodeService.saveState();
  };
  
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', onMouseUp);
};

const toggleNodeSelection = (nodeId) => {
  const index = selectedNodeIds.value.indexOf(nodeId);
  if (index === -1) {
    selectedNodeIds.value.push(nodeId);
  } else {
    selectedNodeIds.value.splice(index, 1);
  }
};

const clearSelection = () => {
  selectedNodeIds.value = [];
};

const onCanvasMouseDown = (e) => {
  if (e.target.classList.contains('graph-inner-area')) {
    if (e.button === 0) { // Left click
      isDraggingCanvas.value = true;
      lastMousePosition.x = e.clientX;
      lastMousePosition.y = e.clientY;
      
      document.body.style.cursor = 'grabbing';
      clearSelection();
    } else if (e.button === 2) { // Right click
      showContextMenu.value = true;
      contextMenuPosition.x = e.clientX;
      contextMenuPosition.y = e.clientY;
      e.preventDefault();
    }
  }
};

const onCanvasMouseMove = (e) => {
  if (isDraggingCanvas.value) {
    const dx = e.clientX - lastMousePosition.x;
    const dy = e.clientY - lastMousePosition.y;
    
    panOffset.x += dx;
    panOffset.y += dy;
    
    lastMousePosition.x = e.clientX;
    lastMousePosition.y = e.clientY;
  }
};

const onCanvasMouseUp = () => {
  isDraggingCanvas.value = false;
  document.body.style.cursor = 'default';
};

// Add method to create connections between nodes
const connectSelectedNodes = () => {
  if (selectedNodeIds.value.length < 2) return;
  
  nodeService.connectNodes(selectedNodeIds.value);
};

// Context menu actions
const handleContextMenuAction = (action) => {
  showContextMenu.value = false;
  
  if (action === 'createNode') {
    nodeService.createNode(contextMenuPosition.x, contextMenuPosition.y, zoomLevel.value, panOffset);
  } else if (action === 'createTask') {
    nodeService.createNode(contextMenuPosition.x, contextMenuPosition.y, zoomLevel.value, panOffset, 1);
  } else if (action === 'createMilestone') {
    nodeService.createNode(contextMenuPosition.x, contextMenuPosition.y, zoomLevel.value, panOffset, 2);
  } else if (action === 'createDecision') {
    nodeService.createNode(contextMenuPosition.x, contextMenuPosition.y, zoomLevel.value, panOffset, 3);
  } else if (action === 'createNote') {
    nodeService.createNode(contextMenuPosition.x, contextMenuPosition.y, zoomLevel.value, panOffset, 4);
  } else if (action === 'connect' && selectedNodeIds.value.length >= 2) {
    connectSelectedNodes();
  } else if (action === 'disconnect' && selectedNodeIds.value.length >= 2) {
    nodeService.disconnectNodes(selectedNodeIds.value);
  } else if (action === 'delete' && selectedNodeIds.value.length > 0) {
    nodeService.deleteNodes(selectedNodeIds.value);
    selectedNodeIds.value = [];
  }
};

// Zoom and keyboard controls
const handleZoom = (delta) => {
  const zoomFactor = delta > 0 ? 1.1 : 0.9;
  const newZoom = Math.min(Math.max(zoomLevel.value * zoomFactor, 0.25), 3);
  zoomLevel.value = newZoom;
};

const handleKeyDown = (e) => {
  // Ignore keyboard shortcuts when user is typing in an input
  if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
    return;
  }
  
  // Keyboard shortcuts
  if (e.ctrlKey || e.metaKey) {
    if (e.key === 'z') {
      e.preventDefault();
      nodeService.undo();
    } else if (e.key === 'y' || (e.shiftKey && e.key === 'z')) {
      e.preventDefault();
      nodeService.redo();
    } else if (e.key === 'a') {
      e.preventDefault();
      selectedNodeIds.value = nodes.map(node => node.node_id);
    } else if (e.key === ' ') {
      e.preventDefault();
      zoomLevel.value = 1;
      panOffset.x = 0;
      panOffset.y = 0;
    } else if (e.key === 'l' && selectedNodeIds.value.length >= 2) {
      e.preventDefault();
      connectSelectedNodes();
    }
  } else {
    if (e.key === 'Delete' && selectedNodeIds.value.length > 0) {
      e.preventDefault();
      nodeService.deleteNodes(selectedNodeIds.value);
      selectedNodeIds.value = [];
    } else if (e.key === 'Escape') {
      e.preventDefault();
      clearSelection();
      showContextMenu.value = false;
      showKeyboardShortcuts.value = false;
    } else if (e.key === '?') {
      e.preventDefault();
      showKeyboardShortcuts.value = !showKeyboardShortcuts.value;
    } else if (e.key === 'm') {
      e.preventDefault();
      showMiniMap.value = !showMiniMap.value;
    } else if (e.key === 't') {
      e.preventDefault();
      toggleTheme();
    }
  }
};

const toggleTheme = () => {
  isDarkTheme.value = !isDarkTheme.value;
};

const handleError = (message) => {
  errorMessage.value = message;
  showError.value = true;
  setTimeout(() => {
    showError.value = false;
  }, 5000);
};

// Init
onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
  
  // Set initial viewport dimensions
  updateViewportDimensions();
  
  // Listen for window resize events
  window.addEventListener('resize', updateViewportDimensions);
  
  // Load theme preference
  const savedTheme = localStorage.getItem('darkTheme');
  if (savedTheme) {
    isDarkTheme.value = savedTheme === 'true';
  } else {
    // Check system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    isDarkTheme.value = prefersDark;
  }
});
</script>

<template>
  <div :class="['app-container', { 'dark-theme': isDarkTheme }]">
    <template v-if="!isLoggedIn">
      <Cloud />
      <div :class="['main-content', { 'with-panel': showSidePan }]">
        <LogSign @open-sidepan="showSidePan = true; type = 'signup'" @open-signup="showSidePan = true; type = 'login'"/>
      </div>
      <SidePan 
        :visible="showSidePan" 
        :type="type" 
        @close="showSidePan = false" 
        @login-success="handleLogin"
        @signup-success="handleSignup"
      />
    </template>
    <template v-else>
      <div class="main-content">
        <ActionBar 
          @undo="nodeService.undo" 
          @redo="nodeService.redo" 
          @toggle-theme="toggleTheme" 
          @toggle-minimap="showMiniMap = !showMiniMap"
          @show-keyboard-shortcuts="showKeyboardShortcuts = true"
          :is-dark="isDarkTheme"
        />
        
        <SearchBar v-model="searchQuery" />
        
        <allancolumn1 
          :nodes="filteredNodes" 
          :categories="nodeCategories"
          @nodes-update="(newNodes) => { nodeService.saveState(); nodes.splice(0, nodes.length, ...newNodes); }" 
          @error="handleError"
        />
        
        <div 
          class="graph-scroll-container" 
          @mousedown="onCanvasMouseDown"
          @mousemove="onCanvasMouseMove"
          @mouseup="onCanvasMouseUp"
          @wheel="(e) => handleZoom(e.deltaY)"
          @contextmenu.prevent
        >
          <div class="graph-inner-area" :style="transformStyle">
            <Rectangular
              v-for="node in filteredNodes"
              :key="node.node_id"
              :style="{
                position: 'absolute',
                left: node.x + 'px',
                top: node.y + 'px',
                userSelect: 'none',
                transition: 'box-shadow 0.2s, transform 0.2s',
                transform: selectedNodeIds.includes(node.node_id) ? 'scale(1.05)' : 'scale(1)',
                boxShadow: selectedNodeIds.includes(node.node_id) ? '0 0 10px rgba(0, 123, 255, 0.8)' : 'none',
                borderColor: nodeCategories.find(cat => cat.id === node.category)?.color || '#666'
              }"
              :text="node.text"
              :information="node.information"
              :category="nodeCategories.find(cat => cat.id === node.category)?.name || ''"
              :color="nodeCategories.find(cat => cat.id === node.category)?.color || '#666'"
              @mousedown.stop="e => onMouseDown(e, node)"
              @dblclick.stop="e => { /* Handle node edit */ }"
              :selected="selectedNodeIds.includes(node.node_id)"
              :node="node"
              :data-node-id="node.node_id"
              class="node-container"
            />
            
            <!-- Lines between nodes with animated dashed lines -->
            <template v-for="node in filteredNodes" :key="`${node.node_id}-connections`">
              <template v-if="node.connected && node.connected.length > 0">
                <template v-for="connectedId in node.connected" :key="`${node.node_id}-${connectedId}`">
                  <LineConnector
                    v-if="nodes.find(n => n.node_id === connectedId)"
                    :rect1="{ 
                      x: node.x, 
                      y: node.y, 
                      width: 270, 
                      height: 100 
                    }"
                    :rect2="{ 
                      x: nodes.find(n => n.node_id === connectedId).x, 
                      y: nodes.find(n => n.node_id === connectedId).y, 
                      width: 270, 
                      height: 100 
                    }"
                    :color="nodeCategories.find(cat => cat.id === node.category)?.color || '#333'"
                    :animated="true"
                  />
                </template>
              </template>
            </template>
            
            <!-- Drawing line preview -->
            <svg v-if="isDrawingLine && drawingLineSource" class="line-preview" 
              style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000;">
              <line 
                :x1="drawingLineSource.x + 135" 
                :y1="drawingLineSource.y + 50"
                :x2="drawingLineTarget.x / zoomLevel"
                :y2="drawingLineTarget.y / zoomLevel"
                stroke="var(--primary-color)"
                stroke-width="2"
                stroke-dasharray="5,5"
              />
            </svg>
          </div>
          
          <!-- Zoom controls -->
          <div class="zoom-controls">
            <button @click="handleZoom(-1)" title="Zoom In">+</button>
            <button @click="zoomLevel = 1; panOffset.x = 0; panOffset.y = 0" title="Reset Zoom">
              {{ Math.round(zoomLevel * 100) }}%
            </button>
            <button @click="handleZoom(1)" title="Zoom Out">-</button>
          </div>
          
          <!-- MiniMap -->
          <MiniMap 
            v-if="showMiniMap"
            :nodes="nodes" 
            :viewport="miniMapViewport"
            :panOffset="panOffset"
            :zoomLevel="zoomLevel"
            @pan-to="(x, y) => { panOffset.x = x; panOffset.y = y; }"
          />
        </div>
        
        <!-- Context Menu with connect/disconnect options -->
        <ContextMenu
          v-if="showContextMenu"
          :x="contextMenuPosition.x"
          :y="contextMenuPosition.y"
          :actions="[
            { id: 'createTask', label: 'New Task', icon: '📋' },
            { id: 'createMilestone', label: 'New Milestone', icon: '🏁' },
            { id: 'createDecision', label: 'New Decision', icon: '❓' },
            { id: 'createNote', label: 'New Note', icon: '📝' },
            { id: 'connect', label: 'Connect Selected', icon: '🔗', disabled: selectedNodeIds.length < 2 },
            { id: 'disconnect', label: 'Remove Connection', icon: '✂️', disabled: selectedNodeIds.length < 2 },
            { id: 'delete', label: 'Delete Selected', icon: '🗑️', disabled: selectedNodeIds.length === 0 },
          ]"
          @select="handleContextMenuAction"
          @close="showContextMenu = false"
        />
        
        <!-- Error Toast -->
        <div v-if="showError" class="error-toast">
          {{ errorMessage }}
          <button @click="showError = false" class="close-error">×</button>
        </div>
        
        <!-- Keyboard Shortcuts Help -->
        <KeyboardShortcutsHelp 
          v-if="showKeyboardShortcuts" 
          @close="showKeyboardShortcuts = false"
        />
      </div>
    </template>
  </div>
</template>
<style scoped>
.app-container {
  --primary-color: #1976d2;
  --bg-color: #f7fbff;
  --text-color: #333;
  --border-color: #b3e0ff;
  --panel-bg: white;
  --hover-bg: #e3f2fd;
  --error-bg: #ffebee;
  --error-color: #d32f2f;
}

.app-container.dark-theme {
  --primary-color: #64b5f6;
  --bg-color: #1e1e1e;
  --text-color: #f0f0f0;
  --border-color: #2c5282;
  --panel-bg: #2d2d2d;
  --hover-bg: #383838;
  --error-bg: #4a1515;
  --error-color: #ff6b6b;
}

.error-toast {
  position: fixed;
  top: 20px;
  right: 20px;
  background: var(--error-bg);
  color: var(--error-color);
  padding: 12px 20px;
  border-radius: 8px;
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.2);
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 10px;
  max-width: 400px;
}

.close-error {
  background: none;
  border: none;
  color: var(--error-color);
  cursor: pointer;
  font-size: 20px;
  padding: 0 5px;
}

/* Animation for connection lines */
@keyframes dash {
  to {
    stroke-dashoffset: -20;
  }
}
</style>