source_filename = "functions.intu"
target triple = "x86_64-unknown-linux-gnu"

declare i64 @f.5(i64 %0)


declare i64 @f.8(i64 %0, i64 %1)


declare i64 @g.12(i64 %0)


declare i64 @f.11(i64 %0)


define i64 @program() {
entry:
  br label %tailcall$23
tailcall$23:
  %tmp$0 = bitcast ptr %lambda$$0 to ptr
  %tmp$1 = alloca { ptr }, align 8
  %tmp$2 = getelementptr inbounds { ptr }, ptr %tmp$1, i32 0, i32 0
  store ptr %tmp$0, ptr %tmp$2, align 8
  %tmp$3 = bitcast ptr %lambda$$2 to ptr
  %tmp$4 = alloca { ptr }, align 8
  %tmp$5 = getelementptr inbounds { ptr }, ptr %tmp$4, i32 0, i32 0
  store ptr %tmp$3, ptr %tmp$5, align 8
  %tmp$6 = bitcast ptr %lambda$$4 to ptr
  %tmp$7 = alloca { ptr }, align 8
  %tmp$8 = getelementptr inbounds { ptr }, ptr %tmp$7, i32 0, i32 0
  store ptr %tmp$6, ptr %tmp$8, align 8
  %tmp$9 = bitcast ptr %lambda$$6 to ptr
  %tmp$10 = alloca { ptr }, align 8
  %tmp$11 = getelementptr inbounds { ptr }, ptr %tmp$10, i32 0, i32 0
  store ptr %tmp$9, ptr %tmp$11, align 8
  %tmp$12 = bitcast ptr %lambda$$8 to ptr
  %tmp$13 = alloca { ptr }, align 8
  %tmp$14 = getelementptr inbounds { ptr }, ptr %tmp$13, i32 0, i32 0
  store ptr %tmp$12, ptr %tmp$14, align 8
  %tmp$15 = bitcast ptr %lambda$$10 to ptr
  %tmp$16 = alloca { ptr }, align 8
  %tmp$17 = getelementptr inbounds { ptr }, ptr %tmp$16, i32 0, i32 0
  store ptr %tmp$15, ptr %tmp$17, align 8
  %tmp$18 = bitcast ptr %lambda$$12 to ptr
  %tmp$19 = alloca { ptr }, align 8
  %tmp$20 = getelementptr inbounds { ptr }, ptr %tmp$19, i32 0, i32 0
  store ptr %tmp$18, ptr %tmp$20, align 8
  %tmp$21 = call ptr ptr %tmp$13(ptr %tmp$19, ptr %tmp$16)
  %tmp$22 = tail call ptr ptr %tmp$7(i64 %tmp$21, i64 5)
  ret i64 %tmp$22
}

define i64 @lambda$$0(ptr %env$$1, i64 %x.0) {
entry:
  br label %ret$24
ret$24:
  ret i64 %x.0
}

define i64 @lambda$$2(ptr %env$$3, i64 %x.2, i64 %y.3) {
entry:
  br label %ret$26
ret$26:
  %tmp$25 = add i64 %x.2, %y.3
  ret i64 %tmp$25
}

define i64 @lambda$$4(ptr %env$$5, ptr %f.5, i64 %x.6) {
entry:
  br label %tailcall$28
tailcall$28:
  %tmp$27 = tail call ptr @f.5(i64 %x.6)
  ret i64 %tmp$27
}

define ptr @lambda$$6(ptr %env$$7, ptr %f.8, i64 %x.9) {
entry:
  br label %tailcall$30
tailcall$30:
  %tmp$29 = tail call ptr @f.8(i64 %x.9)
  ret i64 %tmp$29
}

define i64 @lambda$$8(ptr %env$$9, ptr %f.11, ptr %g.12, i64 %x.13) {
entry:
  br label %tailcall$33
tailcall$33:
  %tmp$31 = call ptr @g.12(i64 %x.13)
  %tmp$32 = tail call ptr @f.11(i64 %tmp$31)
  ret i64 %tmp$32
}

define i64 @lambda$$10(ptr %env$$11, i64 %x.15) {
entry:
  br label %ret$35
ret$35:
  %tmp$34 = mul i64 %x.15, 2
  ret i64 %tmp$34
}

define i64 @lambda$$12(ptr %env$$13, i64 %x.17) {
entry:
  br label %ret$37
ret$37:
  %tmp$36 = add i64 %x.17, 1
  ret i64 %tmp$36
}

define i32 @main() {
entry:
  %tmp$38 = call ptr @program()
  ret i32 0
}
