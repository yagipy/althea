FROM debian:bullseye

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y \
  vim \
  gcc \
  g++
RUN gcc -o /usr/local/bin/benchmark-server-althea main.c

CMD ["benchmark-server-althea"]
