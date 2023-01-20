FROM golang:1.19.0-bullseye

ENV GO111MODULE=on
ENV GOOS=linux
ENV GOARCH=amd64
ENV GOMAXPROCS=1

WORKDIR /app

COPY go.mod go.mod
RUN go mod download

COPY . .

RUN go build -o /usr/local/bin/server main.go

CMD ["server"]
