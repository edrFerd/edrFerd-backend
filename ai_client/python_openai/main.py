import openai
import json
import time
import requests
from datetime import datetime

# --- OpenAI 客户端设置 ---
client = openai.OpenAI(
    api_key="sk-iR4aQLl0390FOC404yXPBF54brpbbiQyIW7RY9QVWmmGUWMi",  # 请替换为您的 API Key
    base_url="https://api.moonshot.cn/v1",
)

# --- 游戏服务器 API 配置 ---
FRONTEND_SERVER_URL = "http://localhost:1416"

# --- AI 状态 ---
declarations = {}

# --- 工具定义 ---


def get_world_state():
    """获取当前游戏世界的状态。返回所有方块的列表。
    每个方块都是一个字典，包含 'point'（x, y, z 坐标）和 'info'（block_id 等信息）。
    """
    print("正在获取世界状态...")
    try:
        response = requests.get(f"{FRONTEND_SERVER_URL}/known_world_state")
        response.raise_for_status()
        return json.dumps(response.json())
    except requests.exceptions.RequestException as e:
        return json.dumps({"error": str(e)})


def set_block(x: int, y: int, z: int, block_id: str):
    """在给定的坐标 (x, y, z) 放置一个具有特定 block_id 的方块。
    有效的 block_id 值包括：RED, WHITE, PURPLE, YELLOW, PINK, ORANGE, BLUE, BROWN, CYAN, LIME, MAGENTA, GRAY, LIGHT_GRAY, LIGHT_BLUE, GREEN, BLACK, air。
    """
    print(f"正在坐标 ({x}, {y}, {z}) 放置方块 {block_id}...")
    try:
        payload = {
            "duration": 100000,  # 根据 set_block_once 的逻辑，使用一个较长的持续时间
            "x": x,
            "y": y,
            "z": z,
            "info": {"block_id": block_id, "block_meta": {}},
        }
        response = requests.post(f"{FRONTEND_SERVER_URL}/set_block_once", json=payload)
        response.raise_for_status()
        return json.dumps({"status": "OK"})
    except requests.exceptions.RequestException as e:
        return json.dumps({"error": str(e)})


def remove_block(x: int, y: int, z: int):
    """移除给定坐标 (x, y, z) 的方块。"""
    print(f"正在移除坐标 ({x}, {y}, {z}) 的方块...")
    try:
        payload = {"x": x, "y": y, "z": z}
        response = requests.post(f"{FRONTEND_SERVER_URL}/remove_block", json=payload)
        response.raise_for_status()
        return json.dumps({"status": "OK"})
    except requests.exceptions.RequestException as e:
        return json.dumps({"error": str(e)})


def add_declaration(key: str, value: str):
    """在 AI 的记忆中添加或更新一个声明。
    用于 AI 记录自己的陈述或计划。
    """
    print(f"正在添加声明: {key} = {value}")
    declarations[key] = value
    return json.dumps({"status": "OK", "declarations": declarations})


def view_declarations():
    """查看 AI 做出的所有当前声明。"""
    print("正在查看声明...")
    return json.dumps(declarations)


# --- 主循环 ---
def run_conversation():
    initial_prompt = (
        "你是一个在3D方块世界中控制角色的AI代理。"
        "你的目标是自由修改世界，做出一些有意义的建筑"
        "你可以查看世界状态、放置方块和移除方块。"
        "你还有一个“声明”字典来记住你的计划。"
        "注意，每个声明的创建都需要一定的算力作为代价。所以请不要选择太大的duraction"
        "可用的方块类型有：RED, WHITE, PURPLE, YELLOW, PINK, ORANGE, BLUE, BROWN, CYAN, LIME, MAGENTA, GRAY, LIGHT_GRAY, LIGHT_BLUE, GREEN, BLACK, air。"
        "让我们先检查一下世界状态，然后你可以告诉我你的计划。"
    )
    messages = [{"role": "user", "content": initial_prompt}]
    tools = [
        {
            "type": "function",
            "function": {
                "name": "get_world_state",
                "description": "获取当前游戏世界的状态。",
                "parameters": {"type": "object", "properties": {}, "required": []},
            },
        },
        {
            "type": "function",
            "function": {
                "name": "set_block",
                "description": "在给定的坐标 (x, y, z) 放置一个具有特定 block_id 的方块。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "x": {"type": "integer"},
                        "y": {"type": "integer"},
                        "z": {"type": "integer"},
                        "block_id": {
                            "type": "string",
                            "enum": [
                                "RED",
                                "WHITE",
                                "PURPLE",
                                "YELLOW",
                                "PINK",
                                "ORANGE",
                                "BLUE",
                                "BROWN",
                                "CYAN",
                                "LIME",
                                "MAGENTA",
                                "GRAY",
                                "LIGHT_GRAY",
                                "LIGHT_BLUE",
                                "GREEN",
                                "BLACK",
                                "air",
                            ],
                        },
                    },
                    "required": ["x", "y", "z", "block_id"],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "remove_block",
                "description": "移除给定坐标 (x, y, z) 的方块。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "x": {"type": "integer"},
                        "y": {"type": "integer"},
                        "z": {"type": "integer"},
                    },
                    "required": ["x", "y", "z"],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "add_declaration",
                "description": "在 AI 的记忆中添加或更新一个声明。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "key": {"type": "string"},
                        "value": {"type": "string"},
                    },
                    "required": ["key", "value"],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "view_declarations",
                "description": "查看 AI 做出的所有当前声明。",
                "parameters": {"type": "object", "properties": {}, "required": []},
            },
        },
    ]

    while True:
        print("\n--- 新一轮对话 ---")
        response = client.chat.completions.create(
            model="kimi-k2-0711-preview",
            messages=messages,
            tools=tools,
            tool_choice="auto",
        )
        response_message = response.choices[0].message
        tool_calls = response_message.tool_calls

        if tool_calls:
            print("检测到工具调用:", tool_calls)
            available_functions = {
                "get_world_state": get_world_state,
                "set_block": set_block,
                "remove_block": remove_block,
                "add_declaration": add_declaration,
                "view_declarations": view_declarations,
            }
            messages.append(response_message)

            for tool_call in tool_calls:
                function_name = tool_call.function.name
                function_to_call = available_functions[function_name]
                function_args = json.loads(tool_call.function.arguments)
                function_response = function_to_call(**function_args)
                messages.append(
                    {
                        "tool_call_id": tool_call.id,
                        "role": "tool",
                        "name": function_name,
                        "content": function_response,
                    }
                )

            second_response = client.chat.completions.create(
                model="kimi-k2-0711-preview",  # 建议使用与之前相同的模型以保持一致性
                messages=messages,
            )
            final_message = second_response.choices[0].message
            print("最终回复:", final_message.content)
            messages.append(final_message)
        else:
            print("无工具调用。最终回复:", response_message.content)
            messages.append(response_message)
            messages.append({"role": "user", "content": "请自行决定，我不会给你任何建议"})


if __name__ == "__main__":
    run_conversation()
