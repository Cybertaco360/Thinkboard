<template>
  <svg :width="svgWidth" :height="svgHeight" style="position:absolute; top:0; left:0; pointer-events:none; z-index:10;">
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
  edge1: { type: String, default: 'bottom' }, // 'top', 'bottom', 'left', 'right'
  edge2: { type: String, default: 'top' }
});

function getEdgeCenter(rect, edge) {
  switch (edge) {
    case 'left':   return { x: rect.x, y: rect.y + rect.height / 2 };
    case 'right':  return { x: rect.x + rect.width, y: rect.y + rect.height / 2 };
    case 'top':    return { x: rect.x + rect.width / 2, y: rect.y };
    case 'bottom': return { x: rect.x + rect.width / 2, y: rect.y + rect.height };
    default:       return { x: rect.x, y: rect.y };
  }
}

const start = computed(() => getEdgeCenter(props.rect1, props.edge1));
const end = computed(() => getEdgeCenter(props.rect2, props.edge2));
const svgWidth = computed(() => Math.max(start.value.x, end.value.x) + 50);
const svgHeight = computed(() => Math.max(start.value.y, end.value.y) + 50);
</script>