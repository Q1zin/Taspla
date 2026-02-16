<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h2>{{ editTask ? 'Редактирование задачи' : 'Создание новой задачи' }}</h2>
            <button class="close-button" @click="$emit('close')" aria-label="Закрыть">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>

          <form @submit.prevent="handleSubmit" class="modal-body">
            <div class="form-group">
              <label for="title">Заголовок</label>
              <input
                id="title"
                v-model="formData.title"
                type="text"
                placeholder="Заголовок"
                required
                class="form-input"
              />
            </div>

            <div class="form-group">
              <label for="description">Описание</label>
              <textarea
                id="description"
                v-model="formData.description"
                placeholder="Описание задачи"
                rows="4"
                class="form-input"
              ></textarea>
            </div>

            <div class="form-group">
              <label>Приоритет</label>
              <div class="priority-buttons">
                <button
                  v-for="priority in priorities"
                  :key="priority.value"
                  type="button"
                  class="priority-button"
                  :class="[
                    `priority-${priority.value}`,
                    { active: formData.priority === priority.value }
                  ]"
                  @click="formData.priority = priority.value"
                >
                  {{ priority.label }}
                </button>
              </div>
            </div>

            <div class="form-group">
              <label for="dueDate">К какой дате необходимо выполнить (ДД/ММ/ГГГГ)</label>
              <div class="input-with-icon">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                  <line x1="16" y1="2" x2="16" y2="6"></line>
                  <line x1="8" y1="2" x2="8" y2="6"></line>
                  <line x1="3" y1="10" x2="21" y2="10"></line>
                </svg>
                <input
                  id="dueDate"
                  v-model="formData.dueDate"
                  type="date"
                  required
                  class="form-input"
                  :min="minDate"
                />
              </div>
            </div>

            <div class="form-group">
              <label>За какое время напомнить</label>
              <div class="reminder-inputs">
                <div class="reminder-input-group">
                  <input
                    v-model.number="formData.reminderDays"
                    type="number"
                    min="0"
                    placeholder="0"
                    class="form-input small"
                  />
                  <span class="input-label">дней</span>
                </div>
                <div class="reminder-input-group">
                  <input
                    v-model.number="formData.reminderHours"
                    type="number"
                    min="0"
                    max="23"
                    placeholder="0"
                    class="form-input small"
                  />
                  <span class="input-label">часов</span>
                </div>
              </div>
            </div>

            <div class="modal-footer">
              <button type="button" class="btn-cancel" @click="$emit('close')">
                Отмена
              </button>
              <button type="submit" class="btn-submit">
                {{ editTask ? 'Сохранить изменения' : 'Создать новую задачу' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Priority } from '../types/task';
import type { CreateTaskData, Task } from '../types/task';

const props = defineProps<{
  isOpen: boolean;
  editTask?: Task | null;
}>();

const emit = defineEmits<{
  close: [];
  submit: [data: CreateTaskData, taskId?: string];
}>();

const priorities = [
  { value: Priority.Low, label: 'Low' },
  { value: Priority.Medium, label: 'Medium' },
  { value: Priority.High, label: 'High' },
  { value: Priority.Critical, label: 'Critical' }
];

const formData = ref<CreateTaskData>({
  title: '',
  description: '',
  priority: Priority.Medium,
  dueDate: '',
  reminderDays: undefined,
  reminderHours: undefined
});

const minDate = computed(() => {
  const today = new Date();
  return today.toISOString().split('T')[0];
});

const resetForm = () => {
  formData.value = {
    title: '',
    description: '',
    priority: Priority.Medium,
    dueDate: '',
    reminderDays: undefined,
    reminderHours: undefined
  };
};

watch(() => props.isOpen, (isOpen) => {
  if (isOpen) {
    if (props.editTask) {
      // Заполняем форму данными редактируемой задачи
      formData.value = {
        title: props.editTask.title,
        description: props.editTask.description,
        priority: props.editTask.priority,
        dueDate: props.editTask.dueDate,
        reminderDays: props.editTask.reminderDays,
        reminderHours: props.editTask.reminderHours
      };
    } else {
      // Устанавливаем дату по умолчанию на завтра
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);
      formData.value.dueDate = tomorrow.toISOString().split('T')[0];
    }
  }
});

const handleSubmit = () => {
  if (props.editTask) {
    emit('submit', { ...formData.value }, props.editTask.id);
  } else {
    emit('submit', { ...formData.value });
  }
  resetForm();
  emit('close');
};

const handleOverlayClick = () => {
  emit('close');
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-content {
  background-color: var(--color-bg-card);
  border-radius: 16px;
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 24px 16px;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  color: var(--color-text-secondary);
  border-radius: 6px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  background-color: var(--color-bg);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 24px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.form-input {
  width: 100%;
  padding: 12px 16px;
  border: 1.5px solid var(--color-border);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background-color: var(--color-bg-card);
  transition: all 0.2s;
  font-family: inherit;
}

.form-input:focus {
  outline: none;
  border-color: #7C3AED;
  box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.1);
}

.form-input::placeholder {
  color: var(--color-text-secondary);
}

textarea.form-input {
  resize: vertical;
  min-height: 100px;
}

.input-with-icon {
  position: relative;
}

.input-with-icon svg {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--color-text-secondary);
  pointer-events: none;
}

.input-with-icon .form-input {
  padding-left: 44px;
}

.priority-buttons {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.priority-button {
  padding: 10px 12px;
  border: 2px solid transparent;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  background-color: var(--color-bg-card);
}

.priority-button.priority-low {
  border-color: var(--priority-low-bg);
  color: var(--priority-low-text);
}

.priority-button.priority-low.active {
  background-color: var(--priority-low-bg);
  border-color: var(--priority-low-border);
  color: var(--priority-low-text);
}

.priority-button.priority-medium {
  border-color: var(--priority-medium-bg);
  color: var(--priority-medium-text);
}

.priority-button.priority-medium.active {
  background-color: var(--priority-medium-bg);
  border-color: var(--priority-medium-border);
  color: var(--priority-medium-text);
}

.priority-button.priority-high {
  border-color: var(--priority-high-bg);
  color: var(--priority-high-text);
}

.priority-button.priority-high.active {
  background-color: var(--priority-high-bg);
  border-color: var(--priority-high-border);
  color: var(--priority-high-text);
}

.priority-button.priority-critical {
  border-color: var(--priority-critical-bg);
  color: var(--priority-critical-text);
}

.priority-button.priority-critical.active {
  background-color: var(--priority-critical-bg);
  border-color: var(--priority-critical-border);
  color: var(--priority-critical-text);
}

.reminder-inputs {
  display: flex;
  gap: 12px;
}

.reminder-input-group {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.reminder-input-group .form-input {
  width: 100%;
}

.input-label {
  font-size: 14px;
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.modal-footer {
  display: flex;
  gap: 12px;
  margin-top: 32px;
}

.btn-cancel,
.btn-submit {
  flex: 1;
  padding: 12px 24px;
  border-radius: 10px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-cancel {
  background-color: var(--color-bg-card);
  color: var(--color-primary);
  border: 2px solid var(--color-primary);
}

.btn-cancel:hover {
  background-color: rgba(124, 58, 237, 0.05);
}

.btn-submit {
  background-color: var(--color-primary);
  color: white;
  box-shadow: 0 2px 8px rgba(124, 58, 237, 0.3);
}

.btn-submit:hover {
  background-color: var(--color-primary-dark);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.4);
}

/* Modal transitions */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content {
  transform: scale(0.9) translateY(20px);
}

.modal-leave-to .modal-content {
  transform: scale(0.95);
}

/* Scrollbar styling */
.modal-content::-webkit-scrollbar {
  width: 8px;
}

.modal-content::-webkit-scrollbar-track {
  background: var(--color-bg);
  border-radius: 0 16px 16px 0;
}

.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 4px;
}

.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-secondary);
}
</style>
