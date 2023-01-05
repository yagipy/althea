; ModuleID = '/root/althea/doc/reference/benchmark-server-althea/app/sample.c'
source_filename = "/root/althea/doc/reference/benchmark-server-althea/app/sample.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct._IO_FILE = type { i32, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, %struct._IO_marker*, %struct._IO_FILE*, i32, i32, i64, i16, i8, [1 x i8], i8*, i64, %struct._IO_codecvt*, %struct._IO_wide_data*, %struct._IO_FILE*, i8*, i64, i32, [20 x i8] }
%struct._IO_marker = type opaque
%struct._IO_codecvt = type opaque
%struct._IO_wide_data = type opaque
; %struct.sockaddr_in = type { i16, i16, { i32 }, [8 x i8] }
; %struct.in_addr = type { i32 }
; %struct.sockaddr = type { i16, [14 x i8] }

@stderr = external dso_local global %struct._IO_FILE*, align 8
@.str = private unnamed_addr constant [13 x i8] c"socket error\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"bind error\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"listen error\00", align 1
@.str.3 = private unnamed_addr constant [52 x i8] c"HTTP/1.0 200 OK\0D\0AContent-Type: text/html\0D\0A\0D\0AHello\0D\0A\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"accept error\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca { i16, i16, { i32 }, [8 x i8] }, align 4
  %6 = alloca { i16, i16, { i32 }, [8 x i8] }, align 4
  %7 = alloca i32, align 4
  %8 = alloca [2048 x i8], align 16
  %9 = alloca [2048 x i8], align 16
  store i32 0, i32* %1, align 4
  store i32 1, i32* %7, align 4
  %10 = call i32 @socket(i32 2, i32 1, i32 0) #6
  store i32 %10, i32* %2, align 4
  %11 = load i32, i32* %2, align 4
  %12 = icmp slt i32 %11, 0
  br i1 %12, label %13, label %16

