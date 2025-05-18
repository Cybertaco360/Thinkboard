<template>
  <svg :width="svgWidth" :height="svgHeight" style="position:absolute; top:0; left:0; pointer-events:none; z-index:15;">
    <line
      :x1="start.x"
      :y1="start.y"
      :x2="end.x"
      :y2="end.y"
      stroke="#BACFFD"
      stroke-width="4"
      stroke-linecap="round"
    />
  </svg>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  rect1: { type: Object, required: true }, // { x, y, width, height }
  rect2: { type: Object, required: true }, // { x, y, width, height }
});

function getCenter(rect) {
  return {
    x: rect.x + rect.width / 2,
    y: rect.y + rect.height / 2
  };
}

const start = computed(() => getCenter(props.rect1));
const end = computed(() => getCenter(props.rect2));
const svgWidth = computed(() => Math.max(start.value.x, end.value.x) + 50);
const svgHeight = computed(() => Math.max(start.value.y, end.value.y) + 50);
</script>