import { ref, computed } from 'vue';
import type { Task, CreateTaskData } from '../types/task';
import { TaskStatus, Priority } from '../types/task';

const tasks = ref<Task[]>([
  {
    id: '1',
    title: 'Завершить дизайн приложения',
    description: 'Доработать макеты всех экранов',
    priority: Priority.Critical,
    dueDate: '2026-02-15',
    status: TaskStatus.Active,
    createdAt: '2026-02-10'
  },
  {
    id: '2',
    title: 'Написать тесты',
    description: 'Покрыть тестами основные компоненты',
    priority: Priority.High,
    dueDate: '2026-02-20',
    status: TaskStatus.Active,
    createdAt: '2026-02-11'
  },
  {
    id: '3',
    title: 'Обновить документацию',
    description: 'Добавить примеры использования API',
    priority: Priority.Medium,
    dueDate: '2026-02-18',
    status: TaskStatus.Completed,
    createdAt: '2026-02-09',
    completedAt: '2026-02-12'
  },
  {
    id: '4',
    title: 'Исправить баг в авторизации',
    description: 'Срочная задача - просрочена!',
    priority: Priority.High,
    dueDate: '2026-02-10',
    status: TaskStatus.Active,
    createdAt: '2026-02-08'
  },
  {
    id: '5',
    title: 'Подготовить презентацию',
    description: 'Скоро нужно напомнить',
    priority: Priority.Medium,
    dueDate: '2026-02-16',
    reminderDays: 2,
    status: TaskStatus.Active,
    createdAt: '2026-02-12'
  },
  {
    id: '6',
    title: 'Обновить зависимости',
    description: 'Low приоритет задача',
    priority: Priority.Low,
    dueDate: '2026-02-25',
    status: TaskStatus.Active,
    createdAt: '2026-02-13'
  }
]);

export function useTasks() {
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

  const createTask = (data: CreateTaskData) => {
    const newTask: Task = {
      id: Date.now().toString(),
      ...data,
      status: TaskStatus.Active,
      createdAt: new Date().toISOString()
    };
    tasks.value.unshift(newTask);
  };

  const completeTask = (taskId: string) => {
    const task = tasks.value.find(t => t.id === taskId);
    if (task) {
      task.status = TaskStatus.Completed;
      task.completedAt = new Date().toISOString();
    }
  };

  const deleteTask = (taskId: string) => {
    const index = tasks.value.findIndex(t => t.id === taskId);
    if (index !== -1) {
      tasks.value.splice(index, 1);
    }
  };

  const restoreTask = (taskId: string) => {
    const task = tasks.value.find(t => t.id === taskId);
    if (task) {
      task.status = TaskStatus.Active;
      task.completedAt = undefined;
    }
  };

  const updateTask = (taskId: string, data: CreateTaskData) => {
    const task = tasks.value.find(t => t.id === taskId);
    if (task) {
      task.title = data.title;
      task.description = data.description;
      task.priority = data.priority;
      task.dueDate = data.dueDate;
      task.reminderDays = data.reminderDays;
      task.reminderHours = data.reminderHours;
    }
  };

  return {
    tasks,
    activeTasks,
    completedTasks,
    createTask,
    completeTask,
    deleteTask,
    restoreTask,
    updateTask
  };
}
