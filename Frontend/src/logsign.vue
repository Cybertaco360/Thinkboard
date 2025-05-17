<template>
  <div class="center-container">
    <img src="/public/thinkboard.png" alt="ThinkBoard Logo" class="logo" />
    <div class="rectangles-container">
      <div class="rounded-rectangle" @click="$emit('open-signup')">
        <div class="bottom-label">Sign Up</div>
      </div>
      <div class="rounded-rectangle" @click="$emit('open-sidepan')">
        <div class="bottom-label">Log In</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.center-container {
  min-height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 1;
}

.logo {
  width: 500px;
  display: block;
  object-fit: contain;
  margin-bottom: 40px;
}

.rectangles-container {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 24px;
}

.rounded-rectangle {
  width: 240px;
  height: 90px;
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
</style>