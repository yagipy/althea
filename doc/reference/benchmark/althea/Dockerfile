FROM yagipy/althea:bullseye

WORKDIR /app

COPY . .

RUN althea main.alt -o /usr/local/bin/server -gc=ownrc

CMD ["server"]
