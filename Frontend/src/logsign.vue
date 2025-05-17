<template>
  <div class="background">
    <div class="clouds-bg">
      <img src="/public/cloud.png" class="cloud cloud1" alt="cloud" />
      <img src="/public/cloud.png" class="cloud cloud2" alt="cloud" />
      <img src="/public/cloud.png" class="cloud cloud3" alt="cloud" />
    </div>
    <div class="center-container">
      <img src="/public/thinkboard.png" alt="ThinkBoard Logo" class="logo" />
      <div class="rectangles-container">
        <div class="rounded-rectangle" @click="$emit('open-signup')">
          <div class="bottom-label">Log In</div>
        </div>
        <div class="rounded-rectangle" @click="$emit('open-sidepan')">
          <div class="bottom-label">Sign Up</div>
        </div>
      </div>
      <div class="fat-arrow-container" @click="scrollDown">
        <svg class="fat-arrow" width="80" height="60" viewBox="0 0 80 60" fill="none" xmlns="http://www.w3.org/2000/svg">
          <polyline points="15,25 40,50 65,25" stroke="#80A4F9" stroke-width="16" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
        </svg>
      </div>
    </div>
    <div class="demo-section">
      <h2>Demo Video</h2>
    </div>
    <div class="spacer"></div>
  </div>
</template>

<script setup>
function scrollDown() {
  window.scrollTo({
    top: window.scrollY + window.innerHeight,
    left: 0,
    behavior: 'smooth'
  });
}
</script>

<style scoped>
.background {
  min-height: 220vh;
  width: 100vw;
  background: linear-gradient(to bottom, #D4E1FE 0%, #b5cfff 100%);
  position: relative;
  overflow-x: hidden;
}

.clouds-bg {
  position: absolute;
  top: 0; left: 0; width: 100vw; height: 220vh;
  pointer-events: none;
  z-index: 0;
}

.cloud {
  position: absolute;
  opacity: 0.8;
  width: 180px;
  height: auto;
  animation: cloud-move 60s linear infinite;
}

.cloud1 {
  top: 10vh; left: 5vw; animation-duration: 60s;
}
.cloud2 {
  top: 30vh; left: 60vw; width: 220px; opacity: 0.6; animation-duration: 80s;
}
.cloud3 {
  top: 60vh; left: 30vw; width: 140px; opacity: 0.5; animation-duration: 100s;
}

@keyframes cloud-move {
  0% { transform: translateX(0); }
  100% { transform: translateX(60vw); }
}

.center-container {
  min-height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  position: relative;
  z-index: 1;
  padding-top: 80px;
  gap: 56px;
}

.logo {
  width: 440px;
  margin-bottom: 24px;
  margin-top: 0;
  padding: 0;
  z-index: 2;
  position: static;
  display: block;
}

.rectangles-container {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 36px;
}

.rounded-rectangle {
  width: 180px;
  height: 70px;
  background: #80A4F9;
  border-radius: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  transition: background 0.2s, box-shadow 0.2s, transform 0.2s;
  cursor: pointer;
}

.rounded-rectangle:hover {
  background: #5c7ecb;
  box-shadow: 0 4px 24px rgba(207, 216, 226, 0.15);
  transform: translateY(-4px) scale(1.03);
}

.bottom-label {
  font-size: 2em;
  color: #fff;
  font-weight: bold;
  text-align: center;
  width: 100%;
  margin-bottom: 10px;
}

.fat-arrow-container {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 56px;
  transition: filter 0.2s;
}

.fat-arrow {
  display: block;
  stroke: #80A4F9;
  filter: none;
  transition: stroke 0.2s, filter 0.2s;
  animation: arrow-bounce 2s ease-in-out infinite;
}

@keyframes arrow-bounce {
  0% { transform: translateY(0); }
  50% { transform: translateY(16px); }
  100% { transform: translateY(0); }
}

.fat-arrow-container:hover .fat-arrow {
  stroke: #5c7ecb;
  filter: drop-shadow(0 4px 24px rgba(128, 164, 249, 0.4));
  animation-play-state: paused;
}

.demo-section {
  width: 100vw;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  margin-top: 80px;
  z-index: 2;
  position: relative;
}
.demo-section h2 {
  color: #5c7ecb;
  font-size: 2.5em;
  font-weight: 700;
  letter-spacing: 1px;
  margin: 0;
  padding: 0;
  text-shadow: 0 2px 12px #d4e1fe99;
}

.spacer {
  height: 120vh;
  width: 100%;
}
</style>