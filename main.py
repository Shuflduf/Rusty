import discord
import ai
import log
from log import log_text

intents = discord.Intents.default()
intents.message_content = True
client = discord.Client(intents=intents)

conversation_mode = False
last_channel_id = 0

log.init()

async def on_ready():
    log_text(f'We have logged in as {client.user}')
    
@client.event
async def on_message(message):
    global conversation_mode
    global last_channel_id
    if message.author == client.user:
        return
    
    if message.channel.id != last_channel_id:
        last_channel_id = message.channel.id 
        log_text(
            f"[Now in {message.guild.name}: #{message.channel.name}]".center(40, '=')
        )

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')
        log_text(f"Hello, {message.author.name}!")

    elif message.content.startswith("$convo"):
        conversation_mode = not conversation_mode
        await message.channel.send(f"Conversation mode is now set to: {conversation_mode}")
        log_text(f"Conversation mode is now set to: {conversation_mode}")
 
    elif conversation_mode:
        log_text(f"{message.author.name}: {message.content}")
        response = ai.continue_chat(message.content, message.channel.id)
        await message.channel.send(response)
        log_text(f"{client.user.name}: {response}")
    
    elif message.content.startswith("$"):
        log_text(f"$ {message.author.name}: {message.content[1:]}")
        response = ai.generate(message.content[1:])
        await message.channel.send(response)
        log_text(f"$ {client.user.name}: {response}")

client.run('MTI3MTMwMzQyMjA0OTY1Mjc3Ng.GlYDdI.7FrHPFaveCUWW01UaABK8Jx1pJrXu91C73JkBQ')

