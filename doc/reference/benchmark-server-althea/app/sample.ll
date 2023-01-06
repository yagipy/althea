; ModuleID = 'sample.c'
source_filename = "sample.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx12.0.0"

%struct.sockaddr_in = type { i8, i8, i16, %struct.in_addr, [8 x i8] }
%struct.in_addr = type { i32 }
%struct.sockaddr = type { i8, i8, [14 x i8] }

@.str = private unnamed_addr constant [13 x i8] c"socket error\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"socket ok\00", align 1
@.str.2 = private unnamed_addr constant [11 x i8] c"bind error\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"bind ok\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"listen error\00", align 1
@.str.5 = private unnamed_addr constant [10 x i8] c"listen ok\00", align 1
@.str.6 = private unnamed_addr constant [52 x i8] c"HTTP/1.0 200 OK\0D\0AContent-Type: text/html\0D\0A\0D\0AHello\0D\0A\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"snprintf ok\00", align 1
@.str.8 = private unnamed_addr constant [13 x i8] c"accept error\00", align 1
@.str.9 = private unnamed_addr constant [10 x i8] c"accept ok\00", align 1
@.str.10 = private unnamed_addr constant [8 x i8] c"recv ok\00", align 1
@.str.11 = private unnamed_addr constant [8 x i8] c"send ok\00", align 1
@.str.12 = private unnamed_addr constant [15 x i8] c"close(sock) ok\00", align 1
@.str.13 = private unnamed_addr constant [16 x i8] c"close(sock0) ok\00", align 1

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca %struct.sockaddr_in, align 4
  %5 = alloca [2048 x i8], align 16
  %6 = alloca [2048 x i8], align 16
  store i32 0, i32* %1, align 4
  %7 = call i32 @socket(i32 noundef 2, i32 noundef 1, i32 noundef 0)
  store i32 %7, i32* %2, align 4
  %8 = load i32, i32* %2, align 4
  %9 = icmp slt i32 %8, 0
  br i1 %9, label %10, label %12

10:                                               ; preds = %0
  %11 = call i32 @puts(i8* noundef getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %66

12:                                               ; preds = %0
  %13 = call i32 @puts(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.1, i64 0, i64 0))
  %14 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 1
  store i8 2, i8* %14, align 1
  %15 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 2
  store i16 20480, i16* %15, align 2
  %16 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 3
  %17 = getelementptr inbounds %struct.in_addr, %struct.in_addr* %16, i32 0, i32 0
  store i32 0, i32* %17, align 4
  %18 = load i32, i32* %2, align 4
  %19 = bitcast %struct.sockaddr_in* %4 to %struct.sockaddr*
  %20 = call i32 @"\01_bind"(i32 noundef %18, %struct.sockaddr* noundef %19, i32 noundef 16)
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %22, label %24

