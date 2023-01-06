; ModuleID = 'sample.c'
source_filename = "sample.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx12.0.0"

%struct.sockaddr_in = type { i8, i8, i16, %struct.in_addr, [8 x i8] }
%struct.in_addr = type { i32 }
%struct.sockaddr = type { i8, i8, [14 x i8] }

@.str = private unnamed_addr constant [52 x i8] c"HTTP/1.0 200 OK\0D\0AContent-Type: text/html\0D\0A\0D\0AHello\0D\0A\00", align 1

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
  br i1 %9, label %10, label %11

10:                                               ; preds = %0
  store i32 -1, i32* %1, align 4
  br label %51

11:                                               ; preds = %0
  %12 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 1
  store i8 2, i8* %12, align 1
  %13 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 2
  store i16 20480, i16* %13, align 2
  %14 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 3
  %15 = getelementptr inbounds %struct.in_addr, %struct.in_addr* %14, i32 0, i32 0
  store i32 0, i32* %15, align 4
  %16 = load i32, i32* %2, align 4
  %17 = bitcast %struct.sockaddr_in* %4 to %struct.sockaddr*
  %18 = call i32 @"\01_bind"(i32 noundef %16, %struct.sockaddr* noundef %17, i32 noundef 16)
  %19 = icmp ne i32 %18, 0
  br i1 %19, label %20, label %21

20:                                               ; preds = %11
  store i32 -1, i32* %1, align 4
  br label %51

21:                                               ; preds = %11
  %22 = load i32, i32* %2, align 4
  %23 = call i32 @"\01_listen"(i32 noundef %22, i32 noundef 5)
  %24 = icmp ne i32 %23, 0
  br i1 %24, label %25, label %26

25:                                               ; preds = %21
  store i32 -1, i32* %1, align 4
  br label %51

26:                                               ; preds = %21
  %27 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %27, i8 0, i64 2048, i1 false)
  %28 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %29 = call i32 (i8*, i64, i32, i64, i8*, ...) @__snprintf_chk(i8* noundef %28, i64 noundef 2048, i32 noundef 0, i64 noundef 2048, i8* noundef getelementptr inbounds ([52 x i8], [52 x i8]* @.str, i64 0, i64 0))
  br label %30

30:                                               ; preds = %26, %36
  %31 = load i32, i32* %2, align 4
  %32 = call i32 @"\01_accept"(i32 noundef %31, %struct.sockaddr* noundef null, i32* noundef null)
  store i32 %32, i32* %3, align 4
  %33 = load i32, i32* %3, align 4
  %34 = icmp slt i32 %33, 0
  br i1 %34, label %35, label %36

35:                                               ; preds = %30
  br label %48

36:                                               ; preds = %30
  %37 = getelementptr inbounds [2048 x i8], [2048 x i8]* %6, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %37, i8 0, i64 2048, i1 false)
  %38 = load i32, i32* %3, align 4
  %39 = getelementptr inbounds [2048 x i8], [2048 x i8]* %6, i64 0, i64 0
  %40 = call i64 @"\01_recv"(i32 noundef %38, i8* noundef %39, i64 noundef 2048, i32 noundef 0)
  %41 = load i32, i32* %3, align 4
  %42 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %43 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %44 = call i64 @strlen(i8* noundef %43)
  %45 = call i64 @"\01_send"(i32 noundef %41, i8* noundef %42, i64 noundef %44, i32 noundef 0)
  %46 = load i32, i32* %3, align 4
  %47 = call i32 @"\01_close"(i32 noundef %46)
  br label %30

48:                                               ; preds = %35
  %49 = load i32, i32* %2, align 4
  %50 = call i32 @"\01_close"(i32 noundef %49)
  store i32 0, i32* %1, align 4
  br label %51

51:                                               ; preds = %48, %25, %20, %10
  %52 = load i32, i32* %1, align 4
  ret i32 %52
}

declare i32 @socket(i32 noundef, i32 noundef, i32 noundef) #1

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
