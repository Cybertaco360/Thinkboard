<script setup>
import { ref, reactive } from 'vue';
import Cloud from './cloud.vue';
import allancolumn1 from "./components/allancolumn1.vue";
import SidePan from './sidepan.vue';
import LogSign from './logsign.vue';

const showSidePan = ref(false);
const type = ref('signup');
const isLoggedIn = ref(false);

//  Schema: { node_id: #, x: X-COORDINATE, y: Y-COORDINATE, text: "TEXT THAT WILL BE DISPLAYED ON THE NODE", connected: [OTHER NODES TO BE CONNECTED TO], information: "Information at this certain point" }
const nodes = reactive([]);

const handleLogin = () => {
  isLoggedIn.value = true;
  showSidePan.value = false;
};

async function GeminiBackendQuery() {
  const response = await fetch('http://localhost:8080/generate', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ prompt: inputText.value }) // Use 'prompt' to match backend
  });

  if (response.ok) {
    const data = await response.json();
    // Assuming the response is an array or object matching your schema
    // Emit an event or update a global store, or directly update nodes if accessible
    // Example: this.$emit('nodes-update', data);
    // Or, if using a global store or prop:
    // nodes.value = Array.isArray(data) ? data : [data];
    // For demo, just log:
    console.log('Received nodes:', data);
    // If you want to update nodes in App.vue, emit an event:
    // emit('nodes-update', data);
  } else {
    console.error('Error:', response.statusText);
  }
}
</script>

<template>
  <div style="position:relative; min-height:100vh; width:100vw; overflow:hidden;">
    <template v-if="!isLoggedIn">
      <Cloud />
      <div :class="['main-content', { 'with-panel': showSidePan }]">
        <LogSign @open-sidepan="showSidePan = true; type = 'signup'" @open-signup="showSidePan = true; type = 'login'"/>
      </div>
      <SidePan :visible="showSidePan" :type="type" @close="showSidePan = false" @login-success="handleLogin()"/>
    </template>
    <template v-else>
      <div class="main-content">
        <allancolumn1 :nodes="nodes" @nodes-update="nodes.splice(0, nodes.length, ...$event)" />
      </div>
    </template>
  </div>
</template>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}
</style>