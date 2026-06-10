target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(ptr noundef, ...)

@hello = constant [6 x i8] c"hello\0A"

define i32 @main() {
    call i32 @printf(ptr @hello)
    ret i32 0
}