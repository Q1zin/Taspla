import { ref, watch } from 'vue';

export type Theme = 'light' | 'dark';

// Загружаем сохраненные настройки из localStorage
const loadSettings = () => {
  const savedTheme = localStorage.getItem('theme') as Theme | null;
  const savedNotifications = localStorage.getItem('notifications');
  
  return {
    theme: savedTheme || 'light',
    notificationsEnabled: savedNotifications ? savedNotifications === 'true' : true
  };
};

const settings = loadSettings();
const theme = ref<Theme>(settings.theme);
const notificationsEnabled = ref(settings.notificationsEnabled);

// Применяем тему при загрузке
const applyTheme = (currentTheme: Theme) => {
  document.documentElement.setAttribute('data-theme', currentTheme);
};

applyTheme(theme.value);

export function useSettings() {
  // Следим за изменениями темы
  watch(theme, (newTheme) => {
    localStorage.setItem('theme', newTheme);
    applyTheme(newTheme);
  });

  // Следим за изменениями настройки уведомлений
  watch(notificationsEnabled, (newValue) => {
    localStorage.setItem('notifications', String(newValue));
  });

  const toggleTheme = () => {
    theme.value = theme.value === 'light' ? 'dark' : 'light';
  };

  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme;
  };

  const toggleNotifications = () => {
    notificationsEnabled.value = !notificationsEnabled.value;
  };

  const setNotifications = (enabled: boolean) => {
    notificationsEnabled.value = enabled;
  };

  return {
    theme,
    notificationsEnabled,
    toggleTheme,
    setTheme,
    toggleNotifications,
    setNotifications
  };
}
