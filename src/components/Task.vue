<script setup lang="ts">
import { ModelRef } from 'vue';


// 创建双向绑定的模型
const isCompleted: ModelRef<boolean> = defineModel("isCompleted", { type: Boolean, required: true });
const description: ModelRef<string> = defineModel("description", { type: String, required: true });

// 按钮音效
const playSound = () => {
  const audio = new Audio("/click.wav");
  audio.play();
  audio.volume = 0.25;
};

</script>

<template>
  <div class="task">
    <input type="checkbox" class="task-checkbox" v-model="isCompleted" @click="playSound" />
    <input type="text" class="task-textbox" v-model="description"/>
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
  content: '✔';
  color: green;
  font-size: 36px;
  position: absolute;
  top: -16px;
  left: -2px;
  pointer-events: none;
}

input[type="checkbox"].task-checkbox:focus {
  outline: none;
}

input[type="text"].task-textbox {
  width: calc(100% - 50px);
  background-color: transparent;
  margin-left: 0.5em;
  font-size: 20px;
  border-top: none;
  border-left: none;
  border-right: none;
  border-bottom: 2px solid gray;
  box-sizing: border-box;
  vertical-align: middle;
  line-height: 20px;
  white-space: nowrap;
}

input[type="text"].task-textbox:focus {
  outline: none;
  border-bottom: 2px solid black;
}
</style>
