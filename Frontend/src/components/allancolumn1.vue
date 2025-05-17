<template>
  <div :class="['left-column', { 'collapsed': isCollapsed }]">
    <button class="toggle-button" @click="isCollapsed = !isCollapsed">
      <ChevronLeft :class="{ 'rotate': !isCollapsed }" />
    </button>
    <img src="/public/thinkboard.png" alt="ThinkBoard Logo" class="logo" />
    <div class="ai-assistant">
      <Circularallan/>
    </div>

    <div class="nav-container">
      <!-- Navigation content can go here -->
    </div>

    <div class="input-bottom">
      <Textinputallan />
    </div>
  </div>

  <div>
    <div v-for="node in nodes" :key="node.node_id" class="node">
      <strong>{{ node.text }}</strong>
      <div>{{ node.information }}</div>
      <div>Connected to: {{ node.connected.join(', ') }}</div>
      <div>Position: ({{ node.x }}, {{ node.y }})</div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { ChevronLeft } from 'lucide-vue-next';
import Circularallan from './Circularallan.vue';
import Textinputallan from './textinputallan.vue';

defineProps({ nodes: Array });
const emit = defineEmits(['nodes-update']);
</script>

<style scoped>
.logo {
  position: relative;  /* changed from absolute */
  width: 170px;
  margin: 10px 0 0 10px;  /* add margin instead of absolute positioning */
  z-index: 101;
}

.left-column {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: #D4E1FE;
  border-top-right-radius: 32px;
  border-bottom-right-radius: 32px;
  box-shadow: 2px 0 12px rgba(0,0,0,0.08);
  display: flex;
  flex-direction: column;
  z-index: 100;
  transition: width 0.3s ease-in-out;
}

.left-column.collapsed {
  width: 400px;
}

.nav-container {
  flex: 1;
  padding: 20px 0;
  overflow-y: auto;
}

/* Add this to anchor the input at the bottom */
.input-bottom {
  padding: 24px 20px;
  margin-top: auto;
  display: flex;
  justify-content: center;
}

.ai-assistant {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px 0;
  margin-top: 0;  /* reduce space between logo and assistant */
}

.ai-circle {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: rgba(52, 152, 219, 0.2);
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  border: 2px solid rgba(52, 152, 219, 0.5);
}

.ai-pulse-container {
  width: 60%;
  height: 40%;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.ai-pulse {
  width: 4px;
  height: 20px;
  background-color: #3498db;
  border-radius: 4px;
  animation: pulse 1.4s ease-in-out infinite;
}

.ai-pulse:nth-child(1) { animation-delay: -1.2s; }
.ai-pulse:nth-child(2) { animation-delay: -0.9s; }
.ai-pulse:nth-child(3) { animation-delay: -0.6s; }
.ai-pulse:nth-child(4) { animation-delay: -0.3s; }
.ai-pulse:nth-child(5) { animation-delay: 0s; }

@keyframes pulse {
  0%, 100% {
    height: 10px;
    background-color: rgba(52, 152, 219, 0.6);
  }
  50% {
    height: 30px;
    background-color: #3498db;
  }
}

.ai-text {
  margin-top: 14px;
  font-size: 16px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.9);
  letter-spacing: 0.5px;
}
</style>