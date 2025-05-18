<script setup>
import { ref, reactive } from 'vue';
import Cloud from './cloud.vue';
import allancolumn1 from "./components/allancolumn1.vue";
import SidePan from './sidepan.vue';
import LogSign from './logsign.vue';
import Rectangular from './components/rectanglenode.vue'; // Changed from rectangle to Rectangular
import LineConnector from './components/LineConnector.vue';

const showSidePan = ref(false);
const type = ref('signup');
const isLoggedIn = ref(false);

//  Schema: { node_id: #, x: X-COORDINATE, y: Y-COORDINATE, text: "TEXT THAT WILL BE DISPLAYED ON THE NODE", connected: [OTHER NODES TO BE CONNECTED TO], information: "Information at this certain point" }
const nodes = reactive([]);

const handleLogin = () => {
  isLoggedIn.value = true;
  showSidePan.value = false;
};

// Add this function to handle mouse down events for dragging
const onMouseDown = (e, node) => {
  // Prevent default browser drag behavior
  e.preventDefault();
  
  const startX = e.clientX;
  const startY = e.clientY;
  const initialX = node.x;
  const initialY = node.y;
  
  const onMouseMove = (moveEvent) => {
    const dx = moveEvent.clientX - startX;
    const dy = moveEvent.clientY - startY;
    
    node.x = initialX + dx;
    node.y = initialY + dy;
  };
  
  const onMouseUp = () => {
    document.removeEventListener('mousemove', onMouseMove);
    document.removeEventListener('mouseup', onMouseUp);
  };
  
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', onMouseUp);
};

// Consider logging to verify the component is receiving props
console.log('Rendering nodes:', nodes);

// Add some initial test nodes to see if rendering works
isLoggedIn.value = true; // Auto-login for testing
// Debug
console.log('Initial nodes:', nodes);
</script>

<template>
  <div style="position:relative; min-height:100vh; width:100vw; overflow:hidden;">
    <template v-if="!isLoggedIn">
      <Cloud />
      <div :class="['main-content', { 'with-panel': showSidePan }]">
        <LogSign @open-sidepan="showSidePan = true; type = 'signup'" @open-signup="showSidePan = true; type = 'login'"/>
      </div>
      <SidePan :visible="showSidePan" :type="type" @close="showSidePan = false" @login-success="handleLogin()"/>
    </template>
    <template v-else>
      <div class="main-content">
        <allancolumn1 :nodes="nodes" @nodes-update="nodes.splice(0, nodes.length, ...$event)" />
        <div class="graph-scroll-container">
          <div class="graph-inner-area">
            <Rectangular
              v-for="node in nodes"
              :key="node.node_id"
              :style="{
                position: 'absolute',
                left: node.x + 'px',
                top: node.y + 'px',
                cursor: 'grab',
                userSelect: 'none'
              }"
              @mousedown="e => onMouseDown(e, node)"
            >
              {{ node.text }}
            </Rectangular>
            <LineConnector
              v-for="(node, idx) in nodes.slice(0, -1)"
              :key="node.node_id + '-line'"
              :rect1="{ x: node.x, y: node.y, width: 270, height: 100 }"
              :rect2="{ x: nodes[idx+1].x, y: nodes[idx+1].y, width: 270, height: 100 }"
              edge1="bottom"
              edge2="top"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}

.graph-scroll-container {
  width: 80vw;
  height: 80vh;
  border: 2px solid #b3e0ff;
  border-radius: 16px;
  overflow: auto;
  background: #f7fbff;
  position: relative;
  margin: 32px auto 0 auto;
}

.graph-inner-area {
  position: relative;
  width: 3000px;  /* Set large enough for your graph */
  height: 2000px; /* Set large enough for your graph */
}
</style>