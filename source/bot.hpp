#pragma once

#include <discordpp/bot.hh>
#include <discordpp/rest-simpleweb.hh>
#include <discordpp/websocket-simpleweb.hh>
#include <discordpp/plugin-native.hh>
#include <discordpp/plugin-overload.hh>
#include <discordpp/plugin-responder.hh>
#include <discordpp/plugin-interactionhandler.hh>
#include <discordpp/plugin-ratelimit.hh>


using Bot = discordpp::PluginRateLimit
    <discordpp::PluginInteractionHandler
    <discordpp::PluginResponder
    <discordpp::PluginOverload
    <discordpp::PluginNative
    <discordpp::WebsocketSimpleWeb
    <discordpp::RestSimpleWeb
        <discordpp::Bot>>>>>>>;

namespace dpp = discordpp;