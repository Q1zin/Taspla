import { ref } from 'vue';
import { getCookie, setCookie, deleteCookie } from '../utils/cookies';
import { api } from '../utils/api';
import { useSettings } from './useSettings';

interface User {
  id?: string;
  username: string;
  email: string;
}

interface ApiResponse {
  token: string;
  id?: string;
  user_id?: string;
  username?: string;
  email?: string;
  user?: {
    id: string;
    username: string;
    email: string;
  };
}

const API_BASE = '/api';
const TOKEN_KEY = 'auth_token';

const user = ref<User | null>(null);
const isAuthenticated = ref(false);
const isInitializing = ref(true); // Флаг инициализации
const token = ref<string | null>(getCookie(TOKEN_KEY));

// Проверяем токен при загрузке
const initPromise = (async () => {
  if (token.value) {
    try {
      await verifyToken();
    } catch {
      // Если токен невалидный, очищаем
      token.value = null;
      deleteCookie(TOKEN_KEY);
    }
  }
  isInitializing.value = false;
})();

async function verifyToken(): Promise<void> {
  if (!token.value) {
    throw new Error('No token');
  }

  const response = await api.get(`${API_BASE}/auth/verify`);

  if (!response.ok) {
    throw new Error('Invalid token');
  }

  const data: any = await response.json();
  console.log('Verify response:', data); // Для отладки
  
  user.value = {
    id: data.id || data.user_id,
    username: data.username || 'User',
    email: data.email || ''
  };
  isAuthenticated.value = true;
  
  // Загружаем настройки пользователя
  const { fetchSettings } = useSettings();
  await fetchSettings();
}

export function useAuth() {
  const login = async (email: string, password: string): Promise<void> => {
    const response = await api.post(`${API_BASE}/auth/login`, { email, password }, { skipAuth: true });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Ошибка входа' }));
      throw new Error(error.message || 'Неверный email или пароль');
    }

    const data: ApiResponse = await response.json();
    console.log('Login response:', data); // Для отладки
    
    if (!data.token) {
      throw new Error('Токен не получен от сервера');
    }
    
    // Сохраняем токен в cookie (7 дней)
    token.value = data.token;
    setCookie(TOKEN_KEY, data.token, 7);
    
    // Обрабатываем разные структуры ответа
    user.value = {
      id: data.user?.id || data.id || data.user_id,
      username: data.user?.username || data.username || 'User',
      email: data.user?.email || data.email || email
    };
    isAuthenticated.value = true;
    isInitializing.value = false;
    
    // Загружаем настройки пользователя
    const { fetchSettings } = useSettings();
    await fetchSettings();
  };

  const register = async (username: string, email: string, password: string): Promise<void> => {
    const response = await api.post(`${API_BASE}/auth/register`, { username, email, password }, { skipAuth: true });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Ошибка регистрации' }));
      throw new Error(error.message || 'Не удалось создать аккаунт');
    }

    const data: ApiResponse = await response.json();
    console.log('Register response:', data); // Для отладки
    
    if (!data.token) {
      throw new Error('Токен не получен от сервера');
    }
    
    // Сохраняем токен в cookie (7 дней)
    token.value = data.token;
    setCookie(TOKEN_KEY, data.token, 7);
    
    // Обрабатываем разные структуры ответа
    user.value = {
      id: data.user?.id || data.id || data.user_id,
      username: data.user?.username || data.username || username,
      email: data.user?.email || data.email || email
    };
    isAuthenticated.value = true;
    isInitializing.value = false;
    
    // Загружаем настройки пользователя
    const { fetchSettings } = useSettings();
    await fetchSettings();
  };

  const logout = () => {
    token.value = null;
    deleteCookie(TOKEN_KEY);
    user.value = null;
    isAuthenticated.value = false;
    isInitializing.value = false;
  };

  const updateUserName = (newName: string) => {
    if (user.value) {
      user.value.username = newName;
    }
  };

  const updateUserEmail = (newEmail: string) => {
    if (user.value) {
      user.value.email = newEmail;
    }
  };

  const changePassword = async (_currentPassword: string, _newPassword: string): Promise<void> => {
    // TODO: Реализовать через API когда будет готов эндпоинт PUT /auth/password
    console.log('Изменение пароля...');
    return new Promise((resolve) => {
      setTimeout(() => {
        console.log('Пароль успешно изменен');
        resolve();
      }, 1000);
    });
  };

  const getToken = (): string | null => {
    return token.value;
  };

  return {
    user,
    isAuthenticated,
    isInitializing,
    token,
    login,
    register,
    logout,
    updateUserName,
    updateUserEmail,
    changePassword,
    getToken,
    verifyToken,
    initPromise
  };
}
