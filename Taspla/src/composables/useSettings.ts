import { ref, watch } from 'vue';
import { api } from '../utils/api';

export type Theme = 'light' | 'dark';

interface Settings {
  theme: Theme;
  notifications_enabled: boolean;
}

const API_BASE = '/api';
const theme = ref<Theme>('light');
const notificationsEnabled = ref(true);
const loading = ref(false);
const error = ref<string | null>(null);
const initialized = ref(false);

// Применяем тему
const applyTheme = (currentTheme: Theme) => {
  document.documentElement.setAttribute('data-theme', currentTheme);
};

// Загрузка настроек с сервера
const fetchSettings = async () => {
  loading.value = true;
  error.value = null;
  
  try {
    const response = await api.get(`${API_BASE}/settings`);
    
    if (!response.ok) {
      throw new Error('Ошибка загрузки настроек');
    }
    
    const data: Settings = await response.json();
    theme.value = data.theme;
    notificationsEnabled.value = data.notifications_enabled;
    applyTheme(theme.value);
    initialized.value = true;
  } catch (e: any) {
    error.value = e.message;
    console.error('Error fetching settings:', e);
    // Откатываемся на localStorage при ошибке
    const savedTheme = localStorage.getItem('theme') as Theme | null;
    const savedNotifications = localStorage.getItem('notifications');
    theme.value = savedTheme || 'light';
    notificationsEnabled.value = savedNotifications ? savedNotifications === 'true' : true;
    applyTheme(theme.value);
  } finally {
    loading.value = false;
  }
};

// Сохранение настроек на сервер
const saveSettings = async () => {
  try {
    const response = await api.put(`${API_BASE}/settings`, {
      theme: theme.value,
      notifications_enabled: notificationsEnabled.value
    });
    
    if (!response.ok) {
      throw new Error('Ошибка сохранения настроек');
    }
    
    // Сохраняем также в localStorage для резервирования
    localStorage.setItem('theme', theme.value);
    localStorage.setItem('notifications', String(notificationsEnabled.value));
  } catch (e: any) {
    error.value = e.message;
    console.error('Error saving settings:', e);
  }
};

export function useSettings() {
  // Автоматически загружаем настройки при первом использовании
  if (!initialized.value && !loading.value) {
    fetchSettings();
  }

  // Следим за изменениями темы
  watch(theme, (newTheme) => {
    applyTheme(newTheme);
    if (initialized.value) {
      saveSettings();
    }
  });

  // Следим за изменениями настройки уведомлений
  watch(notificationsEnabled, () => {
    if (initialized.value) {
      saveSettings();
    }
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
    loading,
    error,
    toggleTheme,
    setTheme,
    toggleNotifications,
    setNotifications,
    fetchSettings
  };
}
