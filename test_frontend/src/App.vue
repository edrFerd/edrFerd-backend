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
    worldData.value = data
    console.log(data)
  } catch (error) {
    message.value = `Error: ${error.message}`
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
    <h1>EdrFerd 测试面板</h1>

    <div class="action-section">
      <h2>通用操作</h2>
      <button @click="callTestSend">发送一个默认共识包</button>
    </div>

    <div class="action-section">
      <h2>发送自定义方块</h2>
      <div class="form-group">
        <label>坐标 X: <input v-model="blockX" type="number" /></label>
        <label>坐标 Y: <input v-model="blockY" type="number" /></label>
        <label>坐标 Z: <input v-model="blockZ" type="number" /></label>
      </div>
      <div class="form-group">
        <label>方块类型 ID: <input v-model="blockType" type="text" /></label>
      </div>
      <button @click="sendBlock">发送自定义方块</button>
    </div>

    <div class="action-section">
      <h2>世界状态</h2>
      <button @click="showWorld">显示/刷新世界</button>
      <div v-if="worldData" class="world-display">
        <div v-for="(info, point) in worldData" :key="point" class="block-card">
          <p><strong>坐标:</strong> {{ point }}</p>
          <p><strong>类型:</strong> {{ info.type_id }}</p>
        </div>
      </div>
    </div>

    <p v-if="message" class="server-message">服务器返回: {{ message }}</p>

  </div>
</template>

<style scoped>
:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
}

h1 {
  text-align: center;
  color: #2c3e50;
}

.action-section {
  margin-top: 20px;
  padding: 20px;
  border: 1px solid #dfe4ea;
  border-radius: 8px;
  background-color: #f8f9fa;
}

.action-section h2 {
  margin-top: 0;
  border-bottom: 2px solid #ced4da;
  padding-bottom: 10px;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  margin-right: 15px;
  font-weight: 500;
}

input {
  padding: 8px;
  border: 1px solid #ced4da;
  border-radius: 4px;
}

button {
  padding: 10px 15px;
  border: none;
  border-radius: 5px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #0056b3;
}

.server-message {
  margin-top: 20px;
  padding: 10px;
  background-color: #e9ecef;
  border-left: 5px solid #007bff;
}

.world-display {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  margin-top: 15px;
}

.block-card {
  background-color: white;
  padding: 15px;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  flex-basis: 200px; /* Set a base width for each card */
  flex-grow: 1;
}

.block-card p {
  margin: 5px 0;
}

.block-card strong {
  color: #495057;
}
</style>