22:                                               ; preds = %12
  %23 = call i32 @puts(i8* noundef getelementptr inbounds ([11 x i8], [11 x i8]* @.str.2, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %66

24:                                               ; preds = %12
  %25 = call i32 @puts(i8* noundef getelementptr inbounds ([8 x i8], [8 x i8]* @.str.3, i64 0, i64 0))
  %26 = load i32, i32* %2, align 4
  %27 = call i32 @"\01_listen"(i32 noundef %26, i32 noundef 5)
  %28 = icmp ne i32 %27, 0
  br i1 %28, label %29, label %31

29:                                               ; preds = %24
  %30 = call i32 @puts(i8* noundef getelementptr inbounds ([13 x i8], [13 x i8]* @.str.4, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %66

31:                                               ; preds = %24
  %32 = call i32 @puts(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.5, i64 0, i64 0))
  %33 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %33, i8 0, i64 2048, i1 false)
  %34 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %35 = call i32 (i8*, i64, i32, i64, i8*, ...) @__snprintf_chk(i8* noundef %34, i64 noundef 2048, i32 noundef 0, i64 noundef 2048, i8* noundef getelementptr inbounds ([52 x i8], [52 x i8]* @.str.6, i64 0, i64 0))
  %36 = call i32 @puts(i8* noundef getelementptr inbounds ([12 x i8], [12 x i8]* @.str.7, i64 0, i64 0))
  br label %37

37:                                               ; preds = %31, %44
  %38 = load i32, i32* %2, align 4
  %39 = call i32 @"\01_accept"(i32 noundef %38, %struct.sockaddr* noundef null, i32* noundef null)
  store i32 %39, i32* %3, align 4
  %40 = load i32, i32* %3, align 4
  %41 = icmp slt i32 %40, 0
  br i1 %41, label %42, label %44

42:                                               ; preds = %37
  %43 = call i32 @puts(i8* noundef getelementptr inbounds ([13 x i8], [13 x i8]* @.str.8, i64 0, i64 0))
  br label %62

44:                                               ; preds = %37
  %45 = call i32 @puts(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.9, i64 0, i64 0))
  %46 = getelementptr inbounds [2048 x i8], [2048 x i8]* %6, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %46, i8 0, i64 2048, i1 false)
  %47 = load i32, i32* %3, align 4
  %48 = getelementptr inbounds [2048 x i8], [2048 x i8]* %6, i64 0, i64 0
  %49 = call i64 @"\01_recv"(i32 noundef %47, i8* noundef %48, i64 noundef 2048, i32 noundef 0)
  %50 = call i32 @puts(i8* noundef getelementptr inbounds ([8 x i8], [8 x i8]* @.str.10, i64 0, i64 0))
  %51 = load i32, i32* %3, align 4
  %52 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %53 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %54 = call i64 @strlen(i8* noundef %53)
  %55 = trunc i64 %54 to i32
  %56 = sext i32 %55 to i64
  %57 = call i64 @"\01_send"(i32 noundef %51, i8* noundef %52, i64 noundef %56, i32 noundef 0)
  %58 = call i32 @puts(i8* noundef getelementptr inbounds ([8 x i8], [8 x i8]* @.str.11, i64 0, i64 0))
  %59 = load i32, i32* %3, align 4
  %60 = call i32 @"\01_close"(i32 noundef %59)
  %61 = call i32 @puts(i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.12, i64 0, i64 0))
  br label %37

62:                                               ; preds = %42
  %63 = load i32, i32* %2, align 4
  %64 = call i32 @"\01_close"(i32 noundef %63)
  %65 = call i32 @puts(i8* noundef getelementptr inbounds ([16 x i8], [16 x i8]* @.str.13, i64 0, i64 0))
  store i32 0, i32* %1, align 4
  br label %66

66:                                               ; preds = %62, %29, %22, %10
  %67 = load i32, i32* %1, align 4
  ret i32 %67
}

declare i32 @socket(i32 noundef, i32 noundef, i32 noundef) #1

declare i32 @puts(i8* noundef) #1

declare i32 @"\01_bind"(i32 noundef, %struct.sockaddr* noundef, i32 noundef) #1

declare i32 @"\01_listen"(i32 noundef, i32 noundef) #1

; Function Attrs: argmemonly nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

declare i32 @__snprintf_chk(i8* noundef, i64 noundef, i32 noundef, i64 noundef, i8* noundef, ...) #1

declare i32 @"\01_accept"(i32 noundef, %struct.sockaddr* noundef, i32* noundef) #1

declare i64 @"\01_recv"(i32 noundef, i8* noundef, i64 noundef, i32 noundef) #1

declare i64 @"\01_send"(i32 noundef, i8* noundef, i64 noundef, i32 noundef) #1

declare i64 @strlen(i8* noundef) #1

declare i32 @"\01_close"(i32 noundef) #1

attributes #0 = { noinline nounwind optnone ssp uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 2}
!4 = !{!"Homebrew clang version 14.0.6"}
