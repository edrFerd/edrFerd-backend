<script setup>
import {ref, computed, onMounted, onUnmounted} from "vue";

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
    block: {
      point: {
        x: parseInt(blockX.value),
        y: parseInt(blockY.value),
        z: parseInt(blockZ.value),
      },
      block_appearance: {
        type_id: blockType.value,
      },
    },
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

// WebSocket 相关状态
const wsConnected = ref(false)
const wsMessages = ref([])
const wsMessage = ref('')
const ws = ref(null)

// WebSocket 连接函数
function connectWebSocket() {
  try {
    ws.value = new WebSocket('ws://127.0.0.1:1416/ws')
    
    ws.value.onopen = () => {
      wsConnected.value = true
      addWsMessage('系统', '已连接到 WebSocket 服务器')
      console.log('WebSocket 连接已建立')
    }
    
    ws.value.onmessage = (event) => {
      addWsMessage('服务器', event.data)
      console.log('收到消息:', event.data)
    }
    
    ws.value.onclose = () => {
      wsConnected.value = false
      addWsMessage('系统', 'WebSocket 连接已断开')
      console.log('WebSocket 连接已断开')
    }
    
    ws.value.onerror = (error) => {
      addWsMessage('系统', `WebSocket 错误: ${error}`)
      console.error('WebSocket 错误:', error)
    }
  } catch (error) {
    addWsMessage('系统', `连接失败: ${error.message}`)
    console.error('WebSocket 连接失败:', error)
  }
}

// 断开 WebSocket 连接
function disconnectWebSocket() {
  if (ws.value) {
    ws.value.close()
    ws.value = null
  }
}

// 发送 WebSocket 消息
function sendWebSocketMessage() {
  if (ws.value && wsConnected.value && wsMessage.value.trim()) {
    ws.value.send(wsMessage.value)
    addWsMessage('客户端', wsMessage.value)
    wsMessage.value = ''
  }
}

// 添加消息到消息列表
function addWsMessage(sender, content) {
  wsMessages.value.push({
    sender,
    content,
    timestamp: new Date().toLocaleTimeString()
  })
  // 保持最多 50 条消息
  if (wsMessages.value.length > 50) {
    wsMessages.value.shift()
  }
}

// 清空消息列表
function clearMessages() {
  wsMessages.value = []
}

// 组件挂载时自动连接 WebSocket
onMounted(() => {
  connectWebSocket()
})

// 组件卸载时断开连接
onUnmounted(() => {
  disconnectWebSocket()
})
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

    <div class="action-section">
      <h2>WebSocket 连接</h2>
      <div class="ws-status">
        <p>连接状态: 
          <span :class="wsConnected ? 'status-connected' : 'status-disconnected'">
            {{ wsConnected ? '已连接' : '未连接' }}
          </span>
        </p>
        <div class="ws-controls">
          <button @click="connectWebSocket" :disabled="wsConnected">连接</button>
          <button @click="disconnectWebSocket" :disabled="!wsConnected">断开</button>
        </div>
      </div>
      
      <div v-if="wsConnected" class="ws-input-section">
        <input 
          v-model="wsMessage" 
          type="text" 
          placeholder="输入消息内容" 
          @keyup.enter="sendWebSocketMessage"
        />
        <button @click="sendWebSocketMessage" :disabled="!wsMessage.trim()">发送消息</button>
      </div>
      
      <div class="ws-message-section">
        <div class="ws-message-header">
          <h3>消息记录</h3>
          <button @click="clearMessages" class="clear-btn">清空</button>
        </div>
        <div class="ws-message-list" v-if="wsMessages.length > 0">
          <div v-for="msg in wsMessages" :key="msg.timestamp" class="ws-message">
            <span class="message-time">{{ msg.timestamp }}</span>
            <span :class="'message-sender-' + msg.sender.toLowerCase()">{{ msg.sender }}:</span>
            <span class="message-content">{{ msg.content }}</span>
          </div>
        </div>
        <div v-else class="no-messages">
          <p>暂无消息</p>
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

.ws-message-list {
  margin-top: 15px;
  padding: 15px;
  border: 1px solid #3a3a3a;
  border-radius: 6px;
  background-color: #2a2a2a;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

.ws-message {
  margin-bottom: 10px;
  color: #e0e0e0;
}

.ws-message strong {
  color: #b0b0b0;
}

.ws-status {
  margin-bottom: 15px;
}

.ws-controls {
  margin-top: 10px;
}

.ws-input-section {
  margin-top: 15px;
}

.ws-message-section {
  margin-top: 15px;
}

.ws-message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.ws-message-header h3 {
  color: #f0f0f0;
}

.clear-btn {
  background-color: #dc3545;
  color: white;
  border: none;
  padding: 5px 10px;
  border-radius: 5px;
  cursor: pointer;
}

.clear-btn:hover {
  background-color: #a82333;
  border-color: #dc3545;
}

.no-messages {
  padding: 15px;
  text-align: center;
  color: #999;
}

.status-connected {
  color: #2ecc71;
}

.status-disconnected {
  color: #e74c3c;
}

.message-time {
  font-size: 12px;
  color: #999;
  margin-right: 5px;
}

.message-sender-client {
  color: #4da3ff;
  font-weight: 500;
}

.message-sender-server {
  color: #ff6b7a;
  font-weight: 500;
}

.message-sender-系统 {
  color: #ffc107;
  font-weight: 500;
}

.message-content {
  font-size: 14px;
  color: #e0e0e0;
  margin-left: 5px;
}
</style>
