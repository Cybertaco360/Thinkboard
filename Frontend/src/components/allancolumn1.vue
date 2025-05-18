<template>
  <div class="left-column" :class="{ 'collapsed': isThinking }">
    <img src="/public/thinkboard.png" alt="ThinkBoard Logo" class="logo" />
    <div class="ai-assistant">
      <Circularallan/>
      <div v-if="!isThinking" class="info-box" :class="{ 'fade-out': fadeAnswer }">
        <p class="info-text">{{ questions[currentQuestion] }}</p>
      </div>
      <div v-else class="thinking-box">
        <p class="thinking-text">Thinking<span class="dots"><span>.</span><span>.</span><span>.</span></span></p>
      </div>
    </div>

    <div class="nav-container">
      <!-- Navigation content can go here -->
    </div>

    <div class="input-bottom">
      <Textinputallan @submit="handleAnswer"/>
    </div>
  </div>

  <div class="nodes-container">
    <rectangle
      v-for="node in nodes"
      :key="node.node_id"
      :node="node"
      :style="{
        position: 'absolute',
        left: `${node.x+350}px`,
        top: `${node.y}px`
      }"
    />
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue';
import Circularallan from './Circularallan.vue';
import Textinputallan from './textinputallan.vue';
import rectangle from './rectanglenode.vue';

defineProps({ nodes: Array });
const emit = defineEmits(['nodes-update']);

const questions = [
  "Welcome to ThinkBoard! Your journey will begin in 10 seconds...",
  "What's your main goal or topic you want to explore?",
  "Do you have any prior experience with this topic? Please be specific about what you already know.",
  "How much time can you dedicate to learning this topic? (e.g., hours per week)",
  "How deep would you like to dive into this topic? (beginner/intermediate/advanced)",
  "What's your preferred learning style? (practical examples, theoretical concepts, or both?)"
];

const answers = reactive({
  goal: '',
  experience: '',
  timeCommitment: '',
  depth: '',
  style: ''
});

const currentQuestion = ref(0);
const fadeAnswer = ref(false);
const isThinking = ref(false);
const startTimeout = ref(null);

// Start 10 second countdown on mount
onMounted(() => {
  startTimeout.value = setTimeout(() => {
    currentQuestion.value++;
    fadeAnswer.value = false;
  }, 10000);
});

onUnmounted(() => {
  if (startTimeout.value) clearTimeout(startTimeout.value);
});

const handleAnswer = async (answer) => {
  // Store answer based on current question
  switch(currentQuestion.value) {
    case 0: // Welcome message - just proceed
      break;
    case 1:
      answers.goal = answer;
      break;
    case 2:
      answers.experience = answer;
      break;
    case 3:
      answers.timeCommitment = answer;
      break;
    case 4:
      answers.depth = answer;
      break;
    case 5:
      answers.style = answer;
      break;
  }

  // Trigger fade effect
  fadeAnswer.value = true;
  await new Promise(resolve => setTimeout(resolve, 1000));

  if (currentQuestion.value < questions.length - 1) {
    // Move to next question
    currentQuestion.value++;
    fadeAnswer.value = false;
  } else {
    // Show thinking animation
    isThinking.value = true;
    
    // Wait 2 seconds before collapsing column
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Wait 2 more seconds before calling API
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    const finalPrompt = `Create a mind map for the following:
Goal: ${answers.goal}
Prior Experience: ${answers.experience}
Time Commitment: ${answers.timeCommitment}
Depth: ${answers.depth}
Learning Style: ${answers.style}`;
    
    emit('nodes-update', finalPrompt);
  }
};
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
  width: 100vw;  /* Changed from 400px to 100vw */
  height: 100vh;
  background-color: #D4E1FE;
  border-top-right-radius: 32px;
  border-bottom-right-radius: 32px;
  box-shadow: 2px 0 12px rgba(0,0,0,0.07); 
  display: flex;
  flex-direction: column;
  z-index: 100;
  transition: width 0.5s ease; /* Add smooth transition */
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

.nodes-container {
  position: fixed;
  top: 0;
  left: 100vw; /* Changed from 400px to follow full width */
  width: 0; /* Initially hidden */
  height: 100vh;
  overflow: hidden;
  pointer-events: none;
  transition: all 0.3s ease; /* Add smooth transition */
}

.nodes-container > * {
  pointer-events: auto;
}

/* Add these classes for when nodes are present */
.left-column.with-nodes {
  width: 400px;
}

.nodes-container.with-nodes {
  left: 400px;
  width: calc(100% - 400px);
}

.info-box {
  background: white;
  border-radius: 16px;
  padding: 15px 20px;
  margin: 20px;
  width: 80%;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  transition: opacity 0.5s ease;
}

.fade-out {
  opacity: 0;
  transform: translateY(-10px);
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.fade-in {
  opacity: 1;
  transform: translateY(0);
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.info-text {
  color: #4A5568;
  font-size: 14px;
  text-align: center;
  line-height: 1.4;
  margin: 0;
}

.thinking-box {
  background: white;
  border-radius: 16px;
  padding: 15px 20px;
  margin: 20px;
  width: 80%;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.thinking-text {
  color: #4A5568;
  font-size: 14px;
  text-align: center;
  line-height: 1.4;
  margin: 0;
}

.dots span {
  animation: dots 1.5s infinite;
  opacity: 0;
  display: inline-block;
}

.dots span:nth-child(1) { animation-delay: 0.0s; }
.dots span:nth-child(2) { animation-delay: 0.3s; }
.dots span:nth-child(3) { animation-delay: 0.6s; }

@keyframes dots {
  0% { opacity: 0; }
  50% { opacity: 1; }
  100% { opacity: 0; }
}

.left-column {
  /* ... existing styles ... */
  transition: width 0.5s ease;
}

.left-column.collapsed {
  width: 400px;
}

/* ... rest of existing styles ... */
</style>