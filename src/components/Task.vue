<script setup lang="ts">

const props = defineProps({
  description: String, 
  isCompleted: Boolean
});

</script>

<template>
  <div class="task">
    <input type="checkbox" v-model="props.isCompleted" class="task-checkbox" />
    <input type="text" class="task-textbox" :value="props.description" />
  </div>
</template>

<style scoped>
div.task {
  width: 100%;
}

input[type="checkbox"].task-checkbox {
  appearance: none;
  width: 32px;
  height: 32px;
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
  font-size: 48px;
  position: absolute;
  top: -24px;
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
  font-size: 24px;
  border-top: none;
  border-left: none;
  border-right: none;
  border-bottom: 2px solid gray;
  box-sizing: border-box;
  vertical-align: middle;
  line-height: 24px;
  white-space: nowrap;
}

input[type="text"].task-textbox:focus {
  outline: none;
  border-bottom: 2px solid black;
}
</style>
