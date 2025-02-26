<script setup lang="ts">
import { onMounted, Ref, ref, watch, watchEffect } from 'vue';
import Task from './components/Task.vue';

// 禁用右键菜单
const disableContextMenu = () => {
  document.addEventListener("contextmenu", (e: MouseEvent) => {
    e.preventDefault();
    return false;
  }, { capture: true });
}

const title: Ref<string> = ref("");

const tasks = ref([
  { description: '任务 1', isCompleted: false },
  { description: '任务 2', isCompleted: true },
  { description: '任务 3', isCompleted: false },
]);

// 更新任务状态
const updateTask = (index: number, newTask: { description: string; isCompleted: boolean }) => {
  tasks.value[index] = newTask;
};

onMounted(() => {
  disableContextMenu();
});

watchEffect(() => {
  tasks.value.forEach((task) => {
    console.log(task);
  });
});

</script>

<template>
  <main class="container">
    <input type="text" class="title" placeholder="待办事项" :value="title" />
    <div class="task-container">
      <task v-for="(task, index) in tasks" :key="index" :description="task.description" :is-completed="task.isCompleted"
        @update="(newTask) => updateTask(index, newTask)" />
    </div>
  </main>
</template>

<style scoped></style>
<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app,
html,
body {
  height: 100%;
}

main.container {
  height: 100%;
  width: 100%;
  margin: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  background-color: #FCFAEB;
  gap: 2em;
  overflow: auto;
}

div.task-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5em;
  width: 90%;
  margin-bottom: 2em;
}

input[type="text"].title {
  margin-top: 1em;
  width: 60%;
  background-color: transparent;
  margin-left: 0.5em;
  font-size: 24px;
  border-top: none;
  border-left: none;
  border-right: none;
  border-bottom: 2px solid gray;
  box-sizing: border-box;
  vertical-align: middle;
  text-align: center;
  line-height: 25px;
}

input[type="text"].title::placeholder {
  user-select: none;
}

input[type="text"].title:focus {
  outline: none;
  border-bottom: 2px solid black;
}

input[type="text"].title:focus::placeholder {
  opacity: 0;
}
</style>