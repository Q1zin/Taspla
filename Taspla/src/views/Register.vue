<script setup lang="ts">
import { ref } from 'vue';
import { useAuth } from '../composables/useAuth';
import { useRouter } from 'vue-router';

const router = useRouter();
const { register } = useAuth();

const username = ref('');
const email = ref('');
const password = ref('');
const showPassword = ref(false);
const error = ref('');
const loading = ref(false);

const handleRegister = async () => {
  if (!username.value || !email.value || !password.value) {
    error.value = 'Заполните все поля';
    return;
  }

  if (password.value.length < 6) {
    error.value = 'Пароль должен содержать минимум 6 символов';
    return;
  }

  error.value = '';
  loading.value = true;

  try {
    await register(username.value, email.value, password.value);
    router.push('/');
  } catch (e: any) {
    error.value = e.message || 'Ошибка регистрации';
  } finally {
    loading.value = false;
  }
};

const goToLogin = () => {
  router.push('/login');
};
</script>

<template>
  <div class="auth-page">
    <div class="auth-container">
      <div class="auth-card">
        <div class="auth-header">
          <h1>Регистрация</h1>
          <p>Создайте новый аккаунт</p>
        </div>

        <form @submit.prevent="handleRegister" class="auth-form">
          <div class="form-group">
            <label for="username">Имя пользователя</label>
            <input
              id="username"
              v-model="username"
              type="text"
              placeholder="Введите имя"
              :disabled="loading"
              autocomplete="username"
            />
          </div>

          <div class="form-group">
            <label for="email">Email</label>
            <input
              id="email"
              v-model="email"
              type="email"
              placeholder="example@mail.com"
              :disabled="loading"
              autocomplete="email"
            />
          </div>

          <div class="form-group">
            <label for="password">Пароль (минимум 6 символов)</label>
            <div class="password-input-wrapper">
              <input
                id="password"
                v-model="password"
                :type="showPassword ? 'text' : 'password'"
                placeholder="••••••••"
                :disabled="loading"
                autocomplete="new-password"
              />
              <button
                type="button"
                class="toggle-password"
                @click="showPassword = !showPassword"
                :disabled="loading"
                :aria-label="showPassword ? 'Скрыть пароль' : 'Показать пароль'"
              >
                <svg v-if="!showPassword" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                  <circle cx="12" cy="12" r="3"></circle>
                </svg>
                <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
                  <line x1="1" y1="1" x2="23" y2="23"></line>
                </svg>
              </button>
            </div>
            <span v-if="error" class="field-error">{{ error }}</span>
          </div>

          <button type="submit" class="btn-primary" :disabled="loading">
            {{ loading ? 'Регистрация...' : 'Зарегистрироваться' }}
          </button>
        </form>

        <div class="auth-footer">
          <p>
            Уже есть аккаунт?
            <button @click="goToLogin" class="link-button">Войти</button>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.auth-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
}

.auth-container {
  width: 100%;
  max-width: 420px;
}

.auth-card {
  background: var(--color-bg-card);
  border-radius: 16px;
  padding: 40px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.auth-header {
  text-align: center;
  margin-bottom: 32px;
}

.auth-header h1 {
  font-size: 32px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 8px;
}

.auth-header p {
  font-size: 16px;
  color: var(--color-text-secondary);
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  position: relative;
  padding-bottom: 4px;
}

.form-group label {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.form-group input {
  padding: 12px 16px;
  border: 2px solid var(--color-border);
  border-radius: 8px;
  font-size: 16px;
  transition: all 0.2s;
  background: var(--color-bg-card);
  color: var(--color-text-primary);
}

.form-group input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.1);
}

.form-group input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.form-group input::placeholder {
  color: var(--color-text-secondary);
}

.password-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.password-input-wrapper input {
  flex: 1;
  padding-right: 48px;
}

.toggle-password {
  position: absolute;
  right: 12px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  color: var(--color-text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s;
}

.toggle-password:hover:not(:disabled) {
  color: var(--color-primary);
}

.toggle-password:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.field-error {
  position: absolute;
  bottom: -16px;
  left: 0;
  font-size: 13px;
  color: var(--color-danger-text);
}

.btn-primary {
  padding: 14px 24px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-dark);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.4);
}

.btn-primary:active:not(:disabled) {
  transform: translateY(0);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.auth-footer {
  margin-top: 24px;
  text-align: center;
}

.auth-footer p {
  font-size: 14px;
  color: var(--color-text-secondary);
}

.link-button {
  background: none;
  border: none;
  color: var(--color-primary);
  font-weight: 600;
  cursor: pointer;
  padding: 0;
  margin-left: 4px;
  transition: color 0.2s;
}

.link-button:hover {
  color: var(--color-primary-dark);
  text-decoration: underline;
}
</style>
