<template>
    <div class="about-container">
      <h1 class="about-title">About OS</h1>
      
      <div class="about-content">
        <div class="info-row">
          <span class="label">Version:</span>
          <span class="value">{{ version }}</span>
        </div>
        <div class="info-row">
          <span class="label">Platform:</span>
          <span class="value">{{ platform }}</span>
        </div>
        <div class="info-row">
          <span class="label">Storage (Used/Max):</span>
          <span class="value">{{ storageUsed }} / {{ storageMax }}</span>
        </div>
      </div>
  
      <div class="about-footer">
        <p class="copyright">© 2026-{{ new Date().getFullYear() }} Lucy Mason</p>
      </div>
    </div>
  </template>
  
  <style scoped>
  .about-container {
    box-sizing: border-box;
    padding: 12px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    overflow: hidden;
  }
  
  .about-title {
    font-size: 1.1rem;
    font-weight: bold;
    margin: 0 0 8px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    padding-bottom: 4px;
  }
  
  .about-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 4px;
  }
  
  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    line-height: 1.2;
  }
  
  .label {
    color: rgba(255, 255, 255, 0.6);
  }
  
  .value {
    font-weight: 500;
    text-align: right;
  }
  
  .about-footer {
    text-align: center;
    border-top: 1px solid rgba(255, 255, 255, 0.2);
    padding-top: 6px;
    margin-top: 1rem;
  }
  
  .copyright {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.4);
    margin: 0;
  }
  </style>

<script>
export default {
  data() {
    return {
      version: "...",
      platform: "...",
      storageUsed: "...",
      storageMax: "...",
    }
  },
  async mounted() {
    try {
      const response = await fetch('/api/info');
      const data = await response.json();
      console.log(data);

      this.version     = data.version;
      this.platform    = data.platform;
      this.storageUsed = data.storageUsed;
      this.storageMax  = data.storageMax;
    } catch (error) {
      console.error("Failed to fetch system info:", error);
      this.version = "Unknown";
      this.platform = "Unknown";
    }
  }
}
</script>