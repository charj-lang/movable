# Movable DSL

from Movable Type（in Chinese: 活字印刷术）

Movable is a intermedia DSL for description tokens which convert by [Scie](github.com/phodal/scie/).

## Syntax

```
// options for movable config
options {
  name  -> "c"
}

// default tokenizer for extends
define default$tokenizer {
  identifier: [a-zA-Z_];
}

// define tokenizer rules
tokenizer extends default$tokenizer {
   ...
}

// definerule
rule <ruleName> {
    ...
}

// ast for generate code
// todo: for future
ast {
  node parameters {
    parameters parameter*
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

## Contributors

<a href="https://github.com/charj-lang/movable/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=charj-lang/movable" />
</a>
