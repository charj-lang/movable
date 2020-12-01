```c
#include <stdio.h>
int main() {
   printf("Hello, World!");
   return 0;
}
```

```bnf
define default$identifier -> [a-zA-Z_]
keywords$DOT -> '.'

symbol$import -> '#' 'include' template$source
template$source#1 -> '<' identifier '>'
template$source#2 -> '<' identifier DOT 'h' '>'

# impl$import -> 'punctuation.definition.directive.c' 'meta.preprocessor.include.c'
# impl$template.source -> 'punctuation.definition.string.begin.c' identifier 'punctuation.definition.string.end.c'

parameters -> @type identifier (',' @type identifier)?

type$int -> int
type$number -> number

symbol$raw_string -> '"""'
symbol$end_line -> ';'

function:
  @type @name '(' @parameters ')' '{'
    @block
  '}'

block: @expr+

expr
  : @call '(' ')'  @end_line
  | @return number @end_line
```
