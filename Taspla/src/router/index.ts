import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { useAuth } from '../composables/useAuth';

import Login from '../views/Login.vue';
import Register from '../views/Register.vue';
import Home from '../views/Home.vue';
import Profile from '../views/Profile.vue';
import Settings from '../views/Settings.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: Login,
    meta: { requiresGuest: true }
  },
  {
    path: '/register',
    name: 'register',
    component: Register,
    meta: { requiresGuest: true }
  },
  {
    path: '/',
    name: 'home',
    component: Home,
    meta: { requiresAuth: true }
  },
  {
    path: '/profile',
    name: 'profile',
    component: Profile,
    meta: { requiresAuth: true }
  },
  {
    path: '/settings',
    name: 'settings',
    component: Settings,
    meta: { requiresAuth: true }
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

// Navigation guard для защиты роутов
router.beforeEach(async (to, _from, next) => {
  const { isAuthenticated, isInitializing, initPromise } = useAuth();

  // Дожидаемся завершения инициализации
  if (isInitializing.value) {
    await initPromise;
  }

  // Если роут требует авторизации и пользователь не авторизован
  if (to.meta.requiresAuth && !isAuthenticated.value) {
    next('/login');
    return;
  }

  // Если роут для гостей и пользователь авторизован
  if (to.meta.requiresGuest && isAuthenticated.value) {
    next('/');
    return;
  }

  next();
});

export default router;
