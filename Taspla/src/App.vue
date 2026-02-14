<script setup lang="ts">
import { ref } from 'vue';
import Home from './views/Home.vue';
import Profile from './views/Profile.vue';

const currentPage = ref<'home' | 'profile'>('home');
const openTaskModal = ref(false);

const navigateTo = (page: string) => {
  if (page === 'home:create-task') {
    currentPage.value = 'home';
    openTaskModal.value = true;
  } else if (page === 'home' || page === 'profile') {
    currentPage.value = page;
    openTaskModal.value = false;
  }
};

const resetTaskModal = () => {
  openTaskModal.value = false;
};
</script>

<template>
  <Home 
    v-if="currentPage === 'home'" 
    @navigate="navigateTo"
    :open-task-modal="openTaskModal"
    @reset-task-modal="resetTaskModal"
  />
  <Profile v-else-if="currentPage === 'profile'" @navigate="navigateTo" />
</template>

<style>
*, *::before, *::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  /* Colors */
  --color-primary: #7C3AED;
  --color-primary-dark: #6D28D9;
  --color-primary-light: #A78BFA;
  
  --color-success: #4ADE80;
  --color-success-dark: #22C55E;
  
  --color-danger: #FCA5A5;
  --color-danger-dark: #F87171;
  
  --color-bg: #E8EEF2;
  --color-bg-card: #FFFFFF;
  
  --color-text-primary: #1F2937;
  --color-text-secondary: #6B7280;
  
  /* Typography */
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Inter', 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  font-size: 16px;
  line-height: 1.5;
  font-weight: 400;

  color: var(--color-text-primary);
  background-color: var(--color-bg);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  min-height: 100vh;
  overflow: hidden;
}

#app {
  height: 100vh;
  overflow: hidden;
}

button {
  font-family: inherit;
}

input, textarea, select {
  font-family: inherit;
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

::-webkit-scrollbar-track {
  background: #F3F4F6;
}

::-webkit-scrollbar-thumb {
  background: #D1D5DB;
  border-radius: 5px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9CA3AF;
}

/* Remove focus outline for mouse users, keep for keyboard users */
:focus:not(:focus-visible) {
  outline: none;
}

:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}
</style>
