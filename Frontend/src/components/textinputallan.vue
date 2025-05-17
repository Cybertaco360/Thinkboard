<template>
  <div class="rectangle">
    <input
      v-model="inputText"
      class="text-input"
      type="text"
      placeholder="let's create!"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue';

const inputText = ref('');
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
  padding-right: 12px;
  padding-left: 12px;
  padding-top:8px;
  padding-bottom: 8px;
  border: none;
  border-radius: 12px;
  font-size: 1.1rem;
  outline: none;
  background: #f5f7fa;
  color: #222;
}
</style>