<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, onUnmounted, Ref, ref, watch, watchEffect } from 'vue';
import Task from './components/Task.vue';
import { ClipboardType, Option } from './types.ts';
import { debounce } from './utils.ts';

const clipboards: Ref<ClipboardType[]> = ref([]);
const clipboardIndex: Ref<number> = ref(0);

const fetchData = async () => {
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
  fetchData();
};

const playSound = (path: string, volume: number = 1) => {
  const audio = new Audio(path);
  audio.play();
  audio.volume = volume;
};

const playPageSound = () => {
  const soundPath: string = (Math.round(Math.random()) === 1) ? './sound/open_flip1.ogg' : './sound/open_flip2.ogg';
  playSound(soundPath, 0.25);
};

const focusedIndex: Ref<number> = ref(-1);

const handleKeyDown = () => {
  if (focusedIndex.value < clipboards.value[clipboardIndex.value].tasks.length - 1) {
    focusedIndex.value += 1;
  }
};

const handleKeyUp = () => {
  if (focusedIndex.value > 0) {
    focusedIndex.value -= 1;
  }
};

const changeClipboardMillisecond: number = 75;

const handleKeyLeft = debounce(() => {
  if (canChangeClipboard.value && clipboardIndex.value > 0) {
    clipboardIndex.value -= 1;
  }
}, changeClipboardMillisecond);

const handleKeyRight = debounce(() => {
  if (canChangeClipboard.value && clipboardIndex.value < clipboards.value.length - 1) {
    clipboardIndex.value += 1;
  }
}, changeClipboardMillisecond);

const taskRefs = ref<Option<InstanceType<typeof Task>[]>>(null);
const titleRef = ref<Option<HTMLInputElement>>(null);

let canChangeClipboard = ref<boolean>(true);

const updateCanChangeClipboard = (target: Option<HTMLElement>) => {
  canChangeClipboard.value = !(
    target?.tagName === 'INPUT' &&
    (target === titleRef.value || target.classList.contains('task-textbox'))
  );
}

const handleKeyChangeClipboard = (e: KeyboardEvent) => {
  if (e.key === 'ArrowLeft') {
    handleKeyLeft();
  } else if (e.key === 'ArrowRight') {
    handleKeyRight();
  }
};

const handleKeyFocus = (e: KeyboardEvent) => {
  if (canChangeClipboard.value) {
    if (e.key === 'ArrowUp') {
      titleRef.value?.focus();
    } else if (e.key === 'ArrowDown') {
      if (taskRefs.value) {
        taskRefs.value[0].textboxRef!.focus();
      }
    }
  }
};

const closeWindow = (flag: boolean) => {
  if (flag) {
    getCurrentWindow().close();
  }
};

const handleEscape = (e: KeyboardEvent) => {
  if (e.key !== 'Escape') return;

  if (currentFocusedTarget.value?.tagName === 'INPUT') {
    currentFocusedTarget.value.blur();
  } else if (canChangeClipboard.value) {
    closeWindow(true);
  }
};

const currentFocusedTarget = ref<Option<HTMLElement>>(null);

const handleFocus = (e: FocusEvent) => {
  currentFocusedTarget.value = e.target as HTMLElement;
};

const handleBlur = () => {
  currentFocusedTarget.value = null;
};

onMounted(() => {
  fetchData();

  document.addEventListener('keydown', handleKeyChangeClipboard, { passive: true });
  document.addEventListener('keydown', handleEscape, { passive: true });
  document.addEventListener('keydown', handleKeyFocus, { passive: true });


  document.addEventListener('focus', handleFocus, { passive: true, capture: true });
  document.addEventListener('blur', handleBlur, { passive: true, capture: true });
});

onUnmounted(() => {
  document.removeEventListener('focus', handleFocus);
  document.removeEventListener('blur', handleBlur);

  document.removeEventListener('keydown', handleKeyChangeClipboard);
  document.removeEventListener('keydown', handleEscape);
  document.removeEventListener('keydown', handleKeyFocus);
});

watch(clipboards, saveData, { deep: true });
watch(clipboardIndex, playPageSound);
watch(currentFocusedTarget, () => {
  updateCanChangeClipboard(currentFocusedTarget.value!);
}, { immediate: true });

watchEffect(() => { console.log(focusedIndex.value) });
</script>

<template>
  <main class="container">
    <div class="page-container">
      <!-- @contextmenu.prevent 阻止右键开发者菜单 -->
      <div class="inner-page-container" @contextmenu.prevent @keydown.up.prevent="handleKeyUp"
        @keydown.down.prevent="handleKeyDown">
        <div class="page__header">
          <input type="text" class="title" v-if="clipboards.length !== 0" :placeholder="`待办事项 ${clipboardIndex + 1}`"
            v-model="clipboards[clipboardIndex].title" ref="titleRef" />
        </div>
        <div class="outer-task-container">
          <button class="change-clipboard-button change-clipboard-button_left" @click="--clipboardIndex"
            :style="{ visibility: clipboardIndex > 0 ? 'visible' : 'hidden' }" />
          <div class="task-container">
            <task v-if="clipboards.length !== 0" v-for="(task, index) in clipboards[clipboardIndex].tasks" :key="index"
              v-model:description="task.description" v-model:is-completed="task.isCompleted"
              @task-focused="focusedIndex = index" :has-focused="focusedIndex === index" ref="taskRefs" />
          </div>
          <button :style="{ visibility: clipboardIndex < clipboards.length - 1 ? 'visible' : 'hidden' }"
            class="change-clipboard-button change-clipboard-button_right" @click="++clipboardIndex"></button>
        </div>
      </div>
    </div>
    <div class="menu-container">
      <button id="menu__button--clear" @click="playSound('./sound/click.wav', 0.25); removeDoneTasks(clipboardIndex)"
        title="清空并规整当前任务组" />
      <button id="menu__button--close" @click="closeWindow(true)" title="收起写字板" />
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
  background-image: url("/img/background.png");
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

button#menu__button--close,
button#menu__button--clear {
  width: 40px;
  height: 40px;
  background-size: cover;
  background-repeat: no-repeat;
  border: none;
  cursor: pointer;
  user-select: none;
}

button#menu__button--close {
  background-image: url("/img/close-button-inactive.png");
}

button#menu__button--clear {
  background-image: url("/img/clear-button-inactive.png");
}

button#menu__button--clear:hover {
  background-image: url("/img/clear-button-hover.png");
}

button#menu__button--close:hover {
  background-image: url("/img/close-button-hover.png");
}

button#menu__button--close:active {
  background-image: url("/img/close-button-active.png");
}

button#menu__button--clear:active {
  background-image: url("/img/clear-button-active.png");
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
  user-select: none;
}

button.change-clipboard-button_left {
  background-image: url("/img/change-clipboard-left-button.png");
}

button.change-clipboard-button_left:hover {
  background-image: url("/img/change-clipboard-left-button-hover.png");
}

button.change-clipboard-button_right {
  transform: translateX(-4px) var(--transform-value);
  background-image: url("/img/change-clipboard-right-button.png");
}

button.change-clipboard-button_right:hover {
  background-image: url("/img/change-clipboard-right-button-hover.png");
}
</style>