```c
#include <stdio.h>
int main() {
   printf("Hello, World!");
   return 0;
}
```

// default$identifier -> [a-zA-Z_]
// keywords$DOT -> '.'

// symbol$import -> '#' 'include'
// template$source#1 -> '<' identifier '>'
// template$source#2 -> '<' identifier DOT 'h' '>'
// parameters -> @type identifier (',' @type identifier)?

// type$int -> int
// type$number -> number

// symbol$raw_string -> '"""'
// symbol$end_line -> ';'

function:
  @type @name '(' @parameters ')' '{'
    @block
  '}'

block: @expr+

expr
  : @call '(' ')'  @end_line
  | @return number @end_line
