import os
import google.generativeai as genai

genai.configure(api_key=os.environ["GEMINI_API_KEY"])
model = genai.GenerativeModel("gemini-1.5-flash")

chats = {}

def generate(prompt):
    response = model.generate_content(prompt)
    return response.text

def continue_chat(prompt, channel_id):
    if not channel_id in chats.keys():
        chats.update({channel_id: model.start_chat()})

    return chats[channel_id].send_message(prompt).text
