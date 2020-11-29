# 1. dsl design

Date: 2020-11-28

## Status

2020-11-28 proposed

## Context

C hello, world examples

```bash
- token from 0 to 1 () with scopes C
- token from 0 to 1 (#) with scopes C, meta.preprocessor.include.c, , punctuation.definition.directive.c
- token from 1 to 8 (include) with scopes C, meta.preprocessor.include.c,
- token from 8 to 9 ( ) with scopes C, meta.preprocessor.include.c
- token from 9 to 10 (<) with scopes C, meta.preprocessor.include.c, string.quoted.other.lt-gt.include.c, punctuation.definition.string.begin.c
- token from 10 to 17 (stdio.h) with scopes C, meta.preprocessor.include.c, string.quoted.other.lt-gt.include.c
- token from 17 to 18 (>) with scopes C, meta.preprocessor.include.c, string.quoted.other.lt-gt.include.c, punctuation.definition.string.end.c
- token from 0 to 3 (int) with scopes C, storage.type.built-in.primitive.c
- token from 3 to 4 ( ) with scopes C
- token from 4 to 8 (main) with scopes C, meta.function.c, meta.function.definition.parameters.c, entity.name.function.c
- token from 8 to 9 (() with scopes C, meta.function.c, meta.function.definition.parameters.c, punctuation.section.parameters.begin.bracket.round.c
- token from 9 to 10 ()) with scopes C, meta.function.c, meta.function.definition.parameters.c, punctuation.section.parameters.end.bracket.round.c
- token from 10 to 11 ( ) with scopes C
- token from 11 to 12 ({) with scopes C, meta.block.c, punctuation.section.block.begin.bracket.curly.c
- token from 0 to 6 (printf) with scopes C, meta.block.c, meta.function-call.c, entity.name.function.c
- token from 6 to 7 (() with scopes C, meta.block.c, meta.function-call.c, punctuation.section.arguments.begin.bracket.round.c
- token from 7 to 8 (") with scopes C, meta.block.c, meta.function-call.c, string.quoted.double.c, punctuation.definition.string.begin.c
- token from 8 to 21 (Hello, World!) with scopes C, meta.block.c, meta.function-call.c, string.quoted.double.c
- token from 21 to 22 (") with scopes C, meta.block.c, meta.function-call.c, string.quoted.double.c, punctuation.definition.string.end.c
- token from 22 to 23 ()) with scopes C, meta.block.c, meta.function-call.c, punctuation.section.arguments.end.bracket.round.c
- token from 23 to 24 (;) with scopes C, meta.block.c, punctuation.terminator.statement.c
- token from 0 to 6 (return) with scopes C, meta.block.c, keyword.control.c
- token from 6 to 7 ( ) with scopes C, meta.block.c
- token from 7 to 8 (0) with scopes C, meta.block.c, constant.numeric.decimal.c
- token from 8 to 9 (;) with scopes C, meta.block.c, punctuation.terminator.statement.c
```

```movable
directory, filename
# alias pkg :directory
# alias string.quoted.other.lt-gt.include.c import
# alias storage.type.built-in.primitive.c type
# alias meta.function.c entity.name.function.c function_name
# alias meta.function-call.c entity.name.function.c callee
# alias meta.function-call.c, string.quoted.double.c parameter string
# alias meta.block.c, keyword.control.c keyword
# alias constant.numeric.decimal.c number
```

or

```movable
# alias meta.function-call.c -> string.quoted.double.c parameter string
```

Basic rules

### PoC

**structure by structure**

```
class A {
	constructure() {}
}
```

or

```
struct A {}

impl A {
	new() -> Self {};
}
```


**expr by regex templates**

```
(+ 2 2)
```

```
a = 2 + 2
```


## Decision

Decision here...

## Consequences

Consequences here...
