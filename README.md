# Movable DSL

![CI](https://github.com/charj-lang/movable/workflows/CI/badge.svg)

Movable (from Movable Type（in Chinese: 活字印刷术）) is a intermedia DSL for description tokens which convert by [Scie](github.com/phodal/scie/).

 - structure organization
 - inheritance
 - structure generate
 - template transpile

## Syntax

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

## HIR DSL for Code Generation (Design)

```
function.block.delimiters "{" "}"
parameters.block.delimiters "(" ")"
parameters. ","
parameters.type.sp ";"
line.block.delimiters ";"

indent "    "
indent.forced

function "main" {
  returns int
  parameters none
  expr `{:?}("hello, world!");`, "print"
  block {
     stmt "if"
     cond "a > b"
     expr `return {:?}`, 0
  }

  expr `return {:?}`, 0
}
```

## Contributors

<a href="https://github.com/charj-lang/movable/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=charj-lang/movable" />
</a>
