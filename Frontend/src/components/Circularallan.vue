<template>
  <div class="canvas-wrapper">
    <canvas ref="canvasRef"></canvas>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';

const canvasRef = ref(null);

onMounted(() => {
  const el = canvasRef.value;
  const ctx = el.getContext('2d');
  const dpr = window.devicePixelRatio || 1;
  const pi = Math.PI;
  const points = 12;
  const h = 240;      // Increased from 200
  const w = 240;      // Increased from 200
  const radius = 100; // Increased from 80
  const center = {
    x: w / 2,
    y: h / 2
  };
  const circles = [];
  const rangeMin = 6;
  const rangeMax = 10;
  const showPoints = false;
  let tick = 0;
  let animationId;

  // Distinct blue gradients (not matching #BACFFD)
  const gradient1 = ctx.createLinearGradient(0, 0, w, 0);
  gradient1.addColorStop(0, '#A7C7E7');
  gradient1.addColorStop(1, '#B4D8F8');

  const gradient2 = ctx.createLinearGradient(0, 0, w, 0);
  gradient2.addColorStop(0, '#7FB3FF');
  gradient2.addColorStop(1, '#D6E6FF');

  const gradient3 = ctx.createLinearGradient(0, 0, w, 0);
  gradient3.addColorStop(0, '#9AB8FF');
  gradient3.addColorStop(1, '#E3F0FF');

  const gradient4 = ctx.createLinearGradient(0, 0, w, 0);
  gradient4.addColorStop(0, '#6C8CD5');
  gradient4.addColorStop(1, '#B8CFFF');

  const gradients = [gradient1, gradient2, gradient3, gradient4];

  el.width = w * dpr;
  el.height = h * dpr;
  el.style.width = w + 'px';
  el.style.height = h + 'px';
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.scale(dpr, dpr);

  // Setup swing circle points
  for (let idx = 0; idx < gradients.length; idx++) {
    let swingpoints = [];
    let radian = 0;

    for (let i = 0; i < points; i++) {
      radian = pi * 2 / points * i;
      const ptX = center.x + radius * Math.cos(radian);
      const ptY = center.y + radius * Math.sin(radian);

      swingpoints.push({
        x: ptX,
        y: ptY,
        radian: radian,
        range: Math.random() * (rangeMax - rangeMin) + rangeMin,
        phase: 0
      });
    }

    circles.push(swingpoints);
  }

  // Function to draw the swing circle
  const swingCircle = () => {
    ctx.clearRect(0, 0, w * dpr, h * dpr);
    ctx.globalAlpha = 1;
    ctx.globalCompositeOperation = 'screen';

    for (let k = 0; k < circles.length; k++) {
      let swingpoints = circles[k];

      for (let i = 0; i < swingpoints.length; i++) {
        swingpoints[i].phase += (Math.random() * 9 + 1) * -0.01;
        let phase = 3.6 * Math.sin(tick / 150); // smaller waves
        const r = radius + (swingpoints[i].range * phase * Math.sin(swingpoints[i].phase)) - rangeMax / 2;
        swingpoints[i].radian += pi / 360;
        const ptX = center.x + r * Math.cos(swingpoints[i].radian);
        const ptY = center.y + r * Math.sin(swingpoints[i].radian);

        swingpoints[i] = {
          x: ptX,
          y: ptY,
          radian: swingpoints[i].radian,
          range: swingpoints[i].range,
          phase: swingpoints[i].phase,
        };
      }

      const fill = gradients[k];
      drawCurve(swingpoints, fill);
    }

    tick++;
    animationId = requestAnimationFrame(swingCircle);
  };

  swingCircle();

  function drawCurve(pts, fillStyle) {
    ctx.fillStyle = fillStyle;
    ctx.beginPath();
    ctx.moveTo(
      (pts[cycle(-1, points)].x + pts[0].x) / 2,
      (pts[cycle(-1, points)].y + pts[0].y) / 2
    );
    for (let i = 0; i < pts.length; i++) {
      ctx.quadraticCurveTo(
        pts[i].x,
        pts[i].y,
        (pts[i].x + pts[cycle(i + 1, points)].x) / 2,
        (pts[i].y + pts[cycle(i + 1, points)].y) / 2
      );
    }
    ctx.closePath();
    ctx.fill();
  }

  function cycle(num1, num2) {
    return (num1 % num2 + num2) % num2;
  }

  onUnmounted(() => {
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
  });
});
</script>

<style scoped>
.canvas-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 260px; /* slightly larger than canvas for padding */
  width: 100%;
}
canvas {
  display: block;
  border-radius: 50%;
  background: transparent;
}
</style>