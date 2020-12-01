```c
#include <stdio.h>
int main() {
   printf("Hello, World!");
   return 0;
}
```

```antlr
// ruleset C.movable

## DEFAULT Rule

define default$identifier -> [a-zA-Z_]
keywords$DOT -> '.'
parameters -> @type identifier (',' @type identifier)?

//
spec {

}

## Type

type$int -> int
type$number -> number

## delimiters

symbol$raw_string -> '"""'
symbol$end_line -> ';'
symbol$DOT = '.'

import -> '#' 'include' template$source
template$source#1 -> '<' identifier '>'
template$source#2 -> '<' identifier @DOT 'h' '>'

function:
  @type @name '(' @parameters ')' '{'
    @block
  '}'

block: @expr+

expr
  : @call '(' ')'  @end_line
  | @return number @end_line
```

```
impl c.movable

impl$import -> 'punctuation.definition.directive.c' 'meta.preprocessor.include.c'
impl$template.source -> 'punctuation.definition.string.begin.c' identifier 'punctuation.definition.string.end.c'

```