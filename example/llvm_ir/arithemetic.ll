source_filename = "arithmetic.intu"
target triple = "x86_64-pc-linux-gnu"

define i64 @program() {
entry:
  br label %ret$8
ret$8:
  %tmp$0 = add i64 10, 3
  %tmp$1 = sub i64 10, 3
  %tmp$2 = mul i64 10, 3
  %tmp$3 = sdiv i64 10, 3
  %tmp$4 = mul i64 10, 3
  %tmp$5 = add i64 %tmp$4, 2
  %tmp$6 = add i64 10, 3
  %tmp$7 = mul i64 %tmp$6, 2
  ret i64 %tmp$7
}

define i32 @main() {
entry:
  %tmp$9 = call ptr @program()
  ret i32 0
}
