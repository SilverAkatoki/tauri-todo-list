<script setup lang="ts">
import { ModelRef, Ref, ref, watch } from 'vue';
import { Option } from '../types';


// 双向绑定的模型
const isCompleted: ModelRef<boolean> = defineModel("isCompleted", { type: Boolean, required: true });
const description: ModelRef<string> = defineModel("description", { type: String, required: true });

const props = defineProps<{
  hasFocused: boolean;
}>();

// 不能直接操作 DOM，所以用 ref 创建一个响应式对象
const textboxRef: Ref<Option<HTMLInputElement>> = ref<Option<HTMLInputElement>>(null);

defineExpose({
  textboxRef,
});

// 用 defineEmits 定义事件出的 emit 函数，不是 tauri 的 emit
const emit = defineEmits(["task-focused"]);

// 按钮音效
const playClickSound = () => {
  const sound = new Audio(isCompleted.value ? "/sound/clipboard-erase.ogg" : "/sound/clipboard-check.ogg");
  sound.volume = 0.5;
  sound.play();
};

// 在这里执行焦点操作
watch(() => props.hasFocused, (hasFocus) => {
  if (hasFocus && textboxRef.value) {
    textboxRef.value.focus();
  }
});

</script>

<template>
  <div class="task">
    <input type="checkbox" class="task-checkbox" v-model="isCompleted" @click="playClickSound()" />
    <input type="text" class="task-textbox" :class="{ 'done': isCompleted }" v-model="description"
      @focus="emit('task-focused')" ref="textboxRef" />
  </div>
</template>

<style scoped>
div.task {
  width: 100%;
}

input[type="checkbox"].task-checkbox {
  appearance: none;
  width: 24px;
  height: 24px;
  border: 2px solid gray;
  border-radius: 0;
  background-color: transparent;
  position: relative;
  cursor: pointer;
  vertical-align: middle;
}

input[type="checkbox"].task-checkbox:checked::after {
  content: "✔";
  color: green;
  font-size: 36px;
  position: absolute;
  top: -16px;
  left: -2px;
  pointer-events: none;
}

input[type="text"].done {
  color: green;
}

input[type="checkbox"].task-checkbox:focus {
  outline: none;
}

input[type="text"].task-textbox {
  width: calc(100% - 50px);
  background-color: transparent;
  margin-left: 0.5em;
  font-size: 20px;
  border: none;
  box-sizing: border-box;
  vertical-align: middle;
  line-height: 20px;
  white-space: nowrap;
}

input[type="text"].task-textbox:focus {
  outline: none;
}
</style>
