<template>
  <div class="allancolumn1">
    <div class="logo-container">
      <h1 class="logo">Thinkboard</h1>
    </div>
    
    <div class="ai-assistant">
      <div class="ai-circle">
        <div class="ai-pulse-container">
          <div class="ai-pulse"></div>
          <div class="ai-pulse"></div>
          <div class="ai-pulse"></div>
          <div class="ai-pulse"></div>
          <div class="ai-pulse"></div>
        </div>
      </div>
      <div class="ai-text">AI Assistant</div>
    </div>
    
    <div class="nav-container">
      <!-- Navigation content can go here -->
    </div>
    
    <div class="footer-container">
      <p class="version">Thinkboard v1.0</p>
    </div>
  </div>
</template>
<style scoped>
.allancolumn1 {
  width: 280px; /* More reasonable width */
  height: 100vh;
  background-color: #2c3e50; /* Darker, more professional color */
  position: fixed;
  left: 0;
  top: 0;
  box-shadow: 2px 0 10px rgba(0,0,0,0.2);
  display: flex;
  flex-direction: column;
  border-radius: 0; /* Remove the border radius for a cleaner look */
  color: #ecf0f1;
  font-family: 'Roboto', Arial, sans-serif;
}

.logo-container {
  padding: 24px 20px;
  border-bottom: 1px solid rgba(255,255,255,0.1);
}

.logo {
  font-size: 24px;
  font-weight: 700;
  margin: 0;
  color: #fff;
  letter-spacing: 1px;
}

.nav-container {
  flex: 1;
  padding: 20px 0;
  overflow-y: auto;
}

.nav-items {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  margin-bottom: 8px;
}

.nav-link {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  color: rgba(255,255,255,0.8);
  text-decoration: none;
  transition: all 0.2s ease;
  border-left: 3px solid transparent;
}

.nav-link:hover {
  background-color: rgba(255,255,255,0.1);
  color: white;
  border-left: 3px solid #3498db;
}

.icon {
  margin-right: 12px;
  font-size: 18px;
  width: 24px;
  display: flex;
  justify-content: center;
}

.label {
  font-size: 15px;
}

.footer-container {
  padding: 16px 20px;
  border-top: 1px solid rgba(255,255,255,0.1);
  font-size: 12px;
  color: rgba(255,255,255,0.5);
  text-align: center;
}

.version {
  margin: 0;
}

.ai-assistant {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px 0;
  margin-top: 10px;
}

.ai-circle {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: rgba(52, 152, 219, 0.2);
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  border: 2px solid rgba(52, 152, 219, 0.5);
}

.ai-pulse-container {
  width: 60%;
  height: 40%;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.ai-pulse {
  width: 4px;
  height: 20px;
  background-color: #3498db;
  border-radius: 4px;
  animation: pulse 1.4s ease-in-out infinite;
}

.ai-pulse:nth-child(1) { animation-delay: -1.2s; }
.ai-pulse:nth-child(2) { animation-delay: -0.9s; }
.ai-pulse:nth-child(3) { animation-delay: -0.6s; }
.ai-pulse:nth-child(4) { animation-delay: -0.3s; }
.ai-pulse:nth-child(5) { animation-delay: 0s; }

@keyframes pulse {
  0%, 100% {
    height: 10px;
    background-color: rgba(52, 152, 219, 0.6);
  }
  50% {
    height: 30px;
    background-color: #3498db;
  }
}

.ai-text {
  margin-top: 14px;
  font-size: 16px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.9);
  letter-spacing: 0.5px;
}
</style>