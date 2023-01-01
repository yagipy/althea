declare void @exit(i32 %status)
declare i32 @puts(i8* %str)
declare i32 @printf(i8* %format, ...)
declare i32 @sprintf(i8* %buf, i8* %format, ...)
declare i8* @malloc(i32 %size)
declare void @free(i8* %ptr)
declare i32 @strcmp(i8* %str1, i8* %str2)
declare i8* @strncpy(i8* %dest, i8* %src, i32 %num)
declare i32 @strlen(i8* %str)
declare i8* @strstr(i8* %haystack, i8* %needle)
declare i32 @read(i32 %fd, i8* %buf, i32 %len)
declare i32 @write(i32 %fd, i8* %buf, i32 %len)
declare i32 @close(i32 %fd)
declare i32 @socket(i32 %domain, i32 %type, i32 %protocol)
declare i32 @bind(i32 %sockfd, %SockAddrIn*, i32 %len)
declare i32 @listen(i32 %sockfd, i32 %backlog)
declare i32 @accept(i32 %sockfd, %SockAddrIn*, i32* %len)
; declare i32 @setsockopt(i32, i32, i32, i8*, i32)

@.str.d = constant [4 x i8] c"%d\0A\00" ; %d\n\0
@.str.hex = constant [6 x i8] c"0x%x\0A\00" ; 0x%x\n\0
@str.create_socket_fail = constant [23 x i8] c"Socket creation failed\00"
@str.listen_socket_fail = constant [21 x i8] c"Socket listen failed\00"
@str.listen_socket_success = constant [13 x i8] c"Listening...\00"
@str.socket_accept_fail = constant [21 x i8] c"Socket accept failed\00"
@str.socket_accept_success = constant [16 x i8] c"New connection!\00"
@str.bind_socket_fail = constant [19 x i8] c"Socket bind failed\00"
%T.str.http_404 = type [49 x i8]
@str.http_404 = constant %T.str.http_404
  c"HTTP/1.1 404 Not Found\0D\0AContent-Length: 3\0D\0A\0D\0A404\00"
%T.str.ua = type [59 x i8]
@str.ua = constant %T.str.ua
  c"HTTP/1.1 200 OK\0D\0AContent-Length: %d\0D\0A\0D\0AYour User-Agent: %s\00"
%T.str.empty_ua = type [19 x i8]
@str.empty_ua = constant %T.str.empty_ua c"<Empty User-Agent>\00"
@str.ua_header = constant [13 x i8] c"User-Agent: \00"
@str.newline = constant [3 x i8] c"\0D\0A\00" ; \r\n\0
%T.buf = type [4096 x i8]
%T.str.valid_header = type [12 x i8]
@str.valid_header = constant %T.str.valid_header c"GET / HTTP/\00"

@SOCK_STREAM = constant i32 1
@SOCK_DGRAM = constant i32 2
@SOCK_RAW = constant i32 3
@SOCK_RDM = constant i32 4
@SOCK_SEQPACKET = constant i32 5
@AF_INET = constant i32 2
@CASTED = constant i32 1

define i32 @print_i32(i32 %n) alwaysinline {
  %res = call i32(i8*, ...) @printf(
    i8* getelementptr ([4 x i8], [4 x i8]* @.str.d, i32 0, i32 0), i32 %n)
  ret i32 %res
}

define i32 @print_i8_ptr(i8* %n) alwaysinline {
  %res = call i32(i8*, ...) @printf(
    i8* getelementptr ([6 x i8], [6 x i8]* @.str.hex, i32 0, i32 0), i8* %n)
  ret i32 %res
}

define i8* @substr(i8* %str, i32 %offset, i32 %length) {
  %newlen = add i32 %length, 1
  %buf = call i8* @malloc(i32 %newlen)

  %lsubstr = getelementptr i8, i8* %str, i32 %offset ; %str + %offset
  call i8* @strncpy(i8* %buf, i8* %lsubstr, i32 %length)

  %last.ptr = getelementptr i8, i8* %buf, i32 %length ; %buf + %length
  store i8 0, i8* %last.ptr, align 1

  ret i8* %buf
}

