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
        return -1;
    }

    addr.sin_family = AF_INET;
    addr.sin_port = htons(80);
    addr.sin_addr.s_addr = INADDR_ANY;

    if (bind(sock0, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
        return -1;
    }

    if (listen(sock0, 50) != 0) {
        return -1;
    }

    snprintf(buf, sizeof(buf),
             "HTTP/1.0 200 OK\r\n"
             "Content-Type: text/html\r\n"
             "\r\n"
             "Hello\r\n");

    while (1) {
        sock = accept(sock0, NULL, NULL);
        if (sock < 0) {
            break;
        }

        memset(req_buf, 0, sizeof(req_buf));
        recv(sock, req_buf, sizeof(req_buf), 0);
        send(sock, buf, strlen(buf), 0);

        close(sock);
    }
    close(sock0);
    return 0;
}
