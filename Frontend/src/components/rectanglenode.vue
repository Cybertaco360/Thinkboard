<template>
  <div 
    class="rectangle" 
    :class="{ 'expanded': isExpanded, 'overlay-active': isExpanded }"
    @mousedown="handleMouseDown"
  >
    <div class="content">
      <div class="main-text">{{ text }}</div>
      <div class="information" v-if="isExpanded">
        {{ information }}
      </div>
    </div>
  </div>
  <div v-if="isExpanded" class="overlay" @click="toggleExpand"></div>
</template>

<script setup>
import { ref, defineProps } from 'vue';

const props = defineProps({
  text: { type: String, required: true },
  information: { type: String, required: true }
});

const isExpanded = ref(false);
const isDragging = ref(false);
let startX = 0;
let startY = 0;

const handleMouseDown = (e) => {
  startX = e.clientX;
  startY = e.clientY;
  
  const handleMouseUp = (e) => {
    const deltaX = Math.abs(e.clientX - startX);
    const deltaY = Math.abs(e.clientY - startY);
    
    // Only trigger click if mouse hasn't moved more than 5px
    if (deltaX < 5 && deltaY < 5) {
      toggleExpand();
    }
    
    document.removeEventListener('mouseup', handleMouseUp);
  };
  
  document.addEventListener('mouseup', handleMouseUp);
};

const toggleExpand = () => {
  isExpanded.value = !isExpanded.value;
  if (isExpanded.value) {
    document.body.style.overflow = 'hidden';
  } else {
    document.body.style.overflow = '';
  }
};
</script>

<style scoped>
.rectangle {
  width: 270px;
  height: 100px;
  background-color: #80A4F9;
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
  position: relative;
  z-index: 20;
  transition: all 0.3s ease;
  cursor: pointer;
  padding: 20px;
  overflow: hidden;
}

.rectangle.expanded {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 80vw;
  max-width: 800px;
  height: 80vh;
  z-index: 1000;
  background-color: #6690F8;
  box-shadow: 0 8px 32px rgba(0,0,0,0.2);
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  z-index: 999;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 15px;
  width: 100%;
  height: 100%;
}

.main-text {
  font-size: 1.1rem;
  font-weight: 600;
  color: white;
  text-align: center;
}

.rectangle.expanded .main-text {
  font-size: 1.5rem;
  margin-top: 20px;
}

.information {
  font-size: 0.9rem;
  color: #F0F4FF;
  overflow-y: auto;
  padding: 20px;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  max-height: calc(80vh - 100px);
  margin: 0 20px 20px 20px;
}

.information::-webkit-scrollbar {
  width: 6px;
}

.information::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
}
</style>