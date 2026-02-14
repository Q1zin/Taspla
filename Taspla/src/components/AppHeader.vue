<template>
  <header class="app-header">
    <button class="menu-button" @click="$emit('toggle-menu')" aria-label="Открыть меню">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="3" y1="6" x2="21" y2="6" />
        <line x1="3" y1="12" x2="21" y2="12" />
        <line x1="3" y1="18" x2="21" y2="18" />
      </svg>
    </button>
    
    <button class="create-button" @click="$emit('create-task')">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="12" y1="5" x2="12" y2="19" />
        <line x1="5" y1="12" x2="19" y2="12" />
      </svg>
      <span>Создать новую задачу</span>
    </button>
  </header>
</template>

<script setup lang="ts">
defineEmits<{
  'toggle-menu': [];
  'create-task': [];
}>();
</script>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background-color: #ffffff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.menu-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  color: #1F2937;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  transition: background-color 0.2s;
}

.menu-button:hover {
  background-color: #F3F4F6;
}

.create-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: #7C3AED;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 12px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 2px 8px rgba(124, 58, 237, 0.3);
}

.create-button:hover {
  background-color: #6D28D9;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.4);
}

.create-button:active {
  transform: translateY(0);
}

@media (max-width: 640px) {
  .create-button span {
    display: none;
  }
  
  .create-button {
    padding: 10px;
    border-radius: 50%;
  }
}
</style>
