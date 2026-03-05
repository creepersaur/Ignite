# Ignite

Bytecode compiled dynamically typed programming language inspired by Rust, C#, python, etc.
(It's a rewrite of [Cranberry](https://creepersaur.github.io/CranberryDocs) in Rust and compiled.)

```rs
let x = 10

println($"The value of x is {x}")
```

```rs
fn hello() {
	for i in 0..10 {
		println(i)
	}
}
```

```rs
let input = Std::IO::read_line("say something: ")
println("You said:", input)
```