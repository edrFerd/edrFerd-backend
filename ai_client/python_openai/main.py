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



# --- 工具定义 ---


def get_world_state():
    """获取当前游戏世界的状态。返回所有方块的列表。
    每个方块都是一个字典，包含 'point'（x, y, z 坐标）和 'info'（block_id 等信息）。
    """
    url = f"{FRONTEND_SERVER_URL}/known_world_state"
    print(f"正在获取世界状态... URL: {url}")
    try:
        response = requests.get(url)
        print(f"响应状态码: {response.status_code}")
        response.raise_for_status()
        data = response.json()
        for item in data:
            item["pub_key"] = None
        print(f"获取到世界数据: {len(data) if isinstance(data, list) else '非列表类型'} 个元素")
        print(f"响应数据的前 200 个字符: {str(data)[:200]}...")
        return json.dumps(data)
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")
        return json.dumps({"error": str(e)})


def set_block(x: int, y: int, z: int, block_id: str, duration: int = 50):
    """在给定的坐标 (x, y, z) 放置一个具有特定 block_id 的方块。
    有效的 block_id 值包括：RED, WHITE, PURPLE, YELLOW, PINK, ORANGE, BLUE, BROWN, CYAN, LIME, MAGENTA, GRAY, LIGHT_GRAY, LIGHT_BLUE, GREEN, BLACK, air。
    duration 参数控制放置方块的持续时间（不能超过50）。
    """
    # 限制 duration 不能超过 50
    if duration > 50:
        duration = 50
        print(f"警告：duration 被限制为 50")


    url = f"{FRONTEND_SERVER_URL}/set_block_once"
    payload = {
        "duration": duration,
        "x": x,
        "y": y,
        "z": z,
        "info": {"type_id": block_id}
    }
    print(f"正在坐标 ({x}, {y}, {z}) 放置方块 {block_id}...")
    print(f"请求 URL: {url}")
    print(f"请求载荷: {json.dumps(payload, indent=2)}")
    try:
        response = requests.post(url, json=payload)
        print(f"响应状态码: {response.status_code}")
        print(f"响应内容: {response.text}")
        response.raise_for_status()
        return json.dumps({"status": "OK", "response": response.text})
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")
        return json.dumps({"error": str(e)})


def remove_block(x: int, y: int, z: int):
    """移除给定坐标 (x, y, z) 的方块。"""
    url = f"{FRONTEND_SERVER_URL}/remove_block"
    payload = {"x": x, "y": y, "z": z}
    print(f"正在移除坐标 ({x}, {y}, {z}) 的方块...")
    print(f"请求 URL: {url}")
    print(f"请求载荷: {json.dumps(payload, indent=2)}")
    try:
        response = requests.post(url, json=payload)
        print(f"响应状态码: {response.status_code}")
        print(f"响应内容: {response.text}")
        response.raise_for_status()
        return json.dumps({"status": "OK", "response": response.text})
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")
        return json.dumps({"error": str(e)})


# --- 主循环 ---
def run_conversation():
    initial_prompt = (
        "你是一个在3D方块世界中控制角色的AI代理。"
        "你的目标是自由修改世界，做出一些有意义的建筑"
        "你可以查看世界状态、放置方块和移除方块。"
        "如果你想移除方块, 你只需要在对应位置设置 air 方块即可"
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
                "description": "在给定的坐标 (x, y, z) 放置一个具有特定 block_id 的方块。可以指定 duration（持续时间，不能超过50）。",
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
        }
    ]

    while True:
        print("\n--- 新一轮对话 ---")
        response = client.chat.completions.create(
            model="kimi-k2-0711-preview",
            messages=messages,
            tools=tools,
            tool_choice="auto",
            temperature=1.0
        )
        response_message = response.choices[0].message
        tool_calls = response_message.tool_calls

        if tool_calls:
            print("检测到工具调用:", tool_calls)
            available_functions = {
                "get_world_state": get_world_state,
                "set_block": set_block,
                "remove_block": remove_block,

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
