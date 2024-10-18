import discord
import ai
import log
from log import log_text

intents = discord.Intents.default()
intents.message_content = True
client = discord.Client(intents=intents)

conversation_mode = False

log.init()

async def on_ready():
    log_text(f'We have logged in as {client.user}')
    
@client.event
async def on_message(message):
    global conversation_mode
    if message.author == client.user:
        return

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')
        log_text(f"Hello, {message.author.name}!")

    elif message.content.startswith("$convo"):
        conversation_mode = not conversation_mode
        await message.channel.send(f"Conversation mode is now set to: {conversation_mode}")
        log_text(f"Conversation mode is now set to: {conversation_mode}")
 
    elif conversation_mode:
        response = ai.generate(message.content)
        await message.channel.send(response)
        log_text(f"{message.author.name}: {message.content}")
        log_text(f"{client.user.name}: {response}")
    
    elif message.content.startswith("$"):
        response = ai.generate(message.content[1:])
        await message.channel.send(response)
        log_text(f"$ {message.author.name}: {message.content[1:]}")
        log_text(f"$ {client.user.name}: {response}")

client.run('MTI3MTMwMzQyMjA0OTY1Mjc3Ng.GlYDdI.7FrHPFaveCUWW01UaABK8Jx1pJrXu91C73JkBQ')

