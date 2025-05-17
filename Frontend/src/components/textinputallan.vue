<template>
  <div class="input-wrapper">
    <input
      v-model="inputText"
      class="text-input"
      type="text"
      placeholder="let's create!"
    />
    <button class="send-button" @click="GeminiBackendQuery">
      <Send size="20" />
    </button>
  </div>
</template>

<script setup>
import { ref, defineEmits } from 'vue';
import { Send } from 'lucide-vue-next'; // Let's try the simpler Send icon first

const emit = defineEmits(['nodes-update']);
const inputText = ref('');

function validateNodeSchema(data) {
  // Handle both array and single object responses
  const nodeArray = Array.isArray(data) ? data : [data];
  
  return nodeArray.map((node, index) => ({
    node_id: node.node_id || index,
    x: node.x || 100,
    y: node.y || (100 + index * 150),
    text: node.text || 'Unknown',
    connected: Array.isArray(node.connected) ? node.connected : [],
    information: node.information || ''
  }));
}

async function GeminiBackendQuery() {
  try {
    const response = await fetch('http://localhost:8080/generate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ prompt: inputText.value })
    });

    if (response.ok) {
      const data = await response.json();
      const validatedNodes = validateNodeSchema(data);
      emit('nodes-update', validatedNodes);
      console.log('Validated nodes:', validatedNodes);
    } else {
      const error = await response.json();
      console.error('API Error:', error);
    }
  } catch (error) {
    console.error('Error processing response:', error);
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
  padding-right: 25px; /* Make room for the button */
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

.send-button:hover {
  color: #222;
}
</style>