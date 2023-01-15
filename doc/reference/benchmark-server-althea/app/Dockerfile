FROM yagipy/althea:bullseye

WORKDIR /app

COPY . .

RUN althea main.alt -o /usr/local/bin/benchmark-server-althea

CMD ["benchmark-server-althea"]
