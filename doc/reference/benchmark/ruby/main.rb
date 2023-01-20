require 'socket'

server = TCPServer.new 80
loop do
  client = server.accept
  headers = []
  while header = client.gets
    break if header.chomp.empty?
    headers << header.chomp
  end

  # TODO: ヒープ領域に80MBをアロケートする
  fib(30)

  client.puts "HTTP/1.0 200 OK"
  client.puts "Content-Type: text/plain"
  client.puts
  client.puts "success"
  client.close
end

def fib(n)
  if n < 2
    return n
  end
  fib(n-2) + fib(n-1)
end
