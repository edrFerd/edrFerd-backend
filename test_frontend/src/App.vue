<script setup>
import {ref} from "vue";

const message = ref('')

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
    <p v-if="message">服务器返回: {{ message }}</p>
  </div>
</template>

<style scoped>
</style>
