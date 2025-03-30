<script setup lang="ts">
import { onMounted, Ref, ref, watch } from 'vue';
import Task from './components/Task.vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ClipboardType } from './types.ts';


// 禁用右键菜单
const disableContextMenu = () => {
  document.addEventListener("contextmenu", (e: MouseEvent) => {
    e.preventDefault();
  }, { capture: true }
  );
};

const clipboards: Ref<ClipboardType[]> = ref([]);
const clipboardIndex: Ref<number> = ref(0);

const fetchTasks = async () => {
  const data = await invoke("read_data") as {
    clipboards: Array<{
      title: string;
      tasks: Array<{ description: string; is_completed: boolean }>;
    }>;
  };

  clipboards.value = data.clipboards.map(clipboard => ({
    title: clipboard.title,
    tasks: clipboard.tasks.map(task => ({
      description: task.description,
      isCompleted: task.is_completed,
    })),
  }));
};

const saveData = async () => {
  const data = {
    clipboards: clipboards.value.map(clipboard => ({
      title: clipboard.title,
      tasks: clipboard.tasks.map(task => ({
        description: task.description,
        is_completed: task.isCompleted,
      })),
    })),
  };
  await invoke("write_data", { data });
};

const removeDoneTasks = async (clipboardIndex: number) => {
  await invoke("remove_done_tasks", { clipboardIndex });
  fetchTasks();
};

onMounted(() => {
  disableContextMenu();
  fetchTasks();
});

watch(clipboards, () => {
  saveData();
}, { deep: true }
);

const playClickSound = () => {
  const audio = new Audio("/click.wav");
  audio.play();
  audio.volume = 0.25;
};

const focusedIndex: Ref<number> = ref(-1);

const handleKeyDown = () => {
  focusedIndex.value =
    focusedIndex.value >= clipboards.value[clipboardIndex.value].tasks.length - 1
      ? focusedIndex.value
      : focusedIndex.value + 1;
  focusTask();
};

const handleKeyUp = () => {
  focusedIndex.value =
    focusedIndex.value <= 0
      ? focusedIndex.value
      : focusedIndex.value - 1;
  focusTask();
};

const focusTask = () => {
  const taskElements = document.querySelectorAll<HTMLInputElement>('.task-textbox');
  if (focusedIndex.value >= 0 && focusedIndex.value < taskElements.length) {
    taskElements[focusedIndex.value].focus();
  }
};
</script>

<template>
  <main class="container">
    <div class="page-container">
      <div data-tauri-drag-region class="inner-page-container" @keydown.down.prevent="handleKeyDown"
        @keydown.up.prevent="handleKeyUp">
        <div>
          <input type="text" class="title" placeholder="待办事项" v-if="clipboards.length > 0"
            v-model="clipboards[clipboardIndex].title" />
          <p class="current-tasks-tip"></p>
        </div>
        <div class="outer-task-container">
            <button class="change-clipboard-button change-clipboard-button__left"
              @click="playClickSound(); --clipboardIndex" :style="{ visibility: clipboardIndex > 0 ? 'visible' : 'hidden' }"/>
          <div class="task-container">
            <task v-if="clipboards.length > 0" v-for="(task, index) in clipboards[clipboardIndex].tasks" :key="index"
              v-model:description="task.description" v-model:is-completed="task.isCompleted"
              @task-focused="focusedIndex = index" />
          </div>
          <button :style="{ visibility: clipboardIndex < clipboards.length -1 ? 'visible' : 'hidden' }"
            class="change-clipboard-button change-clipboard-button__right"
            @click="playClickSound(); ++clipboardIndex"></button>
        </div>
      </div>
    </div>
    <div class="menu-container">
      <button id="clear-button" @click="playClickSound(); removeDoneTasks(clipboardIndex)" title="清空并排序所有任务" />
      <button id="close-button" @click="getCurrentWindow().close()" title="收起写字板" />
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
  gap: 1em;
  overflow: auto;
}

div.menu-container {
  position: absolute;
  top: 65%;
  right: 0;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5em;
}

button#close-button,
button#clear-button {
  width: 40px;
  height: 40px;
  background-size: cover;
  background-repeat: no-repeat;
  border: none;
  cursor: pointer;
  user-select: none;
}

button#close-button {
  background-image: url("/close-button-inactive.png");
}

button#clear-button {
  background-image: url("/clear-button-inactive.png");
}

button#clear-button:hover {
  background-image: url("/clear-button-hover.png");
}

button#close-button:hover {
  background-image: url("/close-button-hover.png");
}

button#close-button:active {
  background-image: url("/close-button-active.png");
}

button#clear-button:active {
  background-image: url("/clear-button-active.png");
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

p.current-tasks-tip {
  margin-top: 2px;
}

div.outer-task-container {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 2px;
  width: 100%;
  margin-top: 1em;

}

button.change-clipboard-button {
  width: 40px;
  height: 40px;

  /* 让翻页按钮在页面水平中心 */
  --transform-value: translateY(-20px);
  transform: var(--transform-value);

  background-color: transparent;
  background-repeat: no-repeat;
  background-size: cover;
  border: none;
  cursor: pointer;
  user-select: none;

  transition: background-image 0.1s;
}

button.change-clipboard-button__left {
  background-image: url("/change-clipboard-left-button-inactive.png");
}

button.change-clipboard-button__left:hover {
  background-image: url("/change-clipboard-left-button-hover.png");
}

button.change-clipboard-button__right {
  transform: translateX(-4px) var(--transform-value);
  background-image: url("/change-clipboard-right-button-inactive.png");
}

button.change-clipboard-button__right:hover {
  background-image: url("/change-clipboard-right-button-hover.png");
}
</style>