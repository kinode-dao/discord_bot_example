# discord_bot_example

This repo implements a Discord bot as a process and handles interactions with Discord's Gateway API and HTTP API.

The `discord_api_runner` process should not be modified.

Feel free to copy this repo and make changes to the `my_bot` process. You will have to add your bot's information as specified below.

## Required Configuration.

1. Get your token (from the Bot page) and put it in a new file `my_bot/.bot_token`.
2. Get your application ID (from the General Information page) and put it in a new file `my_bot/.bot_application_id`.
3. (Optional), if you want to use the `/price` command, put your coinmarketcap API key in `my_bot/.coinmarketcap_api_key`.

Optionally, edit `metadata.json` and `manifest.json` and replace the `template` publisher with your own name.

You must have a Discord bot set up. See `Setting up a Discord Bot` for more info.

## Setting up a Discord Bot

First, follow Step 1 of the (Getting Started guide)[https://discord.com/developers/docs/getting-started].
At the end, you should have:
1. A bot token
2. An application ID
3. An OAuth2 URL in the format `https://discord.com/oauth2/authorize?client_id=*your_client_id*&permissions=8&scope=bot`. You should not have a redirect URL or code grant in here.

You can then open that URL in a browser and add the bot to a server of your choice.
You must have `Manage Server` permissions in order to add the bot.
You can create your own testing server easily using the `+` button in Discord (at the bottom of the server list).
