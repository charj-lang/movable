## Common Part

### Types System Mapping

Thinking in data DSL

### Variable declarations and blocks

### Jasmin 

```smali
.class public HelloWorld
.super java/lang/Object

;
; standard initializer (calls java.lang.Object's initializer)
;
.method public <init>()V
    aload_0
    invokenonvirtual java/lang/Object/<init>()V
    return
.end method

;
; main() - prints out Hello World
;
.method public static main([Ljava/lang/String;)V
    .limit stack 2   ; up to two items can be pushed

    ; push System.out onto the stack
    getstatic java/lang/System/out Ljava/io/PrintStream;

    ; push a string onto the stack
    ldc "Hello World!"

    ; call the PrintStream.println() method.
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V

    ; done
    return
.end method
```

### Smali

```smali
.class public Lfiredt;    #定义一个firedt类
.super Ljava/lang/Object;    #继承object类
.source "firedt.java"       #由firedt.java编译来 
# direct methods
.method public constructor <init>()V     
    .registers 1             #注册一个寄存器
    .prologue   #代码开始
    .line 1            #第一行
    invoke-direct {p0}, Ljava/lang/Object;-><init>()V 
    return-void
.end method
.method public static main([Ljava/lang/String;)V   #说明一个main的静态方法，类型为void
    .registers 3   
    .prologue
    .line 3
    sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;   #获取变量对象保存到v0
    const-string v1, "hello world"   #赋值一个字符串v1=hello world
    invoke-virtual {v0, v1}, Ljava/io/PrintStream;->println(Ljava/lang/String;)V   #调用printstream的println方法把v0赋值v1
    .line 4   
    return-void
.end method  #方法结束，一个method对应一个end method
```

