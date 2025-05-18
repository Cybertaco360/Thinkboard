<template>
  <div class="left-column" :class="{ 'collapsed': isThinking }">
    <div class="cinematic-overlay" :class="{ 'active': showOverlay }"></div>
    <img src="/public/thinkboard.png" alt="ThinkBoard Logo" class="logo" />
    <div class="ai-assistant">
      <Circularallan :pulse="isTransitioning"/>
      <div v-if="!isThinking" class="info-box" :class="{ 'fade-out': fadeAnswer, 'dramatic-entry': dramaticEntry }">
        <p class="info-text">{{ questions[currentQuestion] }}</p>
      </div>
      <div v-else class="thinking-box">
        <p class="thinking-text">Accompanying on your journey<span class="dots"><span>.</span><span>.</span><span>.</span></span></p>
      </div>
    </div>

    <div class="nav-container">
      <!-- Navigation content can go here -->
    </div>

    <div class="input-bottom" :class="{ 'glow': currentQuestion > 0 }">
      <Textinputallan @submit="handleAnswer" :placeholder="inputPlaceholders[currentQuestion]"/>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted, computed } from 'vue';
import Circularallan from './Circularallan.vue';
import Textinputallan from './textinputallan.vue';

defineProps({ nodes: Array });
const emit = defineEmits(['nodes-update']);

const questions = [
  "Welcome to ThinkBoard. Your journey into knowledge begins now...",
  "What quest for knowledge drives you forward? What do you seek to master?",
  "What foundation have you already built? Tell me of your experience in this realm.",
  "How much of your precious time can you devote to this pursuit? How many hours will you sacrifice?",
  "How deep will you venture into this subject? Will you merely scratch the surface, or plunge into its depths?",
  "How does your mind best absorb wisdom? Through practical application, theoretical understanding, or a blend of both?"
];

const inputPlaceholders = [
  "press enter to continue...",
  "your goal awaits...",
  "share your experience...",
  "time is precious...",
  "how deep will you go?",
  "how do you learn best?"
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
const isTransitioning = ref(false);
const showOverlay = ref(false);
const dramaticEntry = ref(true);

// Dramatic intro with timed overlay
onMounted(() => {
  showOverlay.value = true;
  setTimeout(() => {
    showOverlay.value = false;
    dramaticEntry.value = true;
    
    // Start countdown after intro
    startTimeout.value = setTimeout(() => {
      transitionToNextQuestion();
    }, 8000);
  }, 2000);
});

onUnmounted(() => {
  if (startTimeout.value) clearTimeout(startTimeout.value);
});

const transitionToNextQuestion = async () => {
  dramaticEntry.value = false;
  fadeAnswer.value = true;
  isTransitioning.value = true;
  
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  if (currentQuestion.value < questions.length - 1) {
    currentQuestion.value++;
    showOverlay.value = true;
    
    setTimeout(() => {
      showOverlay.value = false;
      fadeAnswer.value = false;
      dramaticEntry.value = true;
      isTransitioning.value = false;
    }, 500);
  }
};

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

  // Dramatic transition between questions
  if (currentQuestion.value < questions.length - 1) {
    await transitionToNextQuestion();
  } else {
    // Final dramatic transition to thinking state
    fadeAnswer.value = true;
    isTransitioning.value = true;
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Dramatic reveal of thinking animation
    showOverlay.value = true;
    await new Promise(resolve => setTimeout(resolve, 700));
    isThinking.value = true;
    showOverlay.value = false;
    
    // Cinematic pause before collapsing
    await new Promise(resolve => setTimeout(resolve, 3000));
    
    const finalPrompt = `Create a timeline or something of assistance for the following:
Goal: ${answers.goal}
Prior Experience: ${answers.experience}
Time Commitment: ${answers.timeCommitment}
Depth: ${answers.depth}
Learning Style: ${answers.style}`;
    
    try {
      // Make API call to get nodes
      const response = await fetch('http://127.0.0.1:8080/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify({ prompt: finalPrompt }),
      });
      
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      
      const data = await response.json();
      console.log('API response with nodes:', data);
      
      // Enhanced validation to ensure we have valid node objects
      if (Array.isArray(data) && data.length > 0) {
        // Check if first item is a node object or a character
        if (typeof data[0] === 'string' || !data[0].hasOwnProperty('node_id')) {
          console.error('API returned invalid node format:', data);
          // Return a fallback node instead
          emit('nodes-update', [{
            node_id: Date.now(),
            x: 100,
            y: 100,
            text: 'Invalid Node Format',
            information: 'The API returned data in an invalid format',
            connected: [],
            category: 3
          }]);
        } else {
          // We have valid node objects
          emit('nodes-update', data);
        }
      } else if (data && typeof data === 'object') {
        // Try to handle object response
        const nodeArray = data.nodes || [data];
        emit('nodes-update', nodeArray);
      } else {
        throw new Error('Unexpected response format');
      }
    } catch (error) {
      console.error('Error getting nodes from API:', error);
      // Create an error node if API call fails
      emit('nodes-update', [{
        node_id: Date.now(),
        x: 100,
        y: 100,
        text: "Error processing request",
        connected: [],
        information: error.message,
        category: 3
      }]);
    }
  }
};
</script>

<style scoped>
.logo {
  position: relative;
  width: 170px;
  margin: 10px 0 0 10px;
  z-index: 101;
  animation: pulse-glow 3s infinite alternate;
}

.left-column {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: #1a2035;
  background-image: radial-gradient(circle at 50% 50%, #2a3b5c, #1a2035);
  box-shadow: 0 0 30px rgba(50, 130, 255, 0.3);
  display: flex;
  flex-direction: column;
  z-index: 100;
  transition: width 0.8s cubic-bezier(0.65, 0, 0.35, 1);
  overflow: hidden;
}

.cinematic-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: none;
  background-color: black;
  opacity: 0;
  z-index: 150;
  pointer-events: none;
  transition: opacity 0.5s ease;
}

.cinematic-overlay.active {
  opacity: 0.7;
}

.nav-container {
  flex: 1;
  padding: 20px 0;
  overflow-y: auto;
}

.input-bottom {
  padding: 24px 20px;
  margin-top: auto;
  display: flex;
  justify-content: center;
  transition: box-shadow 0.5s ease;
}

.input-bottom.glow {
  box-shadow: 0 -5px 20px rgba(160, 196, 255, 0.582);
}

.ai-assistant {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 0;
  margin-top: 0;
}

.info-box {
  background: rgba(100, 170, 255, 0.4);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(100, 170, 255, 0.3);
  padding: 20px 25px;
  margin: 30px;
  width: 85%;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2), 0 0 20px rgba(50, 130, 255, 0.2);
  transition: all 0.6s cubic-bezier(0.33, 1, 0.68, 1);
  opacity: 0;
  transform: translateY(20px);
  animation: pulse-glow 3s infinite alternate;
}

.info-box.dramatic-entry {
  opacity: 1;
  transform: translateY(0);
}

.fade-out {
  opacity: 0 !important;
  transform: translateY(-20px) !important;
  transition: opacity 0.8s cubic-bezier(0.33, 1, 0.68, 1), transform 0.8s cubic-bezier(0.33, 1, 0.68, 1);
}

.info-text {
  color: #e0f0ff;
  font-size: 16px;
  text-align: center;
  line-height: 1.6;
  margin: 0;
  font-weight: 300;
  letter-spacing: 0.5px;
  text-shadow: 0 1px 5px rgba(0, 0, 0, 0.5);
}

.thinking-box {
  background: rgba(30, 50, 80, 0.6);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(100, 170, 255, 0.3);
  border-radius: 16px;
  padding: 20px 25px;
  margin: 30px;
  width: 85%;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2), 0 0 30px rgba(50, 130, 255, 0.3);
  animation: pulse-box 2s infinite alternate;
}

.thinking-text {
  color: #e0f0ff;
  font-size: 16px;
  text-align: center;
  line-height: 1.4;
  margin: 0;
  font-weight: 500;
  letter-spacing: 0.5px;
}

.dots span {
  animation: dots 1.5s infinite;
  opacity: 0;
  display: inline-block;
  font-size: 20px;
}

.dots span:nth-child(1) { animation-delay: 0.0s; }
.dots span:nth-child(2) { animation-delay: 0.3s; }
.dots span:nth-child(3) { animation-delay: 0.6s; }

@keyframes dots {
  0% { opacity: 0; }
  50% { opacity: 1; }
  100% { opacity: 0; }
}

.left-column.collapsed {
  width: 400px;
  transition: width 1.2s cubic-bezier(0.65, 0, 0.35, 1);
}

@keyframes pulse-glow {
  0% { filter: drop-shadow(0 0 2px rgba(50, 130, 255, 0.3)); }
  100% { filter: drop-shadow(0 0 8px rgba(50, 130, 255, 0.7)); }
}

@keyframes pulse-box {
  0% { box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2), 0 0 20px rgba(50, 130, 255, 0.2); }
  100% { box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2), 0 0 40px rgba(50, 130, 255, 0.5); }
}
</style>