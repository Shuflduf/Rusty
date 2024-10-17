# This example requires the 'message_content' intent.

import discord
from discord import app_commands
import ai

intents = discord.Intents.default()
intents.message_content = True

client = discord.Client(intents=intents)
tree = app_commands.CommandTree(client)

@tree.command(
    name = "ai",
    description = "Uses Gemini AI"
)
async def AI_command(interaction, prompt: str = None):
    await interaction.response.send_message(ai.generate(prompt))

@client.event
async def on_ready():
    print(f'We have logged in as {client.user}')

@client.event
async def on_message(message):
    if message.author == client.user:
        return

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')

    elif message.content.startswith("$"):
        await message.channel.send(ai.generate(message.content[1:]))

client.run('MTI3MTMwMzQyMjA0OTY1Mjc3Ng.GlYDdI.7FrHPFaveCUWW01UaABK8Jx1pJrXu91C73JkBQ')

