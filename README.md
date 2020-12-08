# Movable DSL

![CI](https://github.com/charj-lang/movable/workflows/CI/badge.svg)

Movable (from Movable Type（in Chinese: 活字印刷术）) is a intermedia DSL for description tokens which convert by [Scie](github.com/phodal/scie/).

- structure organization
- inheritance
- structure generate
- template transpile



## Typography

Typography is a DSL-base parser for build AST. features:

- extends
- data struct binding
- template match

### Syntax

```
// options for movable config
options {
  name  -> "c";
}

// default tokenizer for extends
define default$tokenizer {
  identifier: [a-zA-Z_];
}

// define tokenizer rules
tokenizer extends default$tokenizer {
   ...
}

// define rule
rule <ruleName> {
    ...
}

// ast for generate code
// todo: for future
ast {
  node parameters {
    parameters parameter*;
  }
}

// template mapping
typo {
  import {
     template: '#' 'include'
     impl: 'punctuation.definition.directive.c' 'meta.preprocessor.include.c'
  }
}
```

## Movable

Movable is a DSL for save convert DSL

### Syntax

symbol match dsl

```
delimiters {
    struct "{" "}"
    parameter ":"
    parameters "(" parameter ","  ")"
    statement_terminator ";"
    ident "\t"
//  ident 2
}
type {
   int   int
   uint
   float 
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

## Contributors

<a href="https://github.com/charj-lang/movable/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=charj-lang/movable" />
</a>
