cmake_minimum_required(VERSION 3.16)

project(discord-oscuro LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -pedantic")

include(cmake/macros.cmake)
include(cmake/CPM.cmake)

CPMAddPackage("gh:DiscordPP/discordpp#daf04ea")
CPMAddPackage("gh:DiscordPP/rest-simpleweb#4cf911b")
CPMAddPackage("gh:DiscordPP/websocket-simpleweb#9082991")
CPMAddPackage("gh:DiscordPP/plugin-native#238b7e8")
CPMAddPackage("gh:DiscordPP/plugin-overload#8862ac5")
CPMAddPackage("gh:DiscordPP/plugin-responder#3c2c485")
CPMAddPackage("gh:DiscordPP/plugin-interactionhandler#df07fd3")
CPMAddPackage("gh:DiscordPP/plugin-ratelimit#2c8cb34")
CPMAddPackage("gh:libcpr/cpr#1.10.4")

set(DISCORDPP_USE_BOOST OFF CACHE BOOL "Override option" FORCE)

CREATE_DISCORDPP_DEFINITIONS()
CREATE_DISCORDPP_INCLUDE()


set(THREADS_PREFER_PTHREAD_FLAG ON)

if(${DISCORDPP_USE_BOOST})
    find_package(Boost 1.71.0 REQUIRED system date_time)
endif()

find_package(Threads REQUIRED)
find_package(OpenSSL REQUIRED)


add_executable(${PROJECT_NAME}
    source/main.cpp
)

#target_precompile_headers(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/source/main.hpp)

target_link_libraries(${PROJECT_NAME}
    ${Boost_LIBRARIES}
    Threads::Threads
    ${OPENSSL_LIBRARIES}
    discordpp
    discordpp-rest-simpleweb
    discordpp-websocket-simpleweb
    discordpp-plugin-native
    discordpp-plugin-overload
    discordpp-plugin-responder
    discordpp-plugin-interactionhandler
    discordpp-plugin-ratelimit
    cpr::cpr
)

copy_file("env")

