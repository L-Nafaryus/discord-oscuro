#pragma once

#include <functional>
#include <nlohmann/json.hpp>

#include "bot.hpp"

using json = nlohmann::json;

using event = std::function<void(const json)>;


auto help(const std::shared_ptr<Bot>& bot)
{
    return [&bot](const dpp::MessageCreateEvent& msg)
    {

    };
}