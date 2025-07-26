<script setup>
import {ref, computed} from "vue";

const message = ref('')
const worldData = ref(null)
const blockX = ref(0)
const blockY = ref(0)
const blockZ = ref(0)
const blockType = ref('default_block')
const blockTime = ref(1000)

const sortedWorldData = computed(() => {
  if (!worldData.value) {
    return [];
  }
  return Object.entries(worldData.value)
    .map(([point, info]) => {
      const coords = point.slice(1, -1).split(',').map(Number);
      return { point, info, coords };
    })
    .sort((a, b) => {
      if (a.coords[0] !== b.coords[0]) return a.coords[0] - b.coords[0];
      if (a.coords[1] !== b.coords[1]) return a.coords[1] - b.coords[1];
      return a.coords[2] - b.coords[2];
    });
});

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

async function sendBlockWithTime() {
  const data = {
    block: [parseInt(blockX.value), parseInt(blockY.value), parseInt(blockZ.value)],
    cost: parseInt(blockTime.value)
  };

  try {
    const response = await fetch('http://127.0.0.1:1415/send_block_with_time', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });
    const result = await response.text();
    message.value = result;
    console.log(result);
  } catch (error) {
    message.value = `Error: ${error.message}`;
    console.error('Error calling send_block_with_time:', error);
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
      <div class="form-group">
        <label>时间 (毫秒): <input v-model="blockTime" type="number" /></label>
      </div>
      <button @click="sendBlock">发送自定义方块</button>
      <button @click="sendBlockWithTime">发送指定耗时pow的方块</button>
    </div>

    <div class="action-section">
      <h2>世界状态</h2>
      <button @click="showWorld">显示/刷新世界</button>
      <div v-if="sortedWorldData.length > 0" class="world-display">
        <div v-for="block in sortedWorldData" :key="block.point" class="block-card">
          <p><strong>坐标:</strong> {{ block.point }}</p>
          <p><strong>类型:</strong> {{ block.info.type_id }}</p>
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

body {
  background-color: #121212;
  color: #e0e0e0;
}

h1 {
  text-align: center;
  color: #e0e0e0;
}

.action-section {
  margin-top: 20px;
  padding: 20px;
  border: 1px solid #3a3a3a;
  border-radius: 8px;
  background-color: #1e1e1e;
}

.action-section h2 {
  margin-top: 0;
  border-bottom: 2px solid #4a4a4a;
  padding-bottom: 10px;
  color: #f0f0f0;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  margin-right: 15px;
  font-weight: 500;
  color: #d0d0d0;
}

input {
  padding: 8px;
  border: 1px solid #4a4a4a;
  border-radius: 4px;
  background-color: #2a2a2a;
  color: #e0e0e0;
}

button {
  padding: 10px 15px;
  border: 1px solid #4a4a4a;
  border-radius: 5px;
  background-color: #2a2a2a;
  color: #e0e0e0;
  cursor: pointer;
  transition: all 0.3s;
}

button:hover {
  background-color: #3a3a3a;
  border-color: #007bff;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.server-message {
  margin-top: 20px;
  padding: 10px;
  background-color: #2a2a2a;
  border-left: 5px solid #007bff;
  color: #e0e0e0;
}

.world-display {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  margin-top: 15px;
}

.block-card {
  background-color: #2a2a2a;
  padding: 15px;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid #3a3a3a;
  flex-basis: 200px; /* Set a base width for each card */
  flex-grow: 1;
}

.block-card p {
  margin: 5px 0;
  color: #e0e0e0;
}

.block-card strong {
  color: #b0b0b0;
}
</style>
