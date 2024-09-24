
# This example requires the 'message_content' intent.

import discord

intents = discord.Intents.default()
intents.message_content = True

client = discord.Client(intents=intents)

token = "MTI3MTMwMzQyMjA0OTY1Mjc3Ng.G6FAPs.TXipKd9CQCejnJIKi8mNMETF5F9cA5FRG95PiM"

@client.event
async def on_ready():
    print(f'We have logged in as {client.user}')

@client.event
async def on_message(message):
    if message.author == client.user:
        return

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')

client.run(token)
