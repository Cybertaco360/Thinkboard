<script setup>
import { reactive, ref } from 'vue';
import Rectangular from './components/rectangular.vue';
import LineConnector from './components/LineConnector.vue';
import allancolumn1 from './components/allancolumn1.vue';

const nodes = reactive([
  { id: 1, x: 320, y: 100, text: "Step 1" },
  { id: 2, x: 320, y: 250, text: "Step 2" },
  { id: 3, x: 320, y: 400, text: "Step 3" }
]);

const dragging = ref(null);
const offset = ref({ x: 0, y: 0 });

function onMouseDown(e, node) {
  dragging.value = node;
  offset.value = {
    x: e.clientX - node.x,
    y: e.clientY - node.y
  };
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
}

function onMouseMove(e) {
  if (dragging.value) {
    dragging.value.x = e.clientX - offset.value.x;
    dragging.value.y = e.clientY - offset.value.y;
  }
}

function onMouseUp() {
  dragging.value = null;
  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', onMouseUp);
}
</script>

<template>
  <div style="position:relative; min-height:100vh;">
    <allancolumn1 />
    <Rectangular
      v-for="node in nodes"
      :key="node.id"
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
      :key="node.id + '-line'"
      :rect1="{ x: node.x, y: node.y, width: 270, height: 100 }"
      :rect2="{ x: nodes[idx+1].x, y: nodes[idx+1].y, width: 270, height: 100 }"
      edge1="bottom"
      edge2="top"
    />
  </div>
</template>



<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  transition: margin 0.4s cubic-bezier(.77,0,.18,1);
}
.with-panel {
  margin-right: 170px; /* Half the width of the side panel for visual balance */
}
</style>