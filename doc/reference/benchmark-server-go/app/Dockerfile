FROM golang:1.19.0-buster

ENV GO111MODULE=on
ENV GOOS=linux
ENV GOARCH=amd64

WORKDIR /app

COPY go.mod go.mod
RUN go mod download

COPY . .

RUN go build -o /usr/local/bin/benchmark-server-go main.go

CMD ["benchmark-server-go"]
