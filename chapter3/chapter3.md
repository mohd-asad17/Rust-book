# **Common Programming Concepts**

This chapter covers **core concepts** that are similar in every programming langauge, explained in **Rust syntax and semantics**. Even though the concepts presented in this chapter are not unique to Rust, but we'll discuss them in context of Rust. Understanding how Rust implements them helps to write **Safe, efficient, and reliable code.** This chapter covers 

---
## Keywords
 
 The Rust language has a set of **reserved keywords** - these cannot be use as a names of variables or functions.

 Example of such keywords include:

 ```
 fn, let, mut, const, if, else, loop, while, match, for, pub , ref, break
 ```
 A few reserved keywords have no current functionality associated with them but might be added to the Rust in the future.

 You can find a list of the keywords [Appendix A](https://doc.rust-lang.org/book/appendix-01-keywords.html).

 ## Variables and Mutability

 * Variables are the named storage location in the computer which can stores a value which can be changed or varies.
 * In Rust, variables are **immutable** by default -  means once a value is bound to a name, you cannot change it and this is a feature, not a restriction in rust.

* Rust encourages **immutablity** to ensure **safety and concurrency**.

However, when needed , you can make a variable mutable usign a keyword `mut`.

- To illustrate this, generate a new project using 
    `cargo new variables`
     - change the directory `cd variables`

### Immutable Example     

 Filename:- `src/main.rs`

```
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // Error: trying to reassign an immutable variable
    println!("The value of x is: {x}");
}
```    
Run the program

```
cargo run 
```

**Output:-**

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

**Explanation of the error**:- Rust is telling you that you cannot assign the value of `x` twice because it immutable variable you can use keyword `mut` before a name of the variable to remove this bugs.

This compile-time safety prevents potential bugs where one part of the code assumes the value will never change, but another part change it unexpectedly.

### Mutable Example

Filename: `src/main.rs`

```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // now it is correct to assign value twice
    println!("The value of x is: {x}");
}
```
**Output**:-

```
The value of x is: 5
The value of x is: 6
```

**Tips**:- Use `mut` carfully. Prefer Immutable variable unless you need to explicitly **mutate** -- it improves safety and readability.
---




