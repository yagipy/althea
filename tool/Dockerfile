FROM rust:bullseye

RUN apt update
RUN apt install build-essential libfontconfig1-dev lsb-release software-properties-common gnupg -y

RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 13

RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-13 1
RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-13 1
RUN update-alternatives --install /usr/bin/llvm-config llvm-config /usr/bin/llvm-config-13 1

COPY ./compiler /althea/compiler
COPY ./library /althea/library
COPY ./tool /althea/tool
COPY ./Cargo.toml /althea/Cargo.toml
COPY ./Cargo.lock /althea/Cargo.lock

RUN cargo build -r --manifest-path /althea/Cargo.toml
RUN cp /althea/target/release/alc /usr/local/bin/althea

RUN rm -rf /althea
