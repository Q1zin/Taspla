<script setup lang="ts">
</script>

<template>
  <router-view />
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
  --color-success-bg: #D1FAE5;
  --color-success-text: #059669;
  
  --color-danger: #FCA5A5;
  --color-danger-dark: #F87171;
  --color-danger-bg: #FEE2E2;
  --color-danger-text: #DC2626;
  
  --color-bg: #E8EEF2;
  --color-bg-card: #FFFFFF;
  --color-border: #E5E7EB;
  
  --color-text-primary: #1F2937;
  --color-text-secondary: #6B7280;
  
  /* Priority colors */
  --priority-low-bg: #D4F4DD;
  --priority-low-text: #2C9C1E;
  --priority-low-border: #2C9C1E;
  
  --priority-medium-bg: #FEF3C7;
  --priority-medium-text: #92400E;
  --priority-medium-border: #FBBF24;
  
  --priority-high-bg: #FED7AA;
  --priority-high-text: #9A3412;
  --priority-high-border: #F59E0B;
  
  --priority-critical-bg: #FEE2E2;
  --priority-critical-text: #991B1B;
  --priority-critical-border: #EF4444;
  
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

/* Dark Theme */
[data-theme="dark"] {
  --color-bg: #1A1D29;
  --color-bg-card: #252936;
  --color-border: #374151;
  
  --color-text-primary: #E5E7EB;
  --color-text-secondary: #9CA3AF;
  
  --color-primary: #8B5CF6;
  --color-primary-dark: #7C3AED;
  --color-primary-light: #A78BFA;
  
  --color-success-bg: #064E3B;
  --color-success-text: #6EE7B7;
  
  --color-danger-bg: #7F1D1D;
  --color-danger-text: #FCA5A5;
  
  /* Priority colors - Dark theme */
  --priority-low-bg: #064E3B;
  --priority-low-text: #6EE7B7;
  --priority-low-border: #10B981;
  
  --priority-medium-bg: #78350F;
  --priority-medium-text: #FCD34D;
  --priority-medium-border: #F59E0B;
  
  --priority-high-bg: #7C2D12;
  --priority-high-text: #FCA5A5;
  --priority-high-border: #F97316;
  
  --priority-critical-bg: #7F1D1D;
  --priority-critical-text: #FCA5A5;
  --priority-critical-border: #EF4444;
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

[data-theme="dark"] ::-webkit-scrollbar-track {
  background: #1A1D29;
}

::-webkit-scrollbar-thumb {
  background: #D1D5DB;
  border-radius: 5px;
}

[data-theme="dark"] ::-webkit-scrollbar-thumb {
  background: #4B5563;
}

::-webkit-scrollbar-thumb:hover {
  background: #9CA3AF;
}

[data-theme="dark"] ::-webkit-scrollbar-thumb:hover {
  background: #6B7280;
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
