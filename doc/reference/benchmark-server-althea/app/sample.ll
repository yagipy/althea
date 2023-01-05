; ModuleID = 'sample.c'
source_filename = "sample.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.sockaddr_in = type { i16, i16, %struct.in_addr, [8 x i8] }
%struct.in_addr = type { i32 }
%struct.sockaddr = type { i16, [14 x i8] }

@.str = private unnamed_addr constant [13 x i8] c"socket error\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"bind error\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"bind\00", align 1
@.str.3 = private unnamed_addr constant [13 x i8] c"listen error\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"listen\00", align 1
@.str.5 = private unnamed_addr constant [52 x i8] c"HTTP/1.0 200 OK\0D\0AContent-Type: text/html\0D\0A\0D\0AHello\0D\0A\00", align 1
@.str.6 = private unnamed_addr constant [13 x i8] c"accept error\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"accept\00", align 1
@.str.8 = private unnamed_addr constant [5 x i8] c"recv\00", align 1
@.str.9 = private unnamed_addr constant [5 x i8] c"send\00", align 1
@.str.10 = private unnamed_addr constant [6 x i8] c"close\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca %struct.sockaddr_in, align 4
  %6 = alloca %struct.sockaddr_in, align 4
  %7 = alloca i32, align 4
  %8 = alloca [2048 x i8], align 16
  %9 = alloca [2048 x i8], align 16
  store i32 0, i32* %1, align 4
  store i32 1, i32* %7, align 4
  %10 = call i32 @socket(i32 2, i32 1, i32 0) #6
  store i32 %10, i32* %2, align 4
  %11 = load i32, i32* %2, align 4
  %12 = icmp slt i32 %11, 0
  br i1 %12, label %13, label %15

13:                                               ; preds = %0
  %14 = call i32 @puts(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %67

15:                                               ; preds = %0
  %16 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %5, i32 0, i32 0
  store i16 2, i16* %16, align 4
  %17 = call zeroext i16 @htons(i16 zeroext 80) #7
  %18 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %5, i32 0, i32 1
  store i16 %17, i16* %18, align 2
  %19 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %5, i32 0, i32 2
  %20 = getelementptr inbounds %struct.in_addr, %struct.in_addr* %19, i32 0, i32 0
  store i32 0, i32* %20, align 4
  %21 = load i32, i32* %2, align 4
  %22 = bitcast i32* %7 to i8*
  %23 = call i32 @setsockopt(i32 %21, i32 1, i32 2, i8* %22, i32 4) #6
  %24 = load i32, i32* %2, align 4
  %25 = bitcast %struct.sockaddr_in* %5 to %struct.sockaddr*
  %26 = call i32 @bind(i32 %24, %struct.sockaddr* %25, i32 16) #6
  %27 = icmp ne i32 %26, 0
  br i1 %27, label %28, label %30

28:                                               ; preds = %15
  %29 = call i32 @puts(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @.str.1, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %67

30:                                               ; preds = %15
  %31 = call i32 @puts(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.2, i64 0, i64 0))
  %32 = load i32, i32* %2, align 4
  %33 = call i32 @listen(i32 %32, i32 5) #6
  %34 = icmp ne i32 %33, 0
  br i1 %34, label %35, label %37

35:                                               ; preds = %30
  %36 = call i32 @puts(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str.3, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %67

37:                                               ; preds = %30
  %38 = call i32 @puts(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.str.4, i64 0, i64 0))
  %39 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %39, i8 0, i64 2048, i1 false)
  %40 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %41 = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %40, i64 2048, i8* getelementptr inbounds ([52 x i8], [52 x i8]* @.str.5, i64 0, i64 0)) #6
  store i32 16, i32* %4, align 4
  %42 = load i32, i32* %2, align 4
  %43 = bitcast %struct.sockaddr_in* %6 to %struct.sockaddr*
  %44 = call i32 @accept(i32 %42, %struct.sockaddr* %43, i32* %4)
  store i32 %44, i32* %3, align 4
  %45 = load i32, i32* %3, align 4
  %46 = icmp slt i32 %45, 0
  br i1 %46, label %47, label %49

47:                                               ; preds = %37
  %48 = call i32 @puts(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str.6, i64 0, i64 0))
  store i32 -1, i32* %1, align 4
  br label %67

49:                                               ; preds = %37
  %50 = call i32 @puts(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.str.7, i64 0, i64 0))
  %51 = getelementptr inbounds [2048 x i8], [2048 x i8]* %9, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* align 16 %51, i8 0, i64 2048, i1 false)
  %52 = load i32, i32* %3, align 4
  %53 = getelementptr inbounds [2048 x i8], [2048 x i8]* %9, i64 0, i64 0
  %54 = call i64 @recv(i32 %52, i8* %53, i64 2048, i32 0)
  %55 = call i32 @puts(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.8, i64 0, i64 0))
  %56 = load i32, i32* %3, align 4
  %57 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %58 = getelementptr inbounds [2048 x i8], [2048 x i8]* %8, i64 0, i64 0
  %59 = call i64 @strlen(i8* %58) #8
  %60 = call i64 @send(i32 %56, i8* %57, i64 %59, i32 0)
  %61 = call i32 @puts(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.9, i64 0, i64 0))
  %62 = load i32, i32* %3, align 4
  %63 = call i32 @close(i32 %62)
  %64 = call i32 @puts(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str.10, i64 0, i64 0))
  %65 = load i32, i32* %2, align 4
  %66 = call i32 @close(i32 %65)
  store i32 0, i32* %1, align 4
  br label %67

67:                                               ; preds = %49, %47, %35, %28, %13
  %68 = load i32, i32* %1, align 4
  ret i32 %68
}

; Function Attrs: nounwind
declare dso_local i32 @socket(i32, i32, i32) #1

declare dso_local i32 @puts(i8*) #2

; Function Attrs: nounwind readnone
declare dso_local zeroext i16 @htons(i16 zeroext) #3

; Function Attrs: nounwind
declare dso_local i32 @setsockopt(i32, i32, i32, i8*, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @bind(i32, %struct.sockaddr*, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @listen(i32, i32) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #4

; Function Attrs: nounwind
declare dso_local i32 @snprintf(i8*, i64, i8*, ...) #1

declare dso_local i32 @accept(i32, %struct.sockaddr*, i32*) #2

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
