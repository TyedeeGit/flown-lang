# Summary
The Flown Programming Language is my personal experiment in creating a programming language. Flown is theory-first, heavily inspired by existing proof assistants such as Lean4, the borrow/ownership system from Rust, and linear logic.

# Syntax
Flown's syntax is postfix, and uses a special syntax for definitions.
Function application in most languages is usually `f(x, y, z)` or `f x y z`. In Flown, it is
```
z y x f
```
As a consequence, it is right-associative:
```
z y x f = z (y (x f))
```
Use parenthesis to nest:
```
(x g) f // usually `f(g(x))` or `f (g x)`
```
If the *leftmost argument* is nested, you can use this syntax:
```
x g |> f = (x g) f
```
However, for nesting in other arguments, you still need paranthesis:
```
x h |> (y g) f = (x h) (y g) f
```
Piping is left-associative:
```
x h |> g |> f = (x h |> g) |> f = ((x h) g) f
```
