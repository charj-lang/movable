# 3. template-syntax

Date: 2020-12-15

## Status

2020-12-15 proposed

2020-12-15 deprecated

## Context

### Old design 

symbol match dsl

1. read target language DSL file
    - MVP version, with **yaml**
    - production version, with **DSL**
2. read code tokenizer json
3. convert to struct DSL as MIR???
4. convert to language

```
type {
   int "int"
   uint "unit"
   float "float"
}
```

struct/class/method DSL:

```
.method <return-type> <method-name>()
  // sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;
  // const-string	v1, "Hello World!"
  // invoke-virtual {v0, v1}, Ljava/io/PrintStream;->println(Ljava/lang/String;)V 
  expr `{:?}("hello, world!");`, "print"
  block {
     stmt "if"
     cond "a > b"
     expr `return {:?}`, 0
  }

  expr `return {:?}`, 0
.end method
```

## Decision

Decision here...

## Consequences

Consequences here...
