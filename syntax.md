# Function Application
Function application needs no paranthesis
```
f x y z
```
As a consequence, it is left-associative:
```
f x y z = ((f x) y) z
```
Use parentheses to nest:
```
f (g x)
```
If the *rightmost argument* is nested, you can use this syntax:
```
f <| g x = f (g x)
```
However, for nesting in other arguments, you still need parentheses:
```
f (g x) <| h x = f (g x) (h x)
```
Piping is right-associative:
```
f <| g <| h x = f <| (g <| h x) = f (g (h x))
```

# Types
In Flown, to say that a value is of a type, we use this syntax:
```
the_type: the_value
```
Notice that, like in C-style languages, the type goes first, but we have a colon in between, making it less ambiguous.
You can also use this within expressions to elaborate their type:
```
f (int32: x) // instead of `f x`
```

## Algebraic Types
These are some ways to create algebraic types:
```
(x_type: .x, y_type: .y) // product/struct
[x_type: .x, y_type: .y] // coproduct
(x_type: .x | y_type: .y) // cosum
[x_type: .x | y_type: .y] // sum/enum
(x_type: x) -> y_type // function
```
You can also add constraint clauses:
```
(x - y) // x serial y
[x - y] // x coserial y
(x < y) // x before y
[x < y] // x cobefore y

```

## Share, Borrow, and Indirect Types
Given a type `t` and region(which can be omitted) `a`:
```
'a&t // share
'a!t // borrow
'a^t // indirect
```

# Definitions
Fundamentally, all definitions take the form:
```
denoter := denotee
```
The syntax makes no distinction between denoter and denotee expressions. In other languages, the terms "lvalue" and "rvalue" roughly correspond to denoters and denotees respectively.

## Exported, Incomplete, and Completion Symbols
To make the denoter visible outside of the current scope, use `export`:
```
export denoter := denotee;
```
You can also control where it is visible with a `to ...` clause:
```
to path::to::scope::one, path::to::scope:two, ...
export denoter := denotee;
```

To mark a symbol as incomplete:
```
incomplete denoter := denotee;
```

To complete a symbol:
```
complete denoter := denotee
```

# Lambda Expressions
Lambda expressions take the following form:
```
(x) (y) ... => body
```
where `x`, `y`, etc are denoter expressions. This is in fact the only way to create named functions:
```
wrapping_add_mul := (int32: a) (int32: b) (int32: x) => int32:
    int32::wrapping_add b <| int32::wrapping_mul a x;
```
