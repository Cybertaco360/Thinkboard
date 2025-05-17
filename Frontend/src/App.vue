<script setup>
import { ref } from 'vue';
import Cloud from './cloud.vue';
import allancolumn1 from "./components/allancolumn1.vue";
import SidePan from './sidepan.vue';
import LogSign from './logsign.vue';

const showSidePan = ref(false);
const type = ref('signup');
const isLoggedIn = ref(false);

const handleLogin = () => {
  isLoggedIn.value = true;
  showSidePan.value = false;
};
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
        <allancolumn1 />
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