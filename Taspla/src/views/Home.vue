<template>
  <div class="home-view">
    <AppHeader 
      @toggle-menu="isMenuOpen = true"
      @create-task="isModalOpen = true"
    />
    
    <TaskList 
      :active-tasks="activeTasks"
      :completed-tasks="completedTasks"
      :active-tab="activeTab"
      :show-tabs="true"
      @complete="handleCompleteTask"
      @delete="handleDeleteTask"
      @restore="handleRestoreTask"
      @edit="handleEditTask"
      @update:active-tab="activeTab = $event"
    />

    <SideMenu 
      :is-open="isMenuOpen"
      @close="isMenuOpen = false"
    />

    <CreateTaskModal
      :is-open="isModalOpen"
      :edit-task="editingTask"
      @close="handleCloseModal"
      @submit="handleSubmitTask"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import AppHeader from '../components/AppHeader.vue';
import TaskList from '../components/TaskList.vue';
import SideMenu from '../components/SideMenu.vue';
import CreateTaskModal from '../components/CreateTaskModal.vue';
import { useTasks } from '../composables/useTasks';
import type { CreateTaskData, Task } from '../types/task';

const { activeTasks, completedTasks, createTask, completeTask, deleteTask, restoreTask, updateTask } = useTasks();

const isMenuOpen = ref(false);
const isModalOpen = ref(false);
const editingTask = ref<Task | null>(null);
const activeTab = ref<'active' | 'completed'>('active');

const handleSubmitTask = (data: CreateTaskData, taskId?: string) => {
  if (taskId) {
    updateTask(taskId, data);
  } else {
    createTask(data);
  }
};

const handleEditTask = (task: Task) => {
  editingTask.value = task;
  isModalOpen.value = true;
};

const handleCloseModal = () => {
  isModalOpen.value = false;
  editingTask.value = null;
};

const handleCompleteTask = (taskId: string) => {
  completeTask(taskId);
};

const handleDeleteTask = (taskId: string) => {
  deleteTask(taskId);
};

const handleRestoreTask = (taskId: string) => {
  restoreTask(taskId);
};
</script>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--color-bg);
  overflow: hidden;
}
</style>
