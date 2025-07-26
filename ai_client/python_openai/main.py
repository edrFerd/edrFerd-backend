import openai
import os
import json
import time

# --- OpenAI Client Setup ---
client = openai.OpenAI(
    api_key="sk-iR4aQLl0390FOC404yXPBF54brpbbiQyIW7RY9QVWmmGUWMi",
    base_url="https://api.moonshot.cn/v1",
)

# --- Tool Definitions ---
def get_current_weather(location, unit="celsius"):
    """Get the current weather in a given location."""
    print(f"Getting weather for {location} in {unit}...")
    if "tokyo" in location.lower():
        return json.dumps({"location": "Tokyo", "temperature": "10", "unit": "celsius"})
    elif "san francisco" in location.lower():
        return json.dumps({"location": "San Francisco", "temperature": "72", "unit": "fahrenheit"})
    else:
        return json.dumps({"location": location, "temperature": "22", "unit": "celsius"})

def get_stock_price(symbol: str):
    """Get the current stock price for a given symbol."""
    print(f"Getting stock price for {symbol}...")
    if symbol.lower() == "msft":
        return json.dumps({"symbol": "MSFT", "price": "300.00"})
    elif symbol.lower() == "aapl":
        return json.dumps({"symbol": "AAPL", "price": "150.00"})
    else:
        return json.dumps({"symbol": symbol, "price": "unknown"})


# --- Main Loop ---
def run_conversation():
    messages = [{"role": "user", "content": "What's the weather like in San Francisco and Tokyo? Also, what's the stock price of MSFT?"}]
    tools = [
        {
            "type": "function",
            "function": {
                "name": "get_current_weather",
                "description": "Get the current weather in a given location",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA",
                        },
                        "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
                    },
                    "required": ["location"],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "get_stock_price",
                "description": "Get the current stock price for a given symbol",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "symbol": {
                            "type": "string",
                            "description": "The stock ticker symbol, e.g. MSFT for Microsoft",
                        }
                    },
                    "required": ["symbol"],
                },
            },
        }
    ]

    while True:
        print("\n--- New Conversation Iteration ---")
        response = client.chat.completions.create(
            model="moonshot-v1-8k",
            messages=messages,
            tools=tools,
            tool_choice="auto",
        )
        response_message = response.choices[0].message
        tool_calls = response_message.tool_calls

        if tool_calls:
            print("Tool calls detected:", tool_calls)
            available_functions = {
                "get_current_weather": get_current_weather,
                "get_stock_price": get_stock_price,
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
                model="moonshot-v1-8k",
                messages=messages,
            )
            print("Final response:", second_response.choices[0].message.content)
        else:
            print("No tool calls. Final response:", response_message.content)
            break # Exit loop if no tool calls are made

        print("Waiting for 10 seconds before next loop...")
        time.sleep(10)
        # Reset messages for the next independent run, or modify logic as needed
        messages = [messages[0]]

if __name__ == "__main__":
    run_conversation()
