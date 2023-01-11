#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main() {
    int sock0, sock;
    struct sockaddr_in addr;
    char buf[2048];
    char req_buf[2048];

    sock0 = socket(AF_INET, SOCK_STREAM, 0);
    if (sock0 < 0) {
//        puts("socket error");
        return -1;
    }
//    puts("socket ok");

    addr.sin_family = AF_INET;
    addr.sin_port = htons(80);
    addr.sin_addr.s_addr = INADDR_ANY;

    if (bind(sock0, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
//        puts("bind error");
        return -1;
    }
//    puts("bind ok");

    if (listen(sock0, 5) != 0) {
//        puts("listen error");
        return -1;
    }
//    puts("listen ok");

//    memset(buf, 0, sizeof(buf));
    snprintf(buf, sizeof(buf),
              "HTTP/1.0 200 OK\r\n"
              "Content-Type: text/html\r\n"
              "\r\n"
              "Hello\r\n");
//    puts("snprintf ok");

    while (1) {
        sock = accept(sock0, NULL, NULL);
        if (sock < 0) {
//            puts("accept error");
            break;
        }
//        puts("accept ok");

        memset(req_buf, 0, sizeof(req_buf));
        recv(sock, req_buf, sizeof(req_buf), 0);
//        puts("recv ok");
        send(sock, buf, strlen(buf), 0);
//        puts("send ok");

        close(sock);
//        puts("close(sock) ok");
    }
    close(sock0);
//    puts("close(sock0) ok");
    return 0;
}
