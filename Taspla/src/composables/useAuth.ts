import { ref } from 'vue';

interface User {
  name: string;
  email: string;
}

const user = ref<User>({
  name: 'Пользователь',
  email: 'user@example.com'
});

const isAuthenticated = ref(true);

export function useAuth() {
  const updateUserName = (newName: string) => {
    user.value.name = newName;
  };

  const updateUserEmail = (newEmail: string) => {
    user.value.email = newEmail;
  };

  const changePassword = (_currentPassword: string, _newPassword: string) => {
    // Здесь будет логика изменения пароля через Tauri backend
    console.log('Изменение пароля...');
    return new Promise((resolve) => {
      setTimeout(() => {
        console.log('Пароль успешно изменен');
        resolve(true);
      }, 1000);
    });
  };

  const logout = () => {
    isAuthenticated.value = false;
    user.value = {
      name: '',
      email: ''
    };
    console.log('Выход из аккаунта');
  };

  return {
    user,
    isAuthenticated,
    updateUserName,
    updateUserEmail,
    changePassword,
    logout
  };
}
