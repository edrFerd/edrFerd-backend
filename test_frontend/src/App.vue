<script setup>
import HelloWorld from './components/HelloWorld.vue'
import { ref } from 'vue'

const message = ref('')

async function callTestSend() {
  try {
    const response = await fetch('http://127.0.0.1:1414/test_send')
    const data = await response.text()
    message.value = data
    console.log(data)
  } catch (error) {
    const errorMsg = `Error: ${error.message}`;
    message.value = errorMsg
    console.error('Error calling test_send:', error)
  }
}
</script>

<template>
  <div>
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld msg="Vite + Vue" />

  <div>
    <button @click="callTestSend">发送共识包</button>
    <p v-if="message">服务器返回: {{ message }}</p>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
