export enum Priority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

export enum TaskStatus {
  Active = 'active',
  Completed = 'completed'
}

export interface Task {
  id: string;
  title: string;
  description: string;
  priority: Priority;
  dueDate: string;
  reminderDays?: number;
  reminderHours?: number;
  status: TaskStatus;
  createdAt: string;
  completedAt?: string;
}

export interface CreateTaskData {
  title: string;
  description: string;
  priority: Priority;
  dueDate: string;
  reminderDays?: number;
  reminderHours?: number;
}
