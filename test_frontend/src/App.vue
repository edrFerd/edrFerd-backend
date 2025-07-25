<script setup>
import {ref} from "vue";

const message = ref('')
const worldData = ref(null)
const blockX = ref(0)
const blockY = ref(0)
const blockZ = ref(0)
const blockType = ref('default_block')

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

async function sendBlock() {
  const block = {
    point: {
      x: parseInt(blockX.value),
      y: parseInt(blockY.value),
      z: parseInt(blockZ.value),
    },
    block_appearance: {
      type_id: blockType.value,
    },
  };

  try {
    const response = await fetch('http://127.0.0.1:1415/send_block', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(block),
    });
    const data = await response.text();
    message.value = data;
    console.log(data);
  } catch (error) {
    message.value = `Error: ${error.message}`;
    console.error('Error calling send_block:', error);
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

    <div class="custom-block-sender">
      <h3>发送自定义方块</h3>
      <div>
        <label>X: <input v-model="blockX" type="number" /></label>
        <label>Y: <input v-model="blockY" type="number" /></label>
        <label>Z: <input v-model="blockZ" type="number" /></label>
      </div>
      <div>
        <label>Type ID: <input v-model="blockType" type="text" /></label>
      </div>
      <button @click="sendBlock">发送自定义方块</button>
    </div>

    <p v-if="message">服务器返回: {{ message }}</p>
    <div v-if="worldData">
      <h3>世界数据:</h3>
      <pre>{{ worldData }}</pre>
    </div>
  </div>
</template>

<style scoped>
.custom-block-sender {
  margin-top: 20px;
  padding: 15px;
  border: 1px solid #ccc;
  border-radius: 5px;
}

.custom-block-sender div {
  margin-bottom: 10px;
}

.custom-block-sender label {
  margin-right: 10px;
}
</style>
