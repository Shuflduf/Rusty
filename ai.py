import google.generativeai as genai

genai.configure(api_key="AIzaSyCW8kjw3V-YdNu1AX_qewTgFOHiOoQOywk")
model = genai.GenerativeModel("gemini-1.5-flash")

def generate(prompt):
    response = model.generate_content(prompt)
    return response.text[:2000]

