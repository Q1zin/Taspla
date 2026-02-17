// Утилита для выполнения API запросов с автоматическим добавлением токена

import { getCookie } from './cookies';

const TOKEN_KEY = 'auth_token';

interface FetchOptions extends RequestInit {
  skipAuth?: boolean;
}

export async function apiFetch(url: string, options: FetchOptions = {}): Promise<Response> {
  const { skipAuth, ...fetchOptions } = options;
  
  const headers = new Headers(fetchOptions.headers);
  
  // Автоматически добавляем токен, если он есть
  if (!skipAuth) {
    const token = getCookie(TOKEN_KEY);
    if (token) {
      headers.set('Authorization', `Bearer ${token}`);
    }
  }
  
  // Устанавливаем Content-Type по умолчанию для JSON
  if (!headers.has('Content-Type') && fetchOptions.body) {
    headers.set('Content-Type', 'application/json');
  }
  
  return fetch(url, {
    ...fetchOptions,
    headers
  });
}

// Вспомогательные методы
export const api = {
  get: (url: string, options?: FetchOptions) => 
    apiFetch(url, { ...options, method: 'GET' }),
    
  post: (url: string, body?: any, options?: FetchOptions) => 
    apiFetch(url, { 
      ...options, 
      method: 'POST', 
      body: body ? JSON.stringify(body) : undefined 
    }),
    
  put: (url: string, body?: any, options?: FetchOptions) => 
    apiFetch(url, { 
      ...options, 
      method: 'PUT', 
      body: body ? JSON.stringify(body) : undefined 
    }),
    
  patch: (url: string, body?: any, options?: FetchOptions) => 
    apiFetch(url, { 
      ...options, 
      method: 'PATCH', 
      body: body ? JSON.stringify(body) : undefined 
    }),
    
  delete: (url: string, options?: FetchOptions) => 
    apiFetch(url, { ...options, method: 'DELETE' })
};
