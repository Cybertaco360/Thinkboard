<script setup>
import rectangle from "./components/rectanglenode.vue";
import allancolumn1 from "./components/allancolumn1.vue";
</script>

<template>

  <div>
    <rectangle></rectangle>
    <allancolumn1></allancolumn1>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;

}
.logo:hover {
  filter: drop-shadow(0 0 2em #9fbaf5);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
