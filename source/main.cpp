
#include <fstream>
#include <iostream>
#include <regex>

#ifdef ASIO_STANDALONE
#include <asio.hpp>
#else
#include <boost/asio.hpp>
namespace asio = boost::asio;
#endif

#include "bot.hpp"

using json = nlohmann::json;

#include <cpr/cpr.h>
#include "utils.hpp"
#include "commands.hpp"


int main() {
    dpp::log::filter = dpp::log::debug;
    dpp::log::out = &std::cerr;

    std::cout << "Starting bot ..." << std::endl;

    std::string token = getToken();
    if (token.empty()) {
        std::cerr << "Failed to read token from environment. Exiting ..." << std::endl;
        exit(1);
    }

    dpp::User self;
    auto bot = std::make_shared<Bot>();

    bot->debugUnhandled = true;
    bot->intents = dpp::intents::NONE | dpp::intents::GUILD_MESSAGES;

    bot->handlers.insert({
        "READY",
        [&self](dpp::ReadyEvent ready){ self = *ready.user; }
    });

    bot->prefix = "/";

    bot->respond("test", help(bot));
    bot->respond("help", "Mention me and I'll echo your message back!");

    bot->respond("about", [&bot](dpp::MessageCreateEvent msg) {
        std::ostringstream content;
        content << "Sure thing, "
                << *(msg.member->nick ? msg.author->username : msg.member->nick)
                << "!\n"
                << "I'm a simple bot meant to demonstrate the "
                   "Discord++ library.\n"
                << "You can learn more about Discord++ at "
                   "https://discord.gg/VHAyrvspCx";
        bot->createMessage()
            ->channel_id(*msg.channel_id)
            ->content(content.str())
            ->run();
    });

    bot->respond("lookatthis", [&bot](dpp::MessageCreateEvent msg) {
        std::ifstream ifs("image.jpg", std::ios::binary);
        if (!ifs) {
            std::cerr << "Couldn't load file 'image.jpg'!\n";
            return;
        }
        ifs.seekg(0, std::ios::end);
        std::ifstream::pos_type fileSize = ifs.tellg();
        ifs.seekg(0, std::ios::beg);
        auto file = std::make_shared<std::string>(fileSize, '\0');
        ifs.read(file->data(), fileSize);

        bot->createMessage()
            ->channel_id(*msg.channel_id)
            ->content("Look at this photograph")
            ->filename("image.jpg")
            ->filetype("image/jpg")
            ->file(file)
            ->run();
    });

    bot->respond("notacat", [&bot](dpp::MessageCreateEvent msg)
    {
        cpr::Response res = cpr::Get(cpr::Url{"https://cataas.com/cat"});
        std::cout << "Fetching a cat: " << res.status_code << ", " << res.header["content-type"] << std::endl;
        bot->createMessage()
            ->channel_id(*msg.channel_id)
            ->filename("cat.jpg")
            ->filetype(res.header["content-type"])
            ->file(res.text)
            ->run();
    });

    bot->respond("channelinfo", [&bot](dpp::MessageCreateEvent msg) {
        bot->getChannel()
            ->channel_id(*msg.channel_id)
            ->onRead([&bot, msg](bool error, json res) {
                bot->createMessage()
                    ->channel_id(*msg.channel_id)
                    ->content("```json\n" + res["body"].dump(4) + "\n```")
                    ->run();
            })
            ->run();
    });

    bot->respond("register", [&bot, &self](dpp::MessageCreateEvent msg) {
        if (*msg.author->id == 272712928074006528) {
            bot->createGuildApplicationCommand()
                ->application_id(*self.id)
                ->guild_id(*msg.guild_id)
                ->name("echo")
                ->description("Echoes what you say")
                ->options({dpp::ApplicationCommandOption(
                    dpp::ApplicationCommandOptionType::STRING,
                    std::string("message"), dpp::omitted, std::string("The message to echo"),
                    dpp::omitted, true)})
                ->command_type(dpp::ApplicationCommandType::CHAT_INPUT)
                ->onRead([](bool error, json res) {
                    std::cout << res.dump(4) << std::endl;
                })
                ->run();
        }

            bot->createGuildApplicationCommand()
                ->application_id(*self.id)
                ->guild_id(*msg.guild_id)
                ->name("lookatthis")
                ->description("Help information")
                ->options({dpp::ApplicationCommandOption(
                    dpp::ApplicationCommandOptionType::STRING,
                    std::string("message"), dpp::omitted, std::string("The message to echo"),
                    dpp::omitted, true)})
                ->command_type(dpp::ApplicationCommandType::CHAT_INPUT)
                ->onRead([](bool error, json res) {
                    std::cout << res.dump(4) << std::endl;
                })
                ->run();

    });

    bot->interactionHandlers.insert(
        {1102290596896460850, [&bot](dpp::Interaction msg) {
             bot->createResponse()
                 ->interaction_id(*msg.id)
                 ->interaction_token(*msg.token)
                 ->interaction_type(
                     dpp::InteractionCallbackType::CHANNEL_MESSAGE_WITH_SOURCE)
                 ->data({{
                     "content",
                     *std::get<dpp::ApplicationCommandData>(*msg.data).options->at(0).value
                 }})
                 ->run();
         }});

    // Create handler for the MESSAGE_CREATE payload, this receives all messages
    // sent that the bot can see.
    bot->handlers.insert(
        {"MESSAGE_CREATE", [&bot, &self](const dpp::MessageCreateEvent msg) {
             // Ignore messages from other bots
             if (msg.webhook_id || (msg.author->bot && *msg.author->bot)) {
                 return;
             }

             // Scan through mentions in the message for self
             bool mentioned = false;
             for (const dpp::User &mention : *msg.mentions) {
                 mentioned = mentioned || (*mention.id == *self.id);
             }
             if (mentioned) {
                 // Identify and remove mentions of self from the message
                 std::stringstream content;
                 content << "чё тебе надо, ";//*msg.content;
                 /*unsigned int oldlength, length = content.length();
                 do {
                     oldlength = length;
                     content = std::regex_replace(
                         content,
                         std::regex(R"(<@!?)" + std::to_string(*self.id) +
                                    R"(> ?)"),
                         "");
                     length = content.length();
                 } while (oldlength > length);
*/
                 // Get the target user's display name
                 std::string name = *(msg.member->nick ? msg.member->nick
                                                       : msg.author->username);
                 content << name << "?";
                 //std::cout << "Echoing " << name << '\n';

                 // Echo the created message
                 bot->createMessage()
                     ->channel_id(*msg.channel_id)
                     ->content(content.str())
                     ->run();

                 // Set status to Playing "with [author]"
                 /*bot->send(3,
                           {{"game", {{"name", "with " + name}, {"type", 0}}},
                            {"status", "online"},
                            {"afk", false},
                            {"since", "null"}});*/
             }
         }});


    auto asioCtx = std::make_shared<asio::io_context>();

    bot->initBot(9, token, asioCtx);
    bot->run();

    return 0;
}


