#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main() {
    int sock0, sock;
    socklen_t len;
    struct sockaddr_in addr, client;
    int yes = 1;
    char buf[2048];
    char req_buf[2048];

    sock0 = socket(AF_INET, SOCK_STREAM, 0);
    if (sock0 < 0) {
        fprintf(stderr, "socket error");
        return -1;
    }

    addr.sin_family = AF_INET;
    addr.sin_port = htons(80);
    addr.sin_addr.s_addr = INADDR_ANY;
    setsockopt(sock0, SOL_SOCKET, SO_REUSEADDR, (const char *)&yes, sizeof(yes));

    if (bind(sock0, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
        fprintf(stderr, "bind error");
        return -1;
    }

    if (listen(sock0, 5) != 0) {
        fprintf(stderr, "listen error");
        return -1;
    }

    memset(buf, 0, sizeof(buf));
    snprintf(buf, sizeof(buf),
              "HTTP/1.0 200 OK\r\n"
              "Content-Type: text/html\r\n"
              "\r\n"
              "Hello\r\n");

    while (1) {
        len = sizeof(client);
        sock = accept(sock0, (struct sockaddr *)&client, &len);
        if (sock < 0) {
            fprintf(stderr, "accept error");
            break;
        }

        memset(req_buf, 0, sizeof(req_buf));
        recv(sock, req_buf, sizeof(req_buf), 0);
        // TODO: クライアントからのリクエストをパースする
        // printf("%s", req_buf);
        send(sock, buf, (int)strlen(buf), 0);

        close(sock);
    }
    close(sock0);
    return 0;
}
