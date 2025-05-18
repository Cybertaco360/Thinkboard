<template>
  <svg class="line-connector" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: -1;">
    <path
      :d="connection.pathD"
      fill="none"
      :stroke="color"
      stroke-width="2"
      :stroke-dasharray="animated ? '5,5' : 'none'"
      :class="{ 'animated-dash': animated }"
    />
  </svg>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  rect1: {
    type: Object,
    required: true
  },
  rect2: {
    type: Object,
    required: true
  },
  edge1: {
    type: String,
    default: 'auto'
  },
  edge2: {
    type: String,
    default: 'auto'
  },
  color: {
    type: String,
    default: '#333'
  },
  animated: {
    type: Boolean,
    default: false
  }
});

// Function to determine the best point to connect from on each rectangle
const calculateConnectionPoints = () => {
  const r1 = {
    left: props.rect1.x,
    top: props.rect1.y,
    right: props.rect1.x + props.rect1.width,
    bottom: props.rect1.y + props.rect1.height,
    centerX: props.rect1.x + props.rect1.width / 2,
    centerY: props.rect1.y + props.rect1.height / 2
  };
  
  const r2 = {
    left: props.rect2.x,
    top: props.rect2.y,
    right: props.rect2.x + props.rect2.width,
    bottom: props.rect2.y + props.rect2.height,
    centerX: props.rect2.x + props.rect2.width / 2,
    centerY: props.rect2.y + props.rect2.height / 2
  };
  
  let p1, p2;
  
  // If edges are specified, use them
  if (props.edge1 !== 'auto' && props.edge2 !== 'auto') {
    switch (props.edge1) {
      case 'top':
        p1 = { x: r1.centerX, y: r1.top };
        break;
      case 'right':
        p1 = { x: r1.right, y: r1.centerY };
        break;
      case 'bottom':
        p1 = { x: r1.centerX, y: r1.bottom };
        break;
      case 'left':
        p1 = { x: r1.left, y: r1.centerY };
        break;
      default:
        p1 = { x: r1.centerX, y: r1.centerY };
    }
    
    switch (props.edge2) {
      case 'top':
        p2 = { x: r2.centerX, y: r2.top };
        break;
      case 'right':
        p2 = { x: r2.right, y: r2.centerY };
        break;
      case 'bottom':
        p2 = { x: r2.centerX, y: r2.bottom };
        break;
      case 'left':
        p2 = { x: r2.left, y: r2.centerY };
        break;
      default:
        p2 = { x: r2.centerX, y: r2.centerY };
    }
    
    return { p1, p2 };
  }
  
  // Otherwise compute the best edges to connect from
  const dx = r2.centerX - r1.centerX;
  const dy = r2.centerY - r1.centerY;
  
  if (Math.abs(dx) > Math.abs(dy)) {
    // Connect horizontally
    p1 = { 
      x: dx > 0 ? r1.right : r1.left, 
      y: r1.centerY 
    };
    p2 = { 
      x: dx > 0 ? r2.left : r2.right, 
      y: r2.centerY 
    };
  } else {
    // Connect vertically
    p1 = { 
      x: r1.centerX, 
      y: dy > 0 ? r1.bottom : r1.top 
    };
    p2 = { 
      x: r2.centerX, 
      y: dy > 0 ? r2.top : r2.bottom 
    };
  }
  
  return { p1, p2 };
};

const connection = computed(() => {
  const { p1, p2 } = calculateConnectionPoints();
  
  // Compute control points for the bezier curve
  const dx = p2.x - p1.x;
  const dy = p2.y - p1.y;
  const distance = Math.sqrt(dx * dx + dy * dy);
  
  const cp1 = {
    x: p1.x + (dx / 2),
    y: p1.y
  };
  
  const cp2 = {
    x: p2.x - (dx / 2),
    y: p2.y
  };
  
  return {
    start: p1,
    end: p2,
    controlPoint1: cp1,
    controlPoint2: cp2,
    pathD: `M${p1.x},${p1.y} C${cp1.x},${cp1.y} ${cp2.x},${cp2.y} ${p2.x},${p2.y}`
  };
});
</script>

<style scoped>
.animated-dash {
  animation: dash 20s linear infinite;
}

@keyframes dash {
  to {
    stroke-dashoffset: -100;
  }
}
</style>