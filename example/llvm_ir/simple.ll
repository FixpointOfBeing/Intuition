source_filename = ""

define i1 @program() {
entry:
  br label %ret$2
ret$2:
  %tmp$0 = icmp slt i64 1, 3
  %tmp$1 = and i1 true, %tmp$0
  ret i1 %tmp$1
}

define i32 @main() {
entry:
  %tmp$3 = call ptr @program()
  ret i32 0
}