; typedef __uint8_t    sa_family_t;
; typedef  __uint16_t    in_port_t;
; typedef  __uint32_t  in_addr_t;
; struct sockaddr {
;   __uint8_t  sa_len;    /* total length */
;   sa_family_t  sa_family;  /* [XSI] address family */
;   char    sa_data[14];  /* [XSI] addr value (actually larger) */
; };
; struct in_addr {
;   in_addr_t s_addr;
; };
; struct sockaddr_in {
;   __uint8_t  sin_len;
;   sa_family_t  sin_family;
;   in_port_t  sin_port;
;   struct  in_addr sin_addr;
;   char    sin_zero[8];
; };

; { sa_len, sa_family, sa_data }
; %SockAddr = type { i8, i8, [14 x i8] } ; not used

; { s_addr }
%InAddr = type { i32 }

; { sin_len, sin_family, sin_port, sin_addr, sin_zero }
%SockAddrIn = type { i8, i8, i16, %InAddr, [8 x i8] }

define i32 @create_socket() {
  %1 = load i32, i32* @AF_INET
  %2 = load i32, i32* @SOCK_STREAM
  %sockfd = call i32 @socket(i32 %1, i32 %2, i32 0)
  %cond = icmp eq i32 %sockfd, -1 ; %sockfd == -1
  br i1 %cond, label %Fail, label %Exit
Fail:
  %ptr = getelementptr [23 x i8], [23 x i8]* @str.create_socket_fail, i32 0, i32 0
  call i32 @puts(i8* %ptr)
  call void @exit(i32 1)
  unreachable
Exit:
  ret i32 %sockfd
}

@.sock_addr = constant %SockAddrIn {
  i8 16, ; length
  i8 2, ; AF_INET
  i16 47115, ; port: 3000 (little endian)
  %InAddr { i32 0 }, ; address: 0.0.0.0
  [8 x i8] [i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0]
}

define void @bind_socket(i32 %sockfd) {
  ; %sock_addr_in = alloca %SockAddrIn
  ; %casted = bitcast i32* @CASTED to i8*
  ; %setsockopt_res = call i32 @setsockopt(i32 %sockfd, i32 1, i32 2, i8* %casted, i32 4)
  %res = call i32 @bind(i32 %sockfd, %SockAddrIn* @.sock_addr, i32 16)
  %cond = icmp ne i32 %res, 0 ; %res != 0
  br i1 %cond, label %Fail, label %Exit
Fail:
  %ptr = getelementptr [19 x i8], [19 x i8]* @str.bind_socket_fail, i32 0, i32 0
  call i32 @puts(i8* %ptr)
  call i32 @print_i32(i32 %res)
  call void @exit(i32 1)
  unreachable
Exit:
  ret void
}

define void @listen_socket(i32 %sockfd) {
  %res = call i32 @listen(i32 %sockfd, i32 5)
  %cond = icmp ne i32 %res, 0 ; %res != 0
  br i1 %cond, label %Fail, label %Exit
Fail:
  %ptr = getelementptr [21 x i8], [21 x i8]* @str.listen_socket_fail, i32 0, i32 0
  call i32 @puts(i8* %ptr)
  call i32 @print_i32(i32 %res)
  call void @exit(i32 1)
  unreachable
Exit:
  %ptr2 = getelementptr [13 x i8], [13 x i8]* @str.listen_socket_success, i32 0, i32 0
  call i32 @puts(i8* %ptr2)
  ret void
}

define void @print_new_conn() alwaysinline {
  %ptr = getelementptr [16 x i8], [16 x i8]* @str.socket_accept_success, i32 0, i32 0
  call i32 @puts(i8* %ptr)
  ret void
}

; \r = \0D
; \n = \0A
; \r\n = \0D\0A

define void @send_404(i32 %connfd) {
  %ptr = getelementptr %T.str.http_404, %T.str.http_404* @str.http_404, i32 0, i32 0
  call i32 @write(i32 %connfd, i8* %ptr, i32 48)
  ; call i32 @close(i32 %connfd)
  ret void
}

define void @send_user_agent(i32 %connfd, i8* %ua) {
  %fmt.ptr = getelementptr %T.str.ua, %T.str.ua* @str.ua, i32 0, i32 0
  %len.tmp = call i32 @strlen(i8* %ua)
  %len = add i32 %len.tmp, 17 ; Length of 'Your User-Agent: '
  %output.len = add i32 %len, 512 ; Yeah, just add 512.
  %output = alloca i8, i32 %output.len, align 1
  call i32(i8*, i8*, ...) @sprintf(i8* %output, i8* %fmt.ptr, i32 %len, i8* %ua)
  %output.len.actual = call i32 @strlen(i8* %output)
  call i32 @write(i32 %connfd, i8* %output, i32 %output.len.actual)
  ret void
}

define i8* @obtain_user_agent(i8* %buf) {
  %ua_header_str = getelementptr [13 x i8], [13 x i8]* @str.ua_header, i32 0, i32 0
  %newline_str = getelementptr [3 x i8], [3 x i8]* @str.newline, i32 0, i32 0

  %ua_header.start = call i8* @strstr(i8* %buf, i8* %ua_header_str)

  %cond0 = icmp ne i8* %ua_header.start, null
  br i1 %cond0, label %Step2, label %NotFound
Step2:
  ; Skip "User-Agent: " part
  %ua_header.content_start = getelementptr i8, i8* %ua_header.start, i32 12 ; ptr+12

  %ua_header.end = call i8* @strstr(i8* %ua_header.content_start, i8* %newline_str)

  %cond1 = icmp ne i8* %ua_header.end, null
  br i1 %cond1, label %Step3, label %NotFound
Step3:
  store i8 0, i8* %ua_header.end, align 1
  call i32 @puts(i8* %ua_header.content_start)
  ret i8* %ua_header.content_start
NotFound:
  %empty_ua_str =
    getelementptr %T.str.empty_ua, %T.str.empty_ua* @str.empty_ua, i32 0, i32 0
  ret i8* %empty_ua_str
}

define void @on_new_connection(i32 %connfd) {
  %buf.ptr = alloca %T.buf
  %buf.ptr.i8 = bitcast %T.buf* %buf.ptr to i8*
  %len = call i32 @read(i32 %connfd, i8* %buf.ptr.i8, i32 4096)
  %last.ptr = getelementptr %T.buf, %T.buf* %buf.ptr, i32 0, i32 %len
  store i8 0, i8* %last.ptr

  call i32 @print_i32(i32 %len)
  call i32 @puts(i8* %buf.ptr.i8)

  ; if first 11 characters == "GET / HTTP/"
  ;   then send 200 OK with user's user-agent
  ;   else send 404 Not Found with body "404"

  %given_header = call i8* @substr(i8* %buf.ptr.i8, i32 0, i32 11)
  %valid_header = getelementptr %T.str.valid_header, %T.str.valid_header*
    @str.valid_header, i32 0, i32 0

  %res = call i32 @strcmp(i8* %given_header, i8* %valid_header)
  call void @free(i8* %given_header)
  %cond = icmp eq i32 %res, 0
  br i1 %cond, label %Success, label %Fail
Success:
  %useragent = call i8* @obtain_user_agent(i8* %buf.ptr.i8)
  call void @send_user_agent(i32 %connfd, i8* %useragent)
  br label %Exit
Fail:
  call void @send_404(i32 %connfd)
  br label %Exit
Exit:
  call i32 @close(i32 %connfd)
  ret void
}

define void @server_loop(i32 %sockfd) {
  %len.ptr = alloca i32
  store i32 16, i32* %len.ptr
  br label %Loop
Loop:
  %claddr.ptr = alloca %SockAddrIn ; client sockaddr_in
  %connfd = call i32 @accept(i32 %sockfd, %SockAddrIn* %claddr.ptr, i32* %len.ptr)
  %cond = icmp sle i32 %connfd, 0 ; %connfd < 0
  br i1 %cond, label %Fail, label %Connected
Connected:
  call void @print_new_conn()
  call void @on_new_connection(i32 %connfd)
  br label %Loop
Fail:
  %ptr = getelementptr [21 x i8], [21 x i8]* @str.socket_accept_fail, i32 0, i32 0
  call i32 @puts(i8* %ptr)
  call i32 @print_i32(i32 %connfd)
  call void @exit(i32 1)
  unreachable
}

define i32 @main() {
  %sockfd = call i32 @create_socket()
  call i32 @print_i32(i32 %sockfd)
  call void @bind_socket(i32 %sockfd)
  call void @listen_socket(i32 %sockfd)
  call void @server_loop(i32 %sockfd)
  call i32 @close(i32 %sockfd)
  ret i32 0
}
