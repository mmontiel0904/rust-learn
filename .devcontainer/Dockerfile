FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
RUN apt-get update
RUN apt-get update && apt-get install -y wget unzip && wget -qO jetbrains-mono.zip https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip && unzip jetbrains-mono.zip -d /usr/share/fonts && fc-cache -f -v && rm -rf jetbrains-mono.zip