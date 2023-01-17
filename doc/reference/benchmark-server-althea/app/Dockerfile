FROM yagipy/althea:bullseye

WORKDIR /app

COPY . .

# TODO: GCを指定する
RUN althea main.alt -o /usr/local/bin/benchmark-server-althea

CMD ["benchmark-server-althea"]
