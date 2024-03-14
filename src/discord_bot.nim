{.define: ssl.}

import os, httpclient, asyncdispatch
import times, options, strutils, options
import dimscord, dimscmd

let discord = newDiscordClient(getEnv("BOT_TOKEN"))
var cmd = discord.newHandler()

proc reply(msg: Message, text: string): Future[Message] {.async.} =
    result = await discord.api.sendMessage(msg.channelId, text)

proc reply(i: Interaction, text: string) {.async.} =
    let response = InteractionResponse(
        kind: irtChannelMessageWithSource,
        data: some InteractionApplicationCommandCallbackData(content: text)
    )
    await discord.api.createInteractionResponse(i.id, i.token, response)

cmd.addChat("hi") do ():
    discard await msg.reply("hello")

cmd.addChat("notacat") do ():
    ## asd
    let client = newHttpClient()
    try:
        let res = client.get("https://cataas.com/cat")
        echo "Fetching a cat: ", res.status, ", ", res.headers["content-type"].string
        echo "cat." & res.headers["content-type"].split("/")[1]

        ## Invalid JSON (50109)
        #[let attach = Attachment(
                filename: "cat." & res.headers["content-type"].split("/")[1],
                content_type:  some res.headers["content-type"].string,
                file: res.body,
                )
        echo attach

        let response = InteractionResponse(
            kind: irtChannelMessageWithSource,
            data: some InteractionApplicationCommandCallbackData(content: "asd", attachments: @[ attach ])
        )
        await discord.api.createInteractionResponse(i.id, i.token, response)]#

        discard await discord.api.sendMessage(msg.channelId, files = @[DiscordFile(
            name: "cat." & res.headers["content-type"].split("/")[1],
            body: res.body
        )])

    except:
        var error = getCurrentException()
        echo "Exception occurred: ", error.msg


    #[discard await discord.api.sendMessage(msg.channelId, "smh",
        files = @[DiscordFile(body: res.body)]
    )]#

# Handle event for on_ready.
proc onReady(s: Shard, r: Ready) {.event(discord).} =
    await cmd.registerCommands()
    echo "Ready as " & $r.user

proc interactionCreate(s: Shard, i: Interaction) {.event(discord).} =
    discard await cmd.handleInteraction(s, i)

# Handle event for message_create.
proc messageCreate(s: Shard, msg: Message) {.event(discord).} =
    if msg.author.bot: return

    discard await cmd.handleMessage("./", s, msg)

# Connect to Discord and run the bot.
waitFor discord.startSession()
