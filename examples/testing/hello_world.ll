; ModuleID = 'examples/testing/hello_world.bc'
source_filename = "main"

define void @main() {
entry:
  call void @main.1()
  ret void
}

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

define i8 @char_at(ptr %0, i64 %1) {
entry:
  %ptr = ptrtoint ptr %0 to i64
  %char = inttoptr i64 %ptr to ptr
  %2 = load i8, ptr %char, align 1
  ret i8 %2
}

define void @main.1() {
entry:
  %0 = alloca [6 x i8], align 1
  store [6 x i8] c"hello\00", ptr %0, align 1
  %c = call i8 @char_at(ptr %0, i64 1)
  %1 = alloca [36 x i8], align 1
  store [36 x i8] c"char at %d: chr(%d) = '%c' -> \22%s\22\0A\00", ptr %1, align 1
  %2 = zext i8 %c to i32
  %3 = call [1 x i8] @char_str(i8 %c)
  %4 = call i32 (ptr, ...) @printf(ptr %1, i64 1, i32 %2, i8 %c, [1 x i8] %3)
  ret void
}

define [1 x i8] @char_str(i8 %0) {
entry:
  %1 = alloca i8, align 1
  store i8 %0, ptr %1, align 1
  %2 = load [1 x i8], ptr %1, align 1
  ret [1 x i8] %2
}
