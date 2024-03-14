FROM alpine:latest AS build

RUN apk update
RUN apk add --no-cache \
    build-base \
    cmake \
    samurai \
    boost-dev \
    git \
    openssl-dev

WORKDIR /echo_bot
COPY . .
RUN cmake -B build -G Ninja .
RUN cmake --build build


FROM alpine:latest

RUN apk update
RUN apk add --no-cache \
    libgcc \
    libstdc++ \
    libc6-compat \
    openssl-dev

WORKDIR /echo_bot
COPY --from=build /echo_bot/build/echo_bot .
CMD source /run/secrets/discord-oscuro-env; ./echo_bot
