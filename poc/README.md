# Design

- base.tg 包含了基础的公共表达式，如 `a = a + 1`，或者 `a++` 等
- c.syntax.tg 包含了使用 Typography DSL 描述的 C 语言语法树
- c.impl.tg 包含了 C 语言模板 - 映射
- c.builtin.tg 对于 builtin 函数的映射

## 基本原则

### 提取公共语法



```c
#include <stdio.h>
int main() {
   printf("Hello, World!");
   return 0;
}
```