13:                                               ; preds = %0
  %14 = load %struct._IO_FILE*, %struct._IO_FILE** @stderr, align 8
  %15 = call i32 (%struct._IO_FILE*, i8*, ...) @fprintf(%struct._IO_FILE* %14, i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %69

16:                                               ; preds = %0
  %17 = getelementptr inbounds { i16, i16, { i32 }, [8 x i8] }, { i16, i16, { i32 }, [8 x i8] }* %5, i32 0, i32 0
  store i16 2, i16* %17, align 4
  %18 = call zeroext i16 @htons(i16 zeroext 80) #7
  %19 = getelementptr inbounds { i16, i16, { i32 }, [8 x i8] }, { i16, i16, { i32 }, [8 x i8] }* %5, i32 0, i32 1
  store i16 %18, i16* %19, align 2
  %20 = getelementptr inbounds { i16, i16, { i32 }, [8 x i8] }, { i16, i16, { i32 }, [8 x i8] }* %5, i32 0, i32 2
  %21 = getelementptr inbounds { i32 }, { i32 }* %20, i32 0, i32 0
  store i32 0, i32* %21, align 4
  %22 = load i32, i32* %2, align 4
  %23 = bitcast i32* %7 to i8*
  %24 = call i32 @setsockopt(i32 %22, i32 1, i32 2, i8* %23, i32 4) #6
  %25 = load i32, i32* %2, align 4
  %26 = bitcast { i16, i16, { i32 }, [8 x i8] }* %5 to { i16, [14 x i8] }*
  %27 = call i32 @bind(i32 %25, { i16, [14 x i8] }* %26, i32 16) #6
  %28 = icmp ne i32 %27, 0
  br i1 %28, label %29, label %32

29:                                               ; preds = %16
  %30 = load %struct._IO_FILE*, %struct._IO_FILE** @stderr, align 8
  %31 = call i32 (%struct._IO_FILE*, i8*, ...) @fprintf(%struct._IO_FILE* %30, i8* getelementptr inbounds ([11 x i8], [11 x i8]* @.str.1, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %69

32:                                               ; preds = %16
  %33 = load i32, i32* %2, align 4
  %34 = call i32 @listen(i32 %33, i32 5) #6
  %35 = icmp ne i32 %34, 0
  br i1 %35, label %36, label %39

36:                                               ; preds = %32
  %37 = load %struct._IO_FILE*, %struct._IO_FILE** @stderr, align 8
  %38 = call i32 (%struct._IO_FILE*, i8*, ...) @fprintf(%struct._IO_FILE* %37, i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str.2, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %69

39:                                               ; preds = %32
  %40 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %40, i8 0, i64 2048, i1 false)
  %41 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %42 = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %41, i64 2048, i8* getelementptr inbounds ([52 x i8], [52 x i8]* @.str.3, i64 0, i64 0)) #6
  br label %43

43:                                               ; preds = %39, %52
  store i32 16, i32* %4, align 4
  %44 = load i32, i32* %2, align 4
  %45 = bitcast { i16, i16, { i32 }, [8 x i8] }* %6 to { i16, [14 x i8] }*
  %46 = call i32 @accept(i32 %44, { i16, [14 x i8] }* %45, i32* %4)
  store i32 %46, i32* %3, align 4
  %47 = load i32, i32* %3, align 4
  %48 = icmp slt i32 %47, 0
  br i1 %48, label %49, label %52

49:                                               ; preds = %43
  %50 = load %struct._IO_FILE*, %struct._IO_FILE** @stderr, align 8
  %51 = call i32 (%struct._IO_FILE*, i8*, ...) @fprintf(%struct._IO_FILE* %50, i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str.4, i64 0, i64 0))
  br label %66

52:                                               ; preds = %43
  %53 = getelementptr inbounds [2048 x i8], [2048 x i8]* %9, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %53, i8 0, i64 2048, i1 false)
  %54 = load i32, i32* %3, align 4
  %55 = getelementptr inbounds [2048 x i8], [2048 x i8]* %9, i64 0, i64 0
  %56 = call i64 @recv(i32 %54, i8* %55, i64 2048, i32 0)
  %57 = load i32, i32* %3, align 4
  %58 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %59 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %60 = call i64 @strlen(i8* %59) #8
  %61 = trunc i64 %60 to i32
  %62 = sext i32 %61 to i64
  %63 = call i64 @send(i32 %57, i8* %58, i64 %62, i32 0)
  %64 = load i32, i32* %3, align 4
  %65 = call i32 @close(i32 %64)
  br label %43

66:                                               ; preds = %49
  %67 = load i32, i32* %2, align 4
  %68 = call i32 @close(i32 %67)
  store i32 0, i32* %1, align 4
  br label %69

69:                                               ; preds = %66, %36, %29, %13
  %70 = load i32, i32* %1, align 4
  ret i32 %70
}

; Function Attrs: nounwind
declare dso_local i32 @socket(i32, i32, i32) #1

declare dso_local i32 @fprintf(%struct._IO_FILE*, i8*, ...) #2

; Function Attrs: nounwind readnone
declare dso_local zeroext i16 @htons(i16 zeroext) #3

; Function Attrs: nounwind
declare dso_local i32 @setsockopt(i32, i32, i32, i8*, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @bind(i32, { i16, [14 x i8] }*, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @listen(i32, i32) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #4

; Function Attrs: nounwind
declare dso_local i32 @snprintf(i8*, i64, i8*, ...) #1

declare dso_local i32 @accept(i32, { i16, [14 x i8] }*, i32*) #2

declare dso_local i64 @recv(i32, i8*, i64, i32) #2

declare dso_local i64 @send(i32, i8*, i64, i32) #2

; Function Attrs: nounwind readonly
declare dso_local i64 @strlen(i8*) #5

declare dso_local i32 @close(i32) #2

attributes #0 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind readnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { nounwind readonly "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #6 = { nounwind }
attributes #7 = { nounwind readnone }
attributes #8 = { nounwind readonly }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{!"clang version 10.0.0-4ubuntu1 "}
