FROM rust:bullseye

RUN apt update
RUN apt install build-essential libfontconfig1-dev lsb-release software-properties-common gnupg -y

RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 13

RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-13 1
RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-13 1
RUN update-alternatives --install /usr/bin/llvm-config llvm-config /usr/bin/llvm-config-13 1

WORKDIR /althea
