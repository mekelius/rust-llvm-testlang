; ModuleID = 'test.c'
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(ptr noundef, ...)
declare i32 @__isoc99_scanf(ptr noundef, ...)

@inf = private unnamed_addr constant [3 x i8] c"%d\00"
@outf = private unnamed_addr constant [4 x i8] c"%d\0A\00"


define i32 @getInt() {
  %buf = alloca i32
  call i32 (ptr, ...) @__isoc99_scanf(ptr @inf, ptr %buf)
  %input = load i32, ptr %buf  
  ret i32 %input
}

define i32 @op(i32 %x) {
  entry:
    %res = mul i32 %x, 2
    ret i32 %res
}

define i32 @main() {
  Entry:
    %init = call i32 @getInt()
    br label %Loop

  Loop:
    %i = phi i32 [%init, %Entry], [%next_i, %Loop]
    %next_i = add i32 1, %i
    call i32 (ptr, ...) @printf(ptr @outf, i32 %i)
    
    %done = icmp sgt i32 %i, 9
    br i1 %done, label %End, label %Loop

  End:
    ret i32 0
}

;define i32 @main() {
;  Entry:
;    br label %Loop

;  Loop:
;    %input = call i32 @getInt()
;    %out = call i32 @op(i32 %input)
;    call i32 (ptr, ...) @printf(ptr @outf, i32 %out)
    
;    %done = icmp sgt i32 %out, 10
;    br i1 %done, label %End, label %Loop

;  End:
;    ret i32 0
;}
