<template>
  <transition name="slide">
    <div v-if="visible" class="side-panel">
      <button class="close-btn" @click="emit('close')" aria-label="Close">&times;</button>
      <h2 class="panel-title">Log In</h2>
      <form @submit.prevent="handleSubmit">
        <label>
          Email
          <input type="email" v-model="email" required />
        </label>
        <label>
          Password
          <input type="password" v-model="password" required />
        </label>
        <button class="confirm-btn" type="submit">Confirm</button>
      </form>
    </div>
  </transition>
</template>

<script setup>
import { defineProps, defineEmits, ref } from 'vue';
const props = defineProps({ visible: Boolean });
const emit = defineEmits(['close']);

const email = ref('');
const password = ref('');

function handleSubmit() {
  // Create a JSON object with the form data
  const data = {
    email: email.value,
    password: password.value
  };

  // Convert to JSON string
  const json = JSON.stringify(data, null, 2);

  // Create a blob and trigger download
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);

  const a = document.createElement('a');
  a.href = url;
  a.download = 'login-info.json';
  a.click();
  URL.revokeObjectURL(url);

  emit('close');
}
</script>

<style scoped>
.side-panel {
  position: fixed;
  top: 0;
  right: 0;
  width: 340px;
  height: 100vh;
  background: #80A4F9;
  box-shadow: -2px 0 24px rgba(0,0,0,0.08);
  display: flex;
  flex-direction: column;
  padding: 36px 32px;
  z-index: 100;
  border-top-left-radius: 32px;
  border-bottom-left-radius: 32px;
}

.close-btn {
  position: absolute;
  top: 18px;
  right: 18px;
  background: none;
  border: none;
  font-size: 2em;
  color: #888;
  cursor: pointer;
  z-index: 101;
  transition: color 0.2s;
}
.close-btn:hover {
  color: #222;
}

.panel-title {
  color: #fff;
  font-size: 2em;
  margin-bottom: 24px;
  text-align: center;
  font-weight: bold;
}

form {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

label {
  color: #fff;
  font-weight: 500;
  font-size: 1.1em;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

input {
  padding: 8px 12px;
  border-radius: 8px;
  border: none;
  background: #e6f7ff;
  color: #29304d;
  font-size: 1em;
  margin-top: 4px;
}

input:focus {
  outline: 2px solid #5c7ecb;
}

.confirm-btn {
  margin-top: 18px;
  padding: 10px 0;
  background: #5c7ecb;
  color: #fff;
  border: none;
  border-radius: 10px;
  font-size: 1.1em;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.2s;
}

.confirm-btn:hover {
  background: #29304d;
}

/* Slide transition */
.slide-enter-active, .slide-leave-active {
  transition: transform 0.4s cubic-bezier(.77,0,.18,1);
}
.slide-enter-from, .slide-leave-to {
  transform: translateX(100%);
}
.slide-enter-to, .slide-leave-from {
  transform: translateX(0);
}
</style>