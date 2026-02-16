<template>
  <div class="profile-view">
    <AppHeader 
      @toggle-menu="isMenuOpen = true"
      @create-task="handleCreateTask"
    />
    
    <div class="profile-content">
      <div class="profile-container">
        <div class="profile-header">
          <div class="avatar">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
          </div>
          <h1 class="profile-title">Профиль</h1>
        </div>

        <div class="profile-section">
          <h2 class="section-title">Информация</h2>
          
          <div class="info-item">
            <label class="info-label">Имя пользователя</label>
            <div class="info-value-container">
              <input 
                v-if="isEditingName" 
                v-model="editedName"
                class="info-input"
                type="text"
                @keyup.enter="saveName"
                @keyup.escape="cancelEditName"
              />
              <span v-else class="info-value">{{ user.name }}</span>
              <button 
                v-if="!isEditingName"
                @click="startEditName" 
                class="edit-button"
                aria-label="Редактировать имя"
              >
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                  <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                </svg>
              </button>
              <div v-else class="edit-actions">
                <button @click="saveName" class="save-button">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20 6 9 17 4 12"></polyline>
                  </svg>
                </button>
                <button @click="cancelEditName" class="cancel-button">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <div class="info-item">
            <label class="info-label">Почта</label>
            <div class="info-value-container">
              <span class="info-value">{{ user.email }}</span>
            </div>
          </div>
        </div>

        <div class="profile-section">
          <h2 class="section-title">Безопасность</h2>
          
          <button 
            @click="showPasswordModal = true" 
            class="action-button primary"
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
              <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
            </svg>
            <span>Изменить пароль</span>
          </button>
        </div>

        <div class="profile-section">
          <h2 class="section-title">Аккаунт</h2>
          
          <button 
            @click="handleLogout" 
            class="action-button danger"
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
              <polyline points="16 17 21 12 16 7"></polyline>
              <line x1="21" y1="12" x2="9" y2="12"></line>
            </svg>
            <span>Выйти из аккаунта</span>
          </button>
        </div>
      </div>
    </div>

    <SideMenu 
      :is-open="isMenuOpen"
      @close="isMenuOpen = false"
      @navigate="handleNavigate"
    />

    <!-- Модальное окно для изменения пароля -->
    <Transition name="modal">
      <div v-if="showPasswordModal" class="modal-overlay" @click="closePasswordModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3 class="modal-title">Изменить пароль</h3>
            <button @click="closePasswordModal" class="modal-close" aria-label="Закрыть">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </div>
          
          <div class="modal-body">
            <div class="input-group">
              <label class="input-label">Текущий пароль</label>
              <input 
                v-model="currentPassword"
                type="password"
                class="password-input"
                placeholder="Введите текущий пароль"
              />
            </div>
            
            <div class="input-group">
              <label class="input-label">Новый пароль</label>
              <input 
                v-model="newPassword"
                type="password"
                class="password-input"
                placeholder="Введите новый пароль"
              />
            </div>
            
            <div class="input-group">
              <label class="input-label">Подтвердите новый пароль</label>
              <input 
                v-model="confirmPassword"
                type="password"
                class="password-input"
                placeholder="Повторите новый пароль"
              />
            </div>

            <p v-if="passwordError" class="error-message">{{ passwordError }}</p>
            <p v-if="passwordSuccess" class="success-message">{{ passwordSuccess }}</p>
          </div>
          
          <div class="modal-footer">
            <button @click="closePasswordModal" class="modal-button secondary">
              Отмена
            </button>
            <button @click="handleChangePassword" class="modal-button primary">
              Изменить
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import AppHeader from '../components/AppHeader.vue';
import SideMenu from '../components/SideMenu.vue';
import { useAuth } from '../composables/useAuth';

const { user, updateUserName, changePassword, logout } = useAuth();

const isMenuOpen = ref(false);
const isEditingName = ref(false);
const editedName = ref('');

const showPasswordModal = ref(false);
const currentPassword = ref('');
const newPassword = ref('');
const confirmPassword = ref('');
const passwordError = ref('');
const passwordSuccess = ref('');

const emit = defineEmits<{
  navigate: [page: string];
}>();

// Редактирование имени
const startEditName = () => {
  editedName.value = user.value.name;
  isEditingName.value = true;
};

const saveName = () => {
  if (editedName.value.trim()) {
    updateUserName(editedName.value.trim());
    isEditingName.value = false;
  }
};

const cancelEditName = () => {
  isEditingName.value = false;
  editedName.value = '';
};

// Создание задачи
const handleCreateTask = () => {
  emit('navigate', 'home:create-task');
};

// Изменение пароля
const handleChangePassword = async () => {
  passwordError.value = '';
  passwordSuccess.value = '';

  if (!currentPassword.value || !newPassword.value || !confirmPassword.value) {
    passwordError.value = 'Заполните все поля';
    return;
  }

  if (newPassword.value !== confirmPassword.value) {
    passwordError.value = 'Новые пароли не совпадают';
    return;
  }

  if (newPassword.value.length < 6) {
    passwordError.value = 'Пароль должен содержать минимум 6 символов';
    return;
  }

  try {
    await changePassword(currentPassword.value, newPassword.value);
    passwordSuccess.value = 'Пароль успешно изменен!';
    
    setTimeout(() => {
      closePasswordModal();
    }, 2000);
  } catch (error) {
    passwordError.value = 'Ошибка при изменении пароля';
  }
};

const closePasswordModal = () => {
  showPasswordModal.value = false;
  currentPassword.value = '';
  newPassword.value = '';
  confirmPassword.value = '';
  passwordError.value = '';
  passwordSuccess.value = '';
};

// Выход из аккаунта
const handleLogout = () => {
  if (confirm('Вы уверены, что хотите выйти?')) {
    logout();
    emit('navigate', 'home');
  }
};

const handleNavigate = (page: string) => {
  emit('navigate', page);
  isMenuOpen.value = false;
};
</script>

<style scoped>
.profile-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--color-bg);
  overflow: hidden;
}

.profile-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  justify-content: center;
}

.profile-container {
  max-width: 600px;
  width: 100%;
}

.profile-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 32px;
}

.avatar {
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

.profile-title {
  font-size: 32px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.profile-section {
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
  margin-bottom: 16px;
}

.info-item {
  margin-bottom: 20px;
}

.info-item:last-child {
  margin-bottom: 0;
}

.info-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.info-value-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.info-value {
  flex: 1;
  font-size: 16px;
  color: var(--color-text-primary);
  padding: 10px 0;
}

.info-input {
  flex: 1;
  padding: 10px 12px;
  border: 2px solid var(--color-primary-light);
  border-radius: 8px;
  font-size: 16px;
  color: var(--color-text-primary);
  outline: none;
  transition: border-color 0.2s;
}

.info-input:focus {
  border-color: var(--color-primary);
}

.edit-button,
.save-button,
.cancel-button {
  background: none;
  border: none;
  padding: 8px;
  cursor: pointer;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.edit-button {
  color: var(--color-primary);
}

.edit-button:hover {
  background-color: rgba(124, 58, 237, 0.1);
}

.edit-actions {
  display: flex;
  gap: 8px;
}

.save-button {
  color: var(--color-success-dark);
}

.save-button:hover {
  background-color: rgba(74, 222, 128, 0.1);
}

.cancel-button {
  color: var(--color-danger-dark);
}

.cancel-button:hover {
  background-color: rgba(248, 113, 113, 0.1);
}

.action-button {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.action-button.primary {
  background: var(--color-primary);
  color: white;
}

.action-button.primary:hover {
  background: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
}

.action-button.danger {
  background: var(--color-danger-dark);
  color: white;
}

.action-button.danger:hover {
  background: #EF4444;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(248, 113, 113, 0.3);
}

/* Модальное окно */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 16px;
}

.modal-content {
  background: var(--color-bg-card);
  border-radius: 16px;
  max-width: 500px;
  width: 100%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 24px 16px;
  border-bottom: 1px solid var(--color-border);
}

.modal-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.modal-close {
  background: none;
  border: none;
  padding: 8px;
  cursor: pointer;
  border-radius: 8px;
  color: var(--color-text-secondary);
  transition: all 0.2s;
}

.modal-close:hover {
  background-color: var(--color-bg);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 24px;
}

.input-group {
  margin-bottom: 16px;
}

.input-group:last-of-type {
  margin-bottom: 0;
}

.input-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.password-input {
  width: 100%;
  padding: 12px;
  border: 2px solid var(--color-border);
  border-radius: 8px;
  font-size: 16px;
  color: var(--color-text-primary);
  background-color: var(--color-bg-card);
  outline: none;
  transition: border-color 0.2s;
}

.password-input:focus {
  border-color: var(--color-primary);
}

.error-message {
  margin-top: 16px;
  padding: 12px;
  background-color: var(--color-danger-bg);
  color: var(--color-danger-text);
  border-radius: 8px;
  font-size: 14px;
}

.success-message {
  margin-top: 16px;
  padding: 12px;
  background-color: var(--color-success-bg);
  color: var(--color-success-text);
  border-radius: 8px;
  font-size: 14px;
}

.modal-footer {
  display: flex;
  gap: 12px;
  padding: 16px 24px 24px;
  border-top: 1px solid var(--color-border);
}

.modal-button {
  flex: 1;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.modal-button.secondary {
  background: var(--color-border);
  color: var(--color-text-primary);
}

.modal-button.secondary:hover {
  background: var(--color-text-secondary);
}

.modal-button.primary {
  background: var(--color-primary);
  color: white;
}

.modal-button.primary:hover {
  background: var(--color-primary-dark);
}

/* Анимации */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.3s;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.9);
}
</style>
