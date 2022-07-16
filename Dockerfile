FROM ubuntu:latest

ENV PATH $PATH:$HOME/.cargo/bin
RUN apt update
RUN apt install -y build-essential curl llvm
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /althea
