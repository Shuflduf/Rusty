import google.generativeai as genai

genai.configure(api_key="AIzaSyCW8kjw3V-YdNu1AX_qewTgFOHiOoQOywk")
model = genai.GenerativeModel("gemini-1.5-flash")

chats = {}

def generate(prompt):
    response = model.generate_content(prompt)
    return response.text[:2000]

def continue_chat(prompt, channel_id):
    if not channel_id in chats.keys():
        chats.update({channel_id: model.start_chat()})

    return chats[channel_id].send_message(prompt).text[:2000]
