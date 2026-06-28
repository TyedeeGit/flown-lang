# Summary
The Flown Programming Language is my personal experiment in creating a programming language. Flown is theory-first, heavily inspired by existing proof assistants such as Lean4, the borrow/ownership system from Rust, and linear logic.

# Syntax
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
