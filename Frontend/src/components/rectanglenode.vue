<template>
  <div class="rectangle">
    <slot />
  </div>
</template>

<script setup>
// No script needed unless you want props for content
</script>

<style scoped>
.rectangle {
  width: 270px;
  height: 100px;
  background-color: #80A4F9;
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
}
</style>