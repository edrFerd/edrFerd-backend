<script setup>
import {ref} from "vue";

const message = ref('')
const worldData = ref(null)

async function showWorld() {
  try {
    const response = await fetch('http://127.0.0.1:1415/show_world')
    const data = await response.json()
    worldData.value = JSON.stringify(data, null, 2)
    console.log(data)
  } catch (error) {
    worldData.value = `Error: ${error.message}`
    console.error('Error calling show_world:', error)
  }
}

async function callTestSend() {
  try {
    const response = await fetch('http://127.0.0.1:1415/test_send')
    const data = await response.text()
    message.value = data
    console.log(data)
  } catch (error) {
    message.value = `Error: ${error.message}`
    console.error('Error calling test_send:', error)
  }
}
</script>

<template>
  <div>
    <h1>EdrFerd 测试</h1>
    <button @click="callTestSend">发送共识包</button>
    <button @click="showWorld">显示世界</button>
    <p v-if="message">服务器返回: {{ message }}</p>
    <div v-if="worldData">
      <h3>世界数据:</h3>
      <pre>{{ worldData }}</pre>
    </div>
  </div>
</template>

<style scoped>
</style>
