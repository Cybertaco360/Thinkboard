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
import { generateNodesFromPrompt } from '../services/api.service';
import nodeService from '../services/node.service';

const emit = defineEmits(['nodes-update', 'error']);
const inputText = ref('');
const isLoading = ref(false);

async function handleSubmit() {
  if (!inputText.value.trim() || isLoading.value) return;
  
  isLoading.value = true;
  try {
    const data = await generateNodesFromPrompt(inputText.value);
    const nodes = nodeService.addNodes(data);
    emit('nodes-update', nodes);
    inputText.value = ''; // Clear the input after submission
  } catch (error) {
    emit('error', `Failed to generate nodes: ${error.message}`);
    console.error('Error processing response:', error);
  } finally {
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