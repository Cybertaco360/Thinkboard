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
import { ref } from 'vue';
import { Send } from 'lucide-vue-next'; // Let's try the simpler Send icon first

const inputText = ref('');

async function GeminiBackendQuery() {
  const response = await fetch('http://localhost:8080/generate', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ text: inputText.value })
  });

  if (response.ok) {
    const data = await response.json();
    console.log(data);
  } else {
    console.error('Error:', response.statusText);
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
  width: 100%;
  padding-right: 40px; /* Make room for the button */
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