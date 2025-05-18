<template>
  <div class="input-wrapper">
    <input
      v-model="inputText"
      class="text-input"
      type="text"
      placeholder="let's create!"
      @keyup.enter="handleSubmit"
      :disabled="isLoading"
    />
    <button class="send-button" @click="handleSubmit" :disabled="isLoading">
      <span v-if="isLoading" class="loading-spinner"></span>
      <Send v-else size="20" />
    </button>
  </div>
</template>

<script setup>
import { ref, defineEmits } from 'vue';
import { Send } from 'lucide-vue-next';

const emit = defineEmits(['submit']);
const inputText = ref('');
const isLoading = ref(false);

async function handleSubmit() {
  if (!inputText.value.trim()) return;
  
  isLoading.value = true;
  try {
    // Send the user's text to the backend API
    const response = await fetch('http://127.0.0.1:8080/generate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      },
      body: JSON.stringify({ prompt: inputText.value.trim() }),
    });
    
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    
    // Use response.json() instead of text() to ensure proper JSON parsing
    const data = await response.json();
    console.log('Parsed API response:', data);
    
    // Ensure we're working with an array of nodes
    if (Array.isArray(data)) {
      // Check if we have an array of characters or actual node objects
      if (data.length > 0 && typeof data[0] === 'string') {
        console.error('Received array of characters instead of node objects:', data);
        emit('submit', [{
          node_id: Date.now(),
          x: 100,
          y: 100,
          text: 'Error: Invalid response format',
          information: 'Received text array instead of node objects',
          connected: [],
          category: 3
        }]);
      } else {
        // We have proper node objects
        emit('submit', data);
      }
    } else if (data && typeof data === 'object') {
      // Handle non-array object response
      if (data.error) {
        console.error('API error:', data.error);
      }
      // Try to convert to array if possible
      const nodeArray = data.nodes || [data];
      emit('submit', nodeArray);
    } else {
      // Fallback for any other unexpected format
      emit('submit', [{
        node_id: Date.now(),
        x: 100,
        y: 100,
        text: 'Unexpected Data Format',
        information: 'The API returned data in an unexpected format',
        connected: [],
        category: 3
      }]);
    }
  } catch (error) {
    console.error('Error submitting prompt:', error);
    // Create an error node to visualize the issue
    emit('submit', [{ 
      node_id: Date.now(),
      x: 100,
      y: 100,
      text: "Error processing request",
      connected: [],
      information: error.message,
      category: 3
    }]);
  } finally {
    inputText.value = ''; // Clear input after submission
    isLoading.value = false;
  }
}
</script>

<style scoped>
.rectangle {
  width: 270px;
  height: 100px;
  background-color: #fff;
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 10%;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
}

.text-input {
  width: 90%;
  padding-right: 25px;
  padding-left: 16px;
  padding-top:8px;
  padding-bottom: 8px;
  border: none;
  border-radius: 20px;
  font-size: 1.1rem;
  outline: none;
  background: #f5f7fa;
  color: #222;
}

.input-wrapper {
  position: relative;
  width: 100%;
}

.send-button {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  color: #666;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s;
}

.send-button:hover:not(:disabled) {
  color: #222;
}

.send-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.loading-spinner {
  display: inline-block;
  width: 20px;
  height: 20px;
  border: 2px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top-color: #666;
  animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>