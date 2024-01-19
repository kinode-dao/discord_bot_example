# discord_bot_example

This repo implements a Discord bot as a process and handles interactions with Discord's Gateway API and HTTP API.

The `discord_api_runner` process should not be modified.

Feel free to copy this repo and make changes to the `my_bot` process. You will have to add your bot's information as specified below.

## Required Configuration.

1. Get your token (from the Bot page) and put it in a new file `my_bot/.bot_token`.
2. Get your application ID (from the General Information page) and put it in a new file `my_bot/.bot_application_id`.

You must have a Discord bot set up. See `Setting up a Discord Bot` for more info.

## Setting up a Discord Bot

First, follow Step 1 of the (Getting Started guide)[https://discord.com/developers/docs/getting-started].
With the URL generator, you can give your bot admin access by setting `permissions=8` and removing `scope=bot` from the OAuth2 URL.
You must then put the URL into a browser and add the bot to your server.
