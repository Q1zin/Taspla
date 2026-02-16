<template>
  <Transition name="menu-overlay">
    <div v-if="isOpen" class="menu-overlay" @click="$emit('close')"></div>
  </Transition>
  
  <Transition name="menu">
    <aside v-if="isOpen" class="side-menu">
      <button class="close-button" @click="$emit('close')" aria-label="Закрыть меню">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
      </button>

      <div class="menu-content">
        <div class="logo-section">
          <div class="logo-icon">
            <img src="../assets/logo.svg" alt="Taspla Logo" class="logo-image" />
          </div>
          <h1 class="logo-text">Taspla</h1>
        </div>

        <nav class="menu-items">
          <button class="menu-item" @click="$emit('navigate', 'home')">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
              <polyline points="9 22 9 12 15 12 15 22"></polyline>
            </svg>
            <span>Задачи</span>
          </button>
          <button class="menu-item" @click="$emit('navigate', 'profile')">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
            <span>Профиль</span>
          </button>
          
          <button class="menu-item" @click="$emit('navigate', 'settings')">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"></circle>
              <path d="M12 1v6m0 6v6"></path>
              <path d="m4.93 4.93 4.24 4.24m5.66 5.66 4.24 4.24"></path>
              <path d="M1 12h6m6 0h6"></path>
              <path d="m4.93 19.07 4.24-4.24m5.66-5.66 4.24-4.24"></path>
            </svg>
            <span>Настройки</span>
          </button>
        </nav>

        <footer class="menu-footer">
          <p>Design & Development<br/>@Q1zin</p>
        </footer>
      </div>
    </aside>
  </Transition>
</template>

<script setup lang="ts">
defineProps<{
  isOpen: boolean;
}>();

defineEmits<{
  close: [];
  navigate: [page: string];
}>();
</script>

<style scoped>
.menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 998;
}

.side-menu {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  width: 280px;
  background: linear-gradient(180deg, #E0E7FF 0%, #DDD6FE 100%);
  z-index: 999;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 16px rgba(0, 0, 0, 0.1);
}

[data-theme="dark"] .side-menu {
  background: linear-gradient(180deg, #1E293B 0%, #0F172A 100%);
}

.close-button {
  position: absolute;
  top: 16px;
  left: 16px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  color: var(--color-text-primary);
  border-radius: 8px;
  transition: background-color 0.2s;
}

.close-button:hover {
  background-color: rgba(255, 255, 255, 0.5);
}

[data-theme="dark"] .close-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.menu-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 72px 24px 24px;
}

.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 48px;
}

.logo-icon {
  background-color: transparent;
  border-radius: 16px;
  padding: 0;
  margin-bottom: 16px;
}

.logo-image {
  width: 64px;
  height: 64px;
  display: block;
  filter: drop-shadow(0 4px 12px rgba(124, 58, 237, 0.3));
}

.logo-text {
  font-family: 'Orbitron', sans-serif;
  font-size: 36px;
  font-weight: 800;
  color: #212D61;
  margin: 0;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.menu-items {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 16px;
  background: rgba(255, 255, 255, 0.7);
  border: none;
  padding: 16px 20px;
  border-radius: 12px;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

[data-theme="dark"] .menu-item {
  background: rgba(255, 255, 255, 0.1);
  color: var(--color-text-primary);
}

.menu-item:hover {
  background: rgba(255, 255, 255, 0.9);
  transform: translateX(4px);
}

[data-theme="dark"] .menu-item:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateX(4px);
}

.menu-footer {
  margin-top: auto;
  text-align: center;
  color: var(--color-text-secondary);
  font-size: 12px;
  line-height: 1.5;
}

/* Transitions */
.menu-overlay-enter-active,
.menu-overlay-leave-active {
  transition: opacity 0.3s ease;
}

.menu-overlay-enter-from,
.menu-overlay-leave-to {
  opacity: 0;
}

.menu-enter-active,
.menu-leave-active {
  transition: transform 0.3s ease;
}

.menu-enter-from,
.menu-leave-to {
  transform: translateX(-100%);
}
</style>
