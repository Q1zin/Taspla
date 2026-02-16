<template>
  <div class="task-list">
    <div v-if="showTabs" class="task-tabs">
      <button 
        class="task-tab"
        :class="{ active: activeTab === 'active' }"
        @click="$emit('update:activeTab', 'active')"
      >
        Активные
        <span class="tab-count">{{ activeTasks.length }}</span>
      </button>
      <button 
        class="task-tab"
        :class="{ active: activeTab === 'completed' }"
        @click="$emit('update:activeTab', 'completed')"
      >
        Завершенные
        <span class="tab-count">{{ completedTasks.length }}</span>
      </button>
    </div>

    <div class="tasks-container">
      <TransitionGroup name="list" tag="div">
        <div v-if="currentTasks.length === 0" key="empty" class="empty-state">
          <svg width="120" height="120" viewBox="0 0 120 120" fill="none">
            <circle cx="60" cy="60" r="50" fill="#E0E7FF"/>
            <path d="M40 60L55 75L80 45" stroke="#7C3AED" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <h3>{{ emptyStateText }}</h3>
          <p>{{ emptyStateDescription }}</p>
        </div>

        <TaskCard
          v-for="task in currentTasks"
          :key="task.id"
          :task="task"
          @complete="$emit('complete', $event)"
          @delete="$emit('delete', $event)"
          @restore="$emit('restore', $event)"
          @edit="$emit('edit', $event)"
        />
      </TransitionGroup>
    </div>

    <div v-if="totalTasks > 0" class="progress-indicator">
      <div class="progress-text">
        <span class="progress-label">Прогресс выполнения</span>
        <span class="progress-value">{{ completedTasks.length }} из {{ totalTasks }}</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: progressPercentage + '%' }"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import TaskCard from './TaskCard.vue';
import type { Task } from '../types/task';

const props = defineProps<{
  activeTasks: Task[];
  completedTasks: Task[];
  activeTab?: 'active' | 'completed';
  showTabs?: boolean;
}>();

defineEmits<{
  complete: [taskId: string];
  delete: [taskId: string];
  restore: [taskId: string];
  edit: [task: Task];
  'update:activeTab': [tab: 'active' | 'completed'];
}>();

const currentTasks = computed(() => {
  if (!props.showTabs) {
    return [...props.activeTasks, ...props.completedTasks];
  }
  return props.activeTab === 'active' ? props.activeTasks : props.completedTasks;
});

const totalTasks = computed(() => props.activeTasks.length + props.completedTasks.length);

const progressPercentage = computed(() => {
  if (totalTasks.value === 0) return 0;
  return Math.round((props.completedTasks.length / totalTasks.value) * 100);
});

const emptyStateText = computed(() => {
  if (!props.showTabs) {
    return 'Нет задач';
  }
  return props.activeTab === 'active' ? 'Все задачи выполнены! 🎉' : 'Нет завершенных задач';
});

const emptyStateDescription = computed(() => {
  if (!props.showTabs) {
    return 'Создайте свою первую задачу';
  }
  return props.activeTab === 'active' 
    ? 'Отличная работа! Создайте новые задачи чтобы продолжить.'
    : 'Здесь будут отображаться завершенные задачи';
});
</script>

<style scoped>
.task-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.task-tabs {
  display: flex;
  gap: 8px;
  padding: 16px 20px 0;
  background-color: var(--color-bg);
}

.task-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 16px;
  background-color: transparent;
  border: none;
  border-radius: 12px 12px 0 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.task-tab.active {
  background-color: var(--color-bg-card);
  color: var(--color-primary);
}

.tab-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 8px;
  background-color: var(--color-bg);
  border-radius: 12px;
  font-size: 13px;
  font-weight: 700;
}

.task-tab.active .tab-count {
  background-color: #EDE9FE;
  color: var(--color-primary);
}

[data-theme="dark"] .task-tab.active .tab-count {
  background-color: rgba(139, 92, 246, 0.2);
  color: var(--color-primary);
}

.tasks-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  background-color: var(--color-bg);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-state svg {
  margin-bottom: 24px;
}

.empty-state h3 {
  margin: 0 0 8px;
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.empty-state p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-secondary);
}

.progress-indicator {
  padding: 16px 20px 20px;
  background-color: var(--color-bg-card);
  border-top: 1px solid var(--color-border);
}

.progress-text {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.progress-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.progress-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--color-primary);
}

.progress-bar {
  height: 8px;
  background-color: var(--color-bg);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #7C3AED 0%, #A78BFA 100%);
  border-radius: 4px;
  transition: width 0.5s ease;
}

/* List transitions */
.list-enter-active {
  transition: opacity 0.3s ease;
}

.list-leave-active {
  transition: opacity 0.2s ease;
  position: absolute;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
}

.list-move {
  transition: transform 0.3s ease;
}
</style>
