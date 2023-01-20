import com.sun.net.httpserver.Headers;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;
import java.net.*;
import java.io.*;
import java.nio.charset.StandardCharsets;

class MyServer {
    public static void main(String argv[]) throws IOException {
        // TODO: Allocate 80MB at heap
        HttpServer server = HttpServer.create(new InetSocketAddress(80), 0);
        server.createContext("/", new MyHandler());
        server.start();
    }

    private static class MyHandler implements HttpHandler {
        public void handle(HttpExchange t) throws IOException {
            Headers resHeaders = t.getResponseHeaders();
            resHeaders.set("Content-Type", "application/json");

            OutputStream os = t.getResponseBody();
            String resBody = "Hello";
            long contentLength = resBody.getBytes(StandardCharsets.UTF_8).length;
            t.sendResponseHeaders(200, contentLength);

            os.write(resBody.getBytes());
            os.close();
        }
    }
}
