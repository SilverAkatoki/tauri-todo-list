<script setup lang="ts">
import { onMounted, Ref, ref, watch } from 'vue';
import Task from './components/Task.vue';
import { invoke } from '@tauri-apps/api/core';


// 禁用右键菜单
const disableContextMenu = () => {
  document.addEventListener("contextmenu", (e: MouseEvent) => {
    e.preventDefault();
    return false;
  }, { capture: true });
}

const title: Ref<string> = ref("");

const tasks = ref<Array<{ description: string; isCompleted: boolean }>>([]);

const fetchTasks = async () => {
  const data = await invoke("read_data") as { title: string; tasks: Array<{ description: string; is_completed: boolean }> };
  title.value = data.title;
  tasks.value = data.tasks.map(task => ({
    description: task.description,
    isCompleted: task.is_completed
  }));
};

const saveData = async () => {
  const data = {
    title: title.value,
    tasks: tasks.value.map(task => ({
      description: task.description,
      is_completed: task.isCompleted
    }))
  };
  await invoke("write_data", { data });
};

onMounted(() => {
  disableContextMenu();
  fetchTasks();
});

watch([title, tasks], () => {
  saveData();
}, { deep: true });


import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
// TODO 能不能塞进 button 的 Click 事件中

</script>

<template>
  <main class="container">
    <div class="page-container">
      <div data-tauri-drag-region class="inner-page-container">
        <input type="text" class="title" placeholder="待办事项" v-model="title" />
        <div class="task-container">
          <task v-for="(task, index) in tasks" :key="index" v-model:description="task.description"
            v-model:is-completed="task.isCompleted" />
        </div>
      </div>
    </div>
    <div class="menu-container">
      <button id="close-button" @click="appWindow.close()" />
    </div>
  </main>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  image-rendering: pixelated;
}

#app,
html,
body {
  height: 100%;
}

main.container {
  height: 100%;
  display: flex;
  flex-direction: row;
  gap: 8px;
  background: transparent;
}

div.page-container {
  height: 100%;
  width: 90%;
  background-image: url("/background.png");
  background-repeat: no-repeat;
  background-size: contain;
}

div.inner-page-container {
  padding: 5em 2em 0 2em;

  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 2em;
  overflow: auto;
}

button#close-button {
  position: relative;
  top: 75%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 40px;
  height: 40px;
  background-image: url("/close-button-inactive.png");
  background-size: cover;
  background-repeat: no-repeat;
  border: none;
  cursor: pointer;
  user-select: none;
}

button#close-button:active {
  background-image: url("/close-button-active.png");
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