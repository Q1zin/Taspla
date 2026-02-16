<template>
  <div 
    class="task-card" 
    :class="{ 'is-completed': task.status === TaskStatus.Completed }"
  >
    <div class="task-card-inner">
      <div class="task-content">
        <div class="task-header">
          <h3 class="task-title" :class="{ 'is-completed': task.status === TaskStatus.Completed }">
            {{ task.title }}
          </h3>
        </div>
        
        <p v-if="task.description" class="task-description">{{ task.description }}</p>
        
        <div class="task-footer">
          <div class="task-meta">
            <span class="priority-badge" :class="`priority-${task.priority}`">
              {{ priorityLabels[task.priority] }}
            </span>
            <div class="task-date">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                <line x1="16" y1="2" x2="16" y2="6"></line>
                <line x1="8" y1="2" x2="8" y2="6"></line>
                <line x1="3" y1="10" x2="21" y2="10"></line>
              </svg>
              <span>{{ formattedDate }}</span>
            </div>
          </div>
          
          <div class="task-actions">
            <button 
              v-if="task.status === TaskStatus.Active"
              class="btn-edit"
              @click="$emit('edit', task)"
              title="Редактировать"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
              </svg>
            </button>
            <button 
              v-if="task.status === TaskStatus.Active"
              class="btn-done"
              @click="$emit('complete', task.id)"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            </button>
            
            <template v-else>
              <button 
                class="btn-restore"
                @click="$emit('restore', task.id)"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
                  <path d="M21 3v5h-5"></path>
                  <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
                </svg>
              </button>
              <button 
                class="btn-delete"
                @click="$emit('delete', task.id)"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                </svg>
              </button>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Task } from '../types/task';
import { Priority, TaskStatus } from '../types/task';

const props = defineProps<{
  task: Task;
}>();

defineEmits<{
  complete: [taskId: string];
  delete: [taskId: string];
  restore: [taskId: string];
  edit: [task: Task];
}>();

const priorityLabels: Record<Priority, string> = {
  [Priority.Low]: 'Low',
  [Priority.Medium]: 'Medium',
  [Priority.High]: 'High',
  [Priority.Critical]: 'Critical'
};

const formattedDate = computed(() => {
  const date = new Date(props.task.dueDate);
  const day = String(date.getDate()).padStart(2, '0');
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const year = date.getFullYear();
  return `${day}/${month}/${year}`;
});
</script>

<style scoped>
.task-card {
  margin-bottom: 12px;
  border-radius: 12px;
}

.task-card-inner {
  background-color: var(--color-bg-card);
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.2s;
}

.task-card:hover .task-card-inner {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.task-content {
  padding: 16px;
}

.task-header {
  margin-bottom: 8px;
}

.task-title {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  line-height: 1.4;
  transition: all 0.3s;
}

.task-title.is-completed {
  text-decoration: line-through;
  color: var(--color-text-secondary);
}

.priority-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.priority-low {
  background-color: var(--priority-low-bg);
  color: var(--priority-low-text);
}

.priority-medium {
  background-color: var(--priority-medium-bg);
  color: var(--priority-medium-text);
}

.priority-high {
  background-color: var(--priority-high-bg);
  color: var(--priority-high-text);
}

.priority-critical {
  background-color: var(--priority-critical-bg);
  color: var(--priority-critical-text);
}

.task-description {
  margin: 0 0 12px;
  font-size: 14px;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.task-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.task-date {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.task-date svg {
  opacity: 0.7;
}

.task-actions {
  display: flex;
  gap: 8px;
}

button {
  display: flex;
  align-items: center;
  gap: 4px;
  border: none;
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-edit {
  background-color: #3B82F6;
  color: white;
  padding: 6px 10px;
}

.btn-edit:hover {
  background-color: #2563EB;
}

.btn-done {
  background-color: #4ADE80;
  color: white;
  padding: 6px 10px;
}

.btn-done:hover {
  background-color: #22C55E;
}

.btn-restore {
  background-color: #3B82F6;
  color: white;
  padding: 6px 10px;
}

.btn-restore:hover {
  background-color: #2563EB;
}

.btn-delete {
  background-color: #FCA5A5;
  color: white;
  padding: 6px 10px;
}

.btn-delete:hover {
  background-color: #F87171;
}

.is-completed .task-card-inner {
  opacity: 0.8;
}
</style>
