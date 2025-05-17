<template>
  <transition name="slide">
    <div v-if="visible" class="side-panel">
      <button class="close-btn" @click="emit('close')" aria-label="Close">&times;</button>
      <h2 class="panel-title">{{ type === 'signup' ? 'Sign Up' : 'Log In' }}</h2>
      <form v-if="type === 'login'" @submit.prevent="handleLogin">
        <label>
          Email
          <input type="email" v-model="loginEmail" required />
        </label>
        <label>
          Password
          <input type="password" v-model="loginPassword" required />
        </label>
        <button class="confirm-btn" type="submit">Confirm</button>
      </form>
      <form v-else @submit.prevent="handleSignup">
        <label>
          Full Name
          <input type="text" v-model="signupName" required />
        </label>
        <label>
          Email
          <input type="email" v-model="signupEmail" required />
        </label>
        <label>
          Password
          <input type="password" v-model="signupPassword" required />
        </label>
        <label>
          Confirm Password
          <input type="password" v-model="signupPassword2" required />
        </label>
        <button class="confirm-btn" type="submit">Sign Up</button>
        <div v-if="signupError" class="error-msg">{{ signupError }}</div>
      </form>
    </div>
  </transition>
</template>

<script setup>
import { defineProps, defineEmits, ref } from 'vue';
import { useUserStore } from '@/stores/userStore';
const userStore = useUserStore();
const props = defineProps({ visible: Boolean, type: String });
const emit = defineEmits(['close']);

// Log In
const loginEmail = ref('');
const loginPassword = ref('');

// Sign Up
const signupName = ref('');
const signupEmail = ref('');
const signupPassword = ref('');
const signupPassword2 = ref('');
const signupError = ref('');

async function handleLogin() {
  const data = {
    email: loginEmail.value,
    password: loginPassword.value
  };
  const response = await fetch('http://localhost:8080/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });

  if (response.ok) {
    const result = await response.json();
    if (result.success) {
      // Store user info in Pinia
      userStore.login({
        username: result.username, // or result.name if that's what your backend returns
        email: result.email
      });
      emit('login-success');
      emit('close');
    } else {
      alert('Login failed: ' + result.message);
    }
  } else {
    alert('Network error: ' + response.statusText);
  }
}

async function handleSignup() {
  if (signupPassword.value !== signupPassword2.value) {
    signupError.value = "Passwords do not match!";
    return;
  }
  signupError.value = "";
  const data = {
    id: generateId(),
    name: signupName.value,
    email: signupEmail.value,
    password: signupPassword.value
  };
  downloadJSON(data, 'signup-info.json');
  emit('close');
}

function downloadJSON(data, filename) {
  const json = JSON.stringify(data, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

function generateId() {
  // Simple random ID (8 chars)
  return Math.random().toString(36).substring(2, 10);
}
function handleLoginSuccess(userData) {
  //userData: {username, email}
  useUserStore.login(userData)
  // emit login-success or navigate as needed
}


}
</script>

<style scoped>
.side-panel {
  position: fixed;
  top: 0;
  right: 0;
  width: 340px;
  height: 100%;
  background: #80A4F9;
  box-shadow: -2px 0 24px rgba(0,0,0,0.08);
  display: flex;
  flex-direction: column;
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

.error-msg {
  color: #ffd6d6;
  background: #b33a3a;
  border-radius: 8px;
  padding: 8px;
  margin-top: 10px;
  text-align: center;
  font-size: 1em;
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