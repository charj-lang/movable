# Movable

![CI](https://github.com/charj-lang/movable/workflows/CI/badge.svg)

Movable (from Movable Type（in Chinese: 活字印刷术）) is a intermedia DSL for description tokens which convert by [Scie](https://github.com/charj-lang/scie).

- structure organization
- inheritance
- structure generate
- template transpile

## Typography

Typography is a DSL-base parser for build common AST. features:

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
```

## Movable

Movable is a DSL for save convert DSL to common HIR or MIR.

- Tier 1. HIR, Java, JavaScript, Golang
- Tier 2. MIR, C/C++, Rust

### HIR

Todo, thinking in based Python Bytecode.

### MIR or Sir

todo: thinking based on [https://github.com/vnmakarov/mir](https://github.com/vnmakarov/mir)

## Modernize

Modernization is working for convert Movable generated code to Languages.

## Contributors

<a href="https://github.com/charj-lang/movable/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=charj-lang/movable" />
</a>
