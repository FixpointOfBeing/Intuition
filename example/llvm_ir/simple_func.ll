source_filename = "simple_func.intu"
target triple = "x86_64-unknown-linux-gnu"

define i64 @lambda$$0(ptr %env$$1, i64 %x.0, i64 %y.1) {
entry:
  br label %ret$6
ret$6:
  %tmp$5 = add i64 %x.0, %y.1
  ret i64 %tmp$5
}

define i64 @program() {
entry:
  br label %tailcall$4
tailcall$4:
  %tmp$0 = bitcast ptr %lambda$$0 to ptr
  %tmp$1 = alloca { ptr }, align 8
  %tmp$2 = getelementptr inbounds { ptr }, ptr %tmp$1, i32 0, i32 0
  store ptr %tmp$0, ptr %tmp$2, align 8
  %tmp$3 = tail call i64 %tmp$1(i64 41, i64 1)
  ret i64 %tmp$3
}

define i32 @main() {
entry:
  %tmp$7 = call i64 @program()
  ret i32 0
}
