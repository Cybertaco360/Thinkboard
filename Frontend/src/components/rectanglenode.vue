<template>
  <div class="rectangle">
    <!-- Add content here if needed -->
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
  const h = 300;
  const w = 300;
  const radius = 140;
  const center = {
    x: w / 2,
    y: h / 2
  };
  const circles = [];
  const rangeMin = 8;   // Bigger minimum for more pronounced waves
  const rangeMax = 22;  // Bigger maximum for more pronounced waves
  const showPoints = false;
  let tick = 0;
  let animationId;

  // Distinct blue gradients (not matching #BACFFD)
  const gradient1 = ctx.createLinearGradient(0, 0, w, 0);
  gradient1.addColorStop(0, '#1A2980'); // deep blue
  gradient1.addColorStop(1, '#26D0CE'); // teal blue

  const gradient2 = ctx.createLinearGradient(0, 0, w, 0);
  gradient2.addColorStop(0, '#134E5E'); // dark blue
  gradient2.addColorStop(1, '#71B280'); // blue-green

  const gradient3 = ctx.createLinearGradient(0, 0, w, 0);
  gradient3.addColorStop(0, '#0F2027'); // very dark blue
  gradient3.addColorStop(1, '#2C5364'); // steel blue

  const gradient4 = ctx.createLinearGradient(0, 0, w, 0);
  gradient4.addColorStop(0, '#396afc'); // vivid blue
  gradient4.addColorStop(1, '#2948ff'); // electric blue

  const gradients = [gradient1, gradient2, gradient3, gradient4];

  el.width = w * dpr;
  el.height = h * dpr;
  el.style.width = w + 'px';
  el.style.height = h + 'px';
  ctx.setTransform(1, 0, 0, 1, 0, 0); // reset transform
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
        let phase = 6 * Math.sin(tick / 50); // Even bigger and faster waves
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
.rectangle {
  width: 270px;
  height: 100px;
  background-color: #D4E1FE; /* Dark blue */
  border-radius: 20px; /* Rounded corners */
  display: flex;
  align-items: center;   /* Vertical centering */
  justify-content: center; /* Horizontal centering */
  margin: 10%;
  /* Remove padding if you want pure centering, or adjust as needed */
}
canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
</style>