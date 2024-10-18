import discord
import ai

intents = discord.Intents.default()
intents.message_content = True

client = discord.Client(intents=intents)

conversation_mode = False

async def on_ready():
    print(f'We have logged in as {client.user}')

@client.event
async def on_message(message):
    global conversation_mode
    if message.author == client.user:
        print(f"Sent Message: {message.content}")
        return

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')
        print(f"Hello, {message.author.name}!")

    elif message.content.startswith("$convo"):
        conversation_mode = not conversation_mode
        await message.channel.send(f"Conversation mode is now set to: {conversation_mode}")
        print(f"Conversation mode is now set to: {conversation_mode}")
 
    elif conversation_mode:
        response = ai.generate(message.content)
        await message.channel.send(response)
        print(f"{message.author.name}: {message.content}")
        print(f"{client.user.name}: {response}")
    
    elif message.content.startswith("$"):
        await message.channel.send(ai.generate(message.content[1:]))
        print(f"Ai response to: {message.content[1:]}")

client.run('MTI3MTMwMzQyMjA0OTY1Mjc3Ng.GlYDdI.7FrHPFaveCUWW01UaABK8Jx1pJrXu91C73JkBQ')

