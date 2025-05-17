<script setup>
import { ref } from 'vue';
import Cloud from './cloud.vue';
import rectangle from "./components/rectanglenode.vue";
import allancolumn1 from "./components/allancolumn1.vue";
import SidePan from './sidepan.vue';
import LogSign from './logsign.vue';
const showSidePan = ref(false);
const signin = ref(false);
</script>


<template>
  <div style="position:relative; min-height:100vh; width:100vw; overflow:hidden;">
    <Cloud />
    <div :class="['main-content', { 'with-panel': showSidePan }]">
      <LogSign @open-sidepan="showSidePan = true" />
    </div>
    <SidePan :visible="showSidePan" @close="showSidePan = false" />
  </div>
</template>



<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  transition: margin 0.4s cubic-bezier(.77,0,.18,1);
}
.with-panel {
  margin-right: 170px; /* Half the width of the side panel for visual balance */
}
</style>