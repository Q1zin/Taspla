import { ref, computed } from 'vue';
import { api } from '../utils/api';
import type { Task, CreateTaskData } from '../types/task';
import { TaskStatus, Priority } from '../types/task';

const API_BASE = '/api';

// Трансформация данных с бэкенда (snake_case) во фронтенд (camelCase)
const transformTask = (backendTask: any): Task => ({
  id: backendTask.id,
  title: backendTask.title,
  description: backendTask.description,
  priority: backendTask.priority,
  dueDate: backendTask.due_date,
  reminderDays: backendTask.reminder_days,
  reminderHours: backendTask.reminder_hours,
  status: backendTask.status,
  createdAt: backendTask.created_at,
  completedAt: backendTask.completed_at
});
const tasks = ref<Task[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const initialized = ref(false);

export function useTasks() {
  // Загрузка задач с сервера
  const fetchTasks = async () => {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await api.get(`${API_BASE}/tasks`);
      
      if (!response.ok) {
        throw new Error('Ошибка загрузки задач');
      }
      
      const data = await response.json();
      tasks.value = Array.isArray(data) ? data.map(transformTask) : [];
      initialized.value = true;
    } catch (e: any) {
      error.value = e.message;
      console.error('Error fetching tasks:', e);
    } finally {
      loading.value = false;
    }
  };

  // Автоматически загружаем задачи при первом использовании
  if (!initialized.value && !loading.value) {
    fetchTasks();
  }
  // Функция для проверки, просрочена ли задача
  const isOverdue = (task: Task): boolean => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const dueDate = new Date(task.dueDate);
    dueDate.setHours(0, 0, 0, 0);
    return dueDate < today;
  };

  // Функция для проверки, пришло ли время напоминания
  const isReminderTime = (task: Task): boolean => {
    if (!task.reminderDays && !task.reminderHours) return false;
    
    const now = new Date();
    const dueDate = new Date(task.dueDate);
    
    // Вычисляем время напоминания
    const reminderDate = new Date(dueDate);
    if (task.reminderDays) {
      reminderDate.setDate(reminderDate.getDate() - task.reminderDays);
    }
    if (task.reminderHours) {
      reminderDate.setHours(reminderDate.getHours() - task.reminderHours);
    }
    
    return now >= reminderDate && !isOverdue(task);
  };

  // Сортировка задач по приоритету
  const getPriorityOrder = (priority: Priority): number => {
    const order: Record<Priority, number> = {
      [Priority.Critical]: 1,
      [Priority.High]: 2,
      [Priority.Medium]: 3,
      [Priority.Low]: 4
    };
    return order[priority];
  };

  const activeTasks = computed(() => {
    const active = tasks.value.filter(task => task.status === TaskStatus.Active);
    
    return active.sort((a, b) => {
      // 1. Critical всегда сверху
      if (a.priority === Priority.Critical && b.priority !== Priority.Critical) return -1;
      if (b.priority === Priority.Critical && a.priority !== Priority.Critical) return 1;
      
      // Если оба Critical, сортируем по дате
      if (a.priority === Priority.Critical && b.priority === Priority.Critical) {
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
      }
      
      // 2. Просроченные задачи после Critical
      const aOverdue = isOverdue(a);
      const bOverdue = isOverdue(b);
      
      if (aOverdue && !bOverdue) return -1;
      if (bOverdue && !aOverdue) return 1;
      
      // Если обе просрочены, сортируем по дате (старые сначала)
      if (aOverdue && bOverdue) {
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
      }
      
      // 3. Задачи с напоминанием после просроченных
      const aReminder = isReminderTime(a);
      const bReminder = isReminderTime(b);
      
      if (aReminder && !bReminder) return -1;
      if (bReminder && !aReminder) return 1;
      
      // Если у обеих напоминание, сортируем по дате
      if (aReminder && bReminder) {
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
      }
      
      // 4. Остальные по приоритету
      const priorityDiff = getPriorityOrder(a.priority) - getPriorityOrder(b.priority);
      if (priorityDiff !== 0) return priorityDiff;
      
      // Если приоритет одинаковый, сортируем по дате
      return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
    });
  });

  const completedTasks = computed(() => 
    tasks.value.filter(task => task.status === TaskStatus.Completed)
  );

  const createTask = async (data: CreateTaskData): Promise<void> => {
    try {
      const payload = {
        title: data.title,
        description: data.description,
        priority: data.priority,
        due_date: data.dueDate,
        reminder_days: data.reminderDays || null,
        reminder_hours: data.reminderHours || null
      };
      
      console.log('Creating task with payload:', payload);
      const response = await api.post(`${API_BASE}/tasks`, payload);
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        console.error('Create task error:', errorData);
        throw new Error('Ошибка создания задачи');
      }
      
      const newTask = await response.json();
      console.log('Task created:', newTask);
      tasks.value.unshift(transformTask(newTask));
    } catch (e: any) {
      error.value = e.message;
      console.error('Error creating task:', e);
      throw e;
    }
  };

  const completeTask = async (taskId: string): Promise<void> => {
    try {
      const response = await api.patch(`${API_BASE}/tasks/${taskId}/complete`);
      
      if (!response.ok) {
        throw new Error('Ошибка завершения задачи');
      }
      
      const updatedTask = await response.json();
      const index = tasks.value.findIndex(t => t.id === taskId);
      if (index !== -1) {
        tasks.value[index] = transformTask(updatedTask);
      }
    } catch (e: any) {
      error.value = e.message;
      console.error('Error completing task:', e);
      throw e;
    }
  };

  const deleteTask = async (taskId: string): Promise<void> => {
    try {
      const response = await api.delete(`${API_BASE}/tasks/${taskId}`);
      
      if (!response.ok) {
        throw new Error('Ошибка удаления задачи');
      }
      
      const index = tasks.value.findIndex(t => t.id === taskId);
      if (index !== -1) {
        tasks.value.splice(index, 1);
      }
    } catch (e: any) {
      error.value = e.message;
      console.error('Error deleting task:', e);
      throw e;
    }
  };

  const restoreTask = async (taskId: string): Promise<void> => {
    try {
      const response = await api.patch(`${API_BASE}/tasks/${taskId}/restore`);
      
      if (!response.ok) {
        throw new Error('Ошибка восстановления задачи');
      }
      
      const updatedTask = await response.json();
      const index = tasks.value.findIndex(t => t.id === taskId);
      if (index !== -1) {
        tasks.value[index] = transformTask(updatedTask);
      }
    } catch (e: any) {
      error.value = e.message;
      console.error('Error restoring task:', e);
      throw e;
    }
  };

  const updateTask = async (taskId: string, data: CreateTaskData): Promise<void> => {
    try {
      const payload = {
        title: data.title,
        description: data.description,
        priority: data.priority,
        due_date: data.dueDate,
        reminder_days: data.reminderDays || null,
        reminder_hours: data.reminderHours || null
      };
      
      console.log('Updating task with payload:', payload);
      const response = await api.put(`${API_BASE}/tasks/${taskId}`, payload);
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        console.error('Update task error:', errorData);
        throw new Error('Ошибка обновления задачи');
      }
      
      const updatedTask = await response.json();
      console.log('Task updated:', updatedTask);
      const index = tasks.value.findIndex(t => t.id === taskId);
      if (index !== -1) {
        tasks.value[index] = transformTask(updatedTask);
      }
    } catch (e: any) {
      error.value = e.message;
      console.error('Error updating task:', e);
      throw e;
    }
  };

  return {
    tasks,
    activeTasks,
    completedTasks,
    loading,
    error,
    fetchTasks,
    createTask,
    completeTask,
    deleteTask,
    restoreTask,
    updateTask
  };
}
