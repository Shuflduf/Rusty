import discord
import math
import ai
import log
import os
from log import log_text

shufl_id = 442706613107687434

intents = discord.Intents.default()
intents.message_content = True
client = discord.Client(intents=intents)

conversation_mode = {}
last_channel_id = 0

log.init()

async def send_long_message(text, channel):
    for i in range(int(math.ceil(len(text) / 2000))):
        await channel.send(text[i * 2000: (i + 1) * 2000])

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
            f"[Now in {message.guild.name}: #{message.channel.name}]".center(49, '=')
        )
        if not message.channel.id in conversation_mode.keys():
            conversation_mode.update({message.channel.id: False})

    if message.content.startswith("$log"):
        print(message.author.id)
        if str(message.author.id) == shufl_id:
            await send_long_message(log.return_log(), message.channel)
        else:
            await message.channel.send(f"You are not Shuflduf!")

    elif message.content.startswith('$hello'):
        await message.channel.send('Hello!')
        log_text(f"Hello, {message.author.name}!")

    elif message.content.startswith('$longhello'):
        await send_long_message('Hello! ' * 300, message.channel)
        log_text(f"BIG Hello, {message.author.name}!")

    elif message.content.startswith("$convo"):
        conversation_mode[message.channel.id] = not conversation_mode[message.channel.id]
        local_conversation_enabled = conversation_mode[message.channel.id]
        await message.channel.send(f"Conversation mode is now set to: {local_conversation_enabled}")
        log_text(f"Conversation mode is now set to: {local_conversation_enabled}")
 
    elif conversation_mode[message.channel.id]:
        log_text(f"{message.author.name}: {message.content}")
        response = ai.continue_chat(message.content, message.channel.id)
        await send_long_message(response, message.channel)
        log_text(f"{client.user.name}: {response}")
    
    elif message.content.startswith("$"):
        log_text(f"$ {message.author.name}: {message.content[1:]}")
        response = ai.generate(message.content[1:])
        await send_long_message(response, message.channel)
        log_text(f"$ {client.user.name}: {response}")

client.run(os.environ["RUSTY_TOKEN"])

