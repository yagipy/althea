; ModuleID = 'sample.c'
source_filename = "sample.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.sockaddr_in = type { i16, i16, %struct.in_addr, [8 x i8] }
%struct.in_addr = type { i32 }
%struct.sockaddr = type { i16, [14 x i8] }

@.str = private unnamed_addr constant [52 x i8] c"HTTP/1.0 200 OK\0D\0AContent-Type: text/html\0D\0A\0D\0AHello\0D\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca %struct.sockaddr_in, align 4
  %5 = alloca [2048 x i8], align 16
  %6 = alloca [2048 x i8], align 16
  store i32 0, i32* %1, align 4
  %7 = call i32 @socket(i32 2, i32 1, i32 0) #6
  store i32 %7, i32* %2, align 4
  %8 = load i32, i32* %2, align 4
  %9 = icmp slt i32 %8, 0
  br i1 %9, label %10, label %11

10:                                               ; preds = %0
  store i32 -1, i32* %1, align 4
  br label %51

11:                                               ; preds = %0
  %12 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 0
  store i16 2, i16* %12, align 4
  %13 = call zeroext i16 @htons(i16 zeroext 80) #7
  %14 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 1
  store i16 %13, i16* %14, align 2
  %15 = getelementptr inbounds %struct.sockaddr_in, %struct.sockaddr_in* %4, i32 0, i32 2
  %16 = getelementptr inbounds %struct.in_addr, %struct.in_addr* %15, i32 0, i32 0
  store i32 0, i32* %16, align 4
  %17 = load i32, i32* %2, align 4
  %18 = bitcast %struct.sockaddr_in* %4 to %struct.sockaddr*
  %19 = call i32 @bind(i32 %17, %struct.sockaddr* %18, i32 16) #6
  %20 = icmp ne i32 %19, 0
  br i1 %20, label %21, label %22

21:                                               ; preds = %11
  store i32 -1, i32* %1, align 4
  br label %51

22:                                               ; preds = %11
  %23 = load i32, i32* %2, align 4
  %24 = call i32 @listen(i32 %23, i32 5) #6
  %25 = icmp ne i32 %24, 0
  br i1 %25, label %26, label %27

26:                                               ; preds = %22
  store i32 -1, i32* %1, align 4
  br label %51

27:                                               ; preds = %22
  %28 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %29 = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %28, i64 2048, i8* getelementptr inbounds ([52 x i8], [52 x i8]* @.str, i64 0, i64 0)) #6
  br label %30

30:                                               ; preds = %27, %36
  %31 = load i32, i32* %2, align 4
  %32 = call i32 @accept(i32 %31, %struct.sockaddr* null, i32* null)
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
  %40 = call i64 @recv(i32 %38, i8* %39, i64 2048, i32 0)
  %41 = load i32, i32* %3, align 4
  %42 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %43 = getelementptr inbounds [2048 x i8], [2048 x i8]* %5, i64 0, i64 0
  %44 = call i64 @strlen(i8* %43) #8
  %45 = call i64 @send(i32 %41, i8* %42, i64 %44, i32 0)
  %46 = load i32, i32* %3, align 4
  %47 = call i32 @close(i32 %46)
  br label %30

48:                                               ; preds = %35
  %49 = load i32, i32* %2, align 4
  %50 = call i32 @close(i32 %49)
  store i32 0, i32* %1, align 4
  br label %51

51:                                               ; preds = %48, %26, %21, %10
  %52 = load i32, i32* %1, align 4
  ret i32 %52
}

; Function Attrs: nounwind
declare dso_local i32 @socket(i32, i32, i32) #1

; Function Attrs: nounwind readnone
declare dso_local zeroext i16 @htons(i16 zeroext) #2

; Function Attrs: nounwind
declare dso_local i32 @bind(i32, %struct.sockaddr*, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @listen(i32, i32) #1

; Function Attrs: nounwind
declare dso_local i32 @snprintf(i8*, i64, i8*, ...) #1

declare dso_local i32 @accept(i32, %struct.sockaddr*, i32*) #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #4

declare dso_local i64 @recv(i32, i8*, i64, i32) #3

declare dso_local i64 @send(i32, i8*, i64, i32) #3

; Function Attrs: nounwind readonly
declare dso_local i64 @strlen(i8*) #5

declare dso_local i32 @close(i32) #3

attributes #0 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { nounwind readnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { nounwind readonly "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #6 = { nounwind }
attributes #7 = { nounwind readnone }
attributes #8 = { nounwind readonly }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{!"clang version 10.0.0-4ubuntu1 "}
