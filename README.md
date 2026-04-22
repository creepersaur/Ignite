<div align="center">

![Ignite Logo](assets/ignite_logo.png=0.5x)

</div>

Bytecode compiled dynamically typed programming language inspired by Rust, C#, python, etc.
(It's a rewrite of [Cranberry](https://creepersaur.github.io/CranberryDocs) in Rust and compiled.)

```js
// variable declaration
let x = 10

println($"The value of x is {x}")
```

```rs
// function declaration
fn hello() {
	for i in 0..10 { // for loop
		println(i)
	}
}
```

```js
// Standard Library `Std`
let input = Std::IO::read_line("say something: ")
println("You said:", input)
```
