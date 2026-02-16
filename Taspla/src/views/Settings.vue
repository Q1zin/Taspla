<template>
  <div class="settings-view">
    <AppHeader 
      @toggle-menu="isMenuOpen = true"
      @create-task="handleCreateTask"
    />
    
    <div class="settings-content">
      <div class="settings-container">
        <div class="settings-header">
          <div class="header-icon">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"></circle>
              <path d="M12 1v6m0 6v6"></path>
              <path d="m4.93 4.93 4.24 4.24m5.66 5.66 4.24 4.24"></path>
              <path d="M1 12h6m6 0h6"></path>
              <path d="m4.93 19.07 4.24-4.24m5.66-5.66 4.24-4.24"></path>
            </svg>
          </div>
          <h1 class="settings-title">Настройки</h1>
        </div>

        <div class="settings-section">
          <h2 class="section-title">Внешний вид</h2>
          
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-icon">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="5"></circle>
                  <line x1="12" y1="1" x2="12" y2="3"></line>
                  <line x1="12" y1="21" x2="12" y2="23"></line>
                  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                  <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                  <line x1="1" y1="12" x2="3" y2="12"></line>
                  <line x1="21" y1="12" x2="23" y2="12"></line>
                  <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                  <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                </svg>
              </div>
              <div class="setting-text">
                <span class="setting-label">Тема</span>
                <span class="setting-description">Выберите светлую или темную тему</span>
              </div>
            </div>
            
            <div class="theme-switcher">
              <button 
                class="theme-button"
                :class="{ active: theme === 'light' }"
                @click="setTheme('light')"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="5"></circle>
                  <line x1="12" y1="1" x2="12" y2="3"></line>
                  <line x1="12" y1="21" x2="12" y2="23"></line>
                  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                  <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                  <line x1="1" y1="12" x2="3" y2="12"></line>
                  <line x1="21" y1="12" x2="23" y2="12"></line>
                  <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                  <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                </svg>
                <span>Светлая</span>
              </button>
              
              <button 
                class="theme-button"
                :class="{ active: theme === 'dark' }"
                @click="setTheme('dark')"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
                <span>Темная</span>
              </button>
            </div>
          </div>
        </div>

        <div class="settings-section">
          <h2 class="section-title">Уведомления</h2>
          
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-icon">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                  <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                </svg>
              </div>
              <div class="setting-text">
                <span class="setting-label">Напоминания о задачах</span>
                <span class="setting-description">Получать уведомления о скором дедлайне</span>
              </div>
            </div>
            
            <label class="toggle-switch">
              <input 
                type="checkbox" 
                v-model="notificationsEnabled"
              />
              <span class="toggle-slider"></span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <SideMenu 
      :is-open="isMenuOpen"
      @close="isMenuOpen = false"
      @navigate="handleNavigate"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import AppHeader from '../components/AppHeader.vue';
import SideMenu from '../components/SideMenu.vue';
import { useSettings } from '../composables/useSettings';

const { theme, notificationsEnabled, setTheme } = useSettings();

const isMenuOpen = ref(false);

const emit = defineEmits<{
  navigate: [page: string];
}>();

const handleCreateTask = () => {
  emit('navigate', 'home:create-task');
};

const handleNavigate = (page: string) => {
  emit('navigate', page);
  isMenuOpen.value = false;
};
</script>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--color-bg);
  overflow: hidden;
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  justify-content: center;
}

.settings-container {
  max-width: 700px;
  width: 100%;
}

.settings-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 32px;
}

.header-icon {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: linear-gradient(135deg, #7C3AED 0%, #A78BFA 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  margin-bottom: 16px;
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
}

.settings-title {
  font-size: 32px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.settings-section {
  background: var(--color-bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 20px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 16px 0;
}

.setting-item:not(:last-child) {
  border-bottom: 1px solid var(--color-border);
}

[data-theme="dark"] .setting-item:not(:last-child) {
  border-bottom: 1px solid #374151;
}

.setting-info {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
}

.setting-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(167, 139, 250, 0.1) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-primary);
  flex-shrink: 0;
}

.setting-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.setting-label {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.setting-description {
  font-size: 14px;
  color: var(--color-text-secondary);
}

/* Theme Switcher */
.theme-switcher {
  display: flex;
  gap: 8px;
  background: var(--color-bg);
  padding: 4px;
  border-radius: 12px;
}

[data-theme="dark"] .theme-switcher {
  background: #1A1D29;
}

.theme-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.theme-button:hover {
  background: rgba(124, 58, 237, 0.1);
  color: var(--color-primary);
}

.theme-button.active {
  background: var(--color-bg-card);
  color: var(--color-primary);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

[data-theme="dark"] .theme-button.active {
  background: #252936;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 56px;
  height: 32px;
  flex-shrink: 0;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--color-border);
  border-radius: 32px;
  transition: all 0.3s;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 24px;
  width: 24px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  border-radius: 50%;
  transition: all 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-switch input:checked + .toggle-slider {
  background: linear-gradient(135deg, #7C3AED 0%, #A78BFA 100%);
}

.toggle-switch input:checked + .toggle-slider:before {
  transform: translateX(24px);
}

.toggle-switch input:focus + .toggle-slider {
  box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.2);
}

/* Responsive */
@media (max-width: 768px) {
  .settings-content {
    padding: 16px;
  }

  .settings-header {
    margin-bottom: 24px;
  }

  .header-icon {
    width: 100px;
    height: 100px;
  }

  .settings-title {
    font-size: 28px;
  }

  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }

  .theme-switcher {
    width: 100%;
  }

  .theme-button {
    flex: 1;
    justify-content: center;
  }
}
</style>
