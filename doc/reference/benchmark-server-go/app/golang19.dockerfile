FROM debian:bullseye

ENV GOOS=linux
ENV GOARCH=amd64
ENV GOROOT=/usr/local/go
ENV PATH=$GOPATH/bin:$GOROOT/bin:$PATH

RUN apt-get update && apt-get install -y \
  wget
RUN wget https://storage.googleapis.com/golang/go1.9.7.linux-amd64.tar.gz
RUN tar -xvf go1.9.7.linux-amd64.tar.gz
RUN mv go /usr/local

WORKDIR /app

COPY . .

RUN go build -o /usr/local/bin/benchmark-server-go main.go

CMD ["benchmark-server-go"]
