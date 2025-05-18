<template>
  <div class="cloud-bg">
    <div class="cloud cloud1"></div>
    <div class="cloud cloud2"></div>
    <div class="cloud cloud3"></div>
    <div class="cloud cloud4"></div>
    <div class="cloud cloud5"></div>
    <div class="cloud cloud6"></div>
    <div class="cloud cloud7"></div>
    <div class="cloud cloud8"></div>
    <!-- Clouds moving from right to left -->
    <div class="cloud cloud9"></div>
    <div class="cloud cloud10"></div>
    <div class="cloud cloud11"></div>
    <div class="cloud cloud12"></div>
  </div>
</template>

<style scoped>
.cloud-bg {
  position: absolute;
  inset: 0;
  width: 100vw;
  height: calc(20vh - 4cm); /* much shorter */
  overflow: hidden;
  background: linear-gradient(to bottom, #b3e0ff 0%, #e6f7ff 100%);
  pointer-events: none;
  z-index: 0;
  animation: cloudstart 0s linear;

}
.cloud {
  position: relative;
  background: url('/public/cloud.png') no-repeat center/contain;
  background-size: contain;
  opacity: 0.8;
  pointer-events: none;
  z-index: 0;
}

/* Left to right clouds (existing) */
.cloud1 { width:220px; height:100px; top: 8vh; left: -220px; animation: cloudMove1 60s linear infinite; animation-delay: 0s; }
.cloud2 { width:300px; height:140px; top: 18vh; left: -300px; animation: cloudMove2 80s linear infinite; animation-delay: 10s; }
.cloud3 { width:180px; height:80px; top: 28vh; left: -180px; animation: cloudMove3 100s linear infinite; animation-delay: 20s; }
.cloud4 { width:260px; height:120px; top: 38vh; left: -220px; animation: cloudMove1 90s linear infinite; animation-delay: 30s; }
.cloud5 { width:200px; height:90px; top: 48vh; left: -300px; animation: cloudMove2 70s linear infinite; animation-delay: 40s; }
.cloud6 { width:220px; height:100px; top: 58vh; left: -180px; animation: cloudMove3 60s linear infinite; animation-delay: 50s; }
.cloud7 { width:300px; height:140px; top: 68vh; left: -220px; animation: cloudMove1 80s linear infinite; animation-delay: 60s; }
.cloud8 { width:180px; height:80px; top: 78vh; left: -300px; animation: cloudMove2 100s linear infinite; animation-delay: 70s; }

/* Right to left clouds (new) */
.cloud9  { width:220px; height:100px; top: 12vh; right: -220px; left: auto; animation: cloudMoveR1 65s linear infinite; animation-delay: 5s; }
.cloud10 { width:300px; height:140px; top: 26vh; right: -300px; left: auto; animation: cloudMoveR2 85s linear infinite; animation-delay: 15s; }
.cloud11 { width:180px; height:80px; top: 54vh; right: -180px; left: auto; animation: cloudMoveR3 105s linear infinite; animation-delay: 25s; }
.cloud12 { width:260px; height:120px; top: 72vh; right: -220px; left: auto; animation: cloudMoveR1 95s linear infinite; animation-delay: 35s; }

@keyframes cloudMove1 { 0% { left: -220px; } 100% { left: 110vw; } }
@keyframes cloudMove2 { 0% { left: -300px; } 100% { left: 110vw; } }
@keyframes cloudMove3 { 0% { left: -180px; } 100% { left: 110vw; } }

/* Right to left keyframes */
@keyframes cloudMoveR1 { 0% { right: -220px; } 100% { right: 110vw; } }
@keyframes cloudMoveR2 { 0% { right: -300px; } 100% { right: 110vw; } }
@keyframes cloudMoveR3 { 0% { right: -180px; } 100% { right: 110vw; } }
</style>