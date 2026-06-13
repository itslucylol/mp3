<template>
    <div class="ipod-container">
      <div class="ipod-screen" :style="{ backgroundImage: `url(${albumArt})` }">
        <div class="screen-overlay">
          <div class="status-bar">
            <span class="time">9:41 AM</span>
            <span class="context">Music</span>
            <div class="status-icons">
              <span class="signal">📶 5G</span>
              <span class="battery">🔋</span>
            </div>
          </div>
  
          <div class="track-info">
            <div class="meta">
              <h1 class="song-title">Good Days <span class="explicit">E</span></h1>
              <p class="artist">SZA</p>
              <p class="album">SOS</p>
            </div>
          </div>
  
          <div class="audio-features">
            <span class="dolby"> Dolby Atmos</span>
            <div class="screen-actions">
              <button class="icon-btn">⭐</button>
              <button class="icon-btn">•••</button>
            </div>
          </div>
  
          <div class="progress-container">
            <div class="time-stamps">
              <span>{{ formatTime(currentTime) }}</span>
              <span>-{{ formatTime(duration - currentTime) }}</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
            </div>
          </div>
  
          <div class="screen-footer">
            <span class="device-name">🎧 {{ deviceName }}'s AirPods</span>
            <div class="footer-icons">
              <button class="icon-btn">💬</button>
              <button class="icon-btn">☰</button>
            </div>
          </div>
        </div>
      </div>
  
      <div class="click-wheel">
        <button class="wheel-btn top-btn">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
            <rect x="4" y="4" width="6" height="6" rx="1"/>
            <rect x="14" y="4" width="6" height="6" rx="1"/>
            <rect x="4" y="14" width="6" height="6" rx="1"/>
            <rect x="14" y="14" width="6" height="6" rx="1"/>
          </svg>
        </button>
        <button class="wheel-btn right-btn">⏭</button>
        <button class="wheel-btn bottom-btn">⏯</button>
        <button class="wheel-btn left-btn">⏮</button>
        
        <button class="center-button"></button>
      </div>
    </div>
  </template>

<style scoped>
/* Chassis / Body */
.ipod-container {
  width: 380px;
  height: 680px;
  background-color: #3e4654; /* Matte slate gray */
  border-radius: 40px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: inset 0 2px 4px rgba(255,255,255,0.2), 0 20px 40px rgba(0,0,0,0.4);
  box-sizing: border-box;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  user-select: none;
}

/* --- SCREEN STYLES --- */
.ipod-screen {
  width: 100%;
  height: 320px;
  border-radius: 24px;
  background-size: cover;
  background-position: center;
  overflow: hidden;
  position: relative;
  box-shadow: 0 0 0 2px #2d333f;
}

.screen-overlay {
  width: 100%;
  height: 100%;
  background: linear-gradient(to bottom, rgba(0,0,0,0.3) 0%, rgba(0,0,0,0.5) 40%, rgba(0,0,0,0.85) 100%);
  backdrop-filter: blur(10px); /* Gives it that Apple fluid look */
  padding: 14px 16px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  color: #ffffff;
}

.status-bar {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  font-weight: 600;
  opacity: 0.8;
}
.status-bar .context { font-weight: 400; }

.track-info {
  margin-top: auto;
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}

.song-title {
  font-size: 22px;
  font-weight: 600;
  margin: 0 0 4px 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.explicit {
  font-size: 10px;
  background: rgba(255,255,255,0.2);
  padding: 1px 4px;
  border-radius: 3px;
  text-transform: uppercase;
}

.artist { font-size: 15px; opacity: 0.7; margin: 0; }
.album { font-size: 13px; opacity: 0.5; margin: 2px 0 0 0; }

.audio-features {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 16px;
  font-size: 13px;
  font-weight: 500;
}
.dolby { opacity: 0.9; display: flex; align-items: center; gap: 4px;}

.screen-actions { display: flex; gap: 12px; }
.icon-btn {
  background: none;
  border: none;
  color: white;
  cursor: pointer;
  font-size: 16px;
  opacity: 0.7;
  padding: 0;
}
.icon-btn:hover { opacity: 1; }

/* Progress Bar */
.progress-container {
  margin-top: 8px;
}
.time-stamps {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  opacity: 0.5;
  margin-bottom: 6px;
}
.progress-bar {
  width: 100%;
  height: 4px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: #ffffff;
}

/* Screen Footer */
.screen-footer {
  margin-top: 14px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  border-top: 1px solid rgba(255,255,255,0.1);
  padding-top: 10px;
}
.device-name { opacity: 0.8; }
.footer-icons { display: flex; gap: 14px; }


/* --- CLICK WHEEL STYLES --- */
.click-wheel {
  margin-top: 50px;
  width: 240px;
  height: 240px;
  background-color: #1a1a1a;
  border-radius: 50%;
  position: relative;
  display: grid;
  grid-template-areas: 
    ". top ."
    "left center right"
    ". bottom .";
  grid-template-rows: 1fr 1fr 1fr;
  grid-template-columns: 1fr 1fr 1fr;
  box-shadow: inset 0 4px 10px rgba(0,0,0,0.5), 0 4px 10px rgba(0,0,0,0.3);
}

.wheel-btn {
  background: none;
  border: none;
  color: #ffffff;
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: opacity 0.2s;
  opacity: 0.8;
}
.wheel-btn:active { opacity: 0.4; }

.top-btn { grid-area: top; align-items: flex-start; padding-top: 16px; }
.right-btn { grid-area: right; justify-content: flex-end; padding-right: 16px; }
.bottom-btn { grid-area: bottom; align-items: flex-end; padding-bottom: 16px; }
.left-btn { grid-area: left; justify-content: flex-start; padding-left: 16px; }

.center-button {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 86px;
  height: 86px;
  background-color: #262626;
  border: 1px solid #1a1a1a;
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 4px 6px rgba(0,0,0,0.3);
}
.center-button:active {
  background-color: #202020;
}
</style>
  
  <script setup>
  import { ref, computed } from 'vue';
  
  // Mock State
  const albumArt = ref('https://images.unsplash.com/photo-1518837695005-2083093ee35b?w=500'); // Deep ocean placeholder
  const currentTime = ref(82); // 1:22 in seconds
  const duration = ref(279); // Total song duration
  const deviceName = ref('kilobyte');
  
  const progressPercent = computed(() => (currentTime.value / duration.value) * 100);
  
  const formatTime = (seconds) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs < 10 ? '0' : ''}${secs}`;
  };
  </script>