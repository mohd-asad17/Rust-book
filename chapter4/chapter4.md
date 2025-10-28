# Understanding Ownership

Ownership is Rust's most unique feature which makes Rust stand out from almost all the other langauge. It ensures momery safety without needing garbage collector. So in this chapter we'll understand how ownership works , how it is different from the others, what is the need of ownerships 

Before diving into the ownership, first you need to understand

1. What makes a program safe or unsafe?
2. How memory works (Stack vs Heap)?
3. How ownership, moves, and cloning control memory?

## What is ownership?

> "Ownership is a set of rules that govern how a Rust program manages memory."

- Some language have a garbage collection that regularly looks out as the programs runs there is no-longer-used memory or programmer explicitly allocate and free the memory - means

    * languages like python , java have a garbage  collector which runs automatically and finds the memory which no longer used , and frees it.
    * While in C or C++ the programmers manually allocate the memory and when the no longer used it explicitly free memory.

- While in Rust the memory is managed through a system of ownership having a set of rules that compiler checks - means

    * Rust doesn't use a garbage collector and also it doesn't require to manually manage memory. Instead, Rust use a system of ownership that checked by the compiler which means **compiler knows who owns a memory** and when it not in used, Rust **automatically frees the memory**.

 In more simpler words , assume you have a bag(Variable) which owns some things inside it and you meet your friend and pass this bag to him and now you are free from the bag. Likewise , In Rust a variable named **memory_variable -> "Rust Book"** and also there is another variable **ownership_variable -> memory_variable**  now **"ownership_variable owns the book and memory_variable no longer owns it"**.

 ```
 fn main() {
    let a = String::from("Rust Book");
    let b = a; // ownership moves and a doesn't owns it.
    println!("{}", b); // No error because the ownership moves to b.

    println!("{}", a); // Error: a no longer owns the data.
 }
 ```

 ## The Stack and the Heap

 Both the stack and the heap are the areas of memory you program used while it's running.But, they both work in different ways.

 - The stack stores values in the same order as the gets them in and but while removing the values from the stack it will be in the opposite order while is referred as **Last In First Out(LIFO)**. 

 - Adding data is called *pushing onto the stack* and removing data is called *popping off the stack*.

 - All the data stored on the stack must have a fixed size at compile time - Rust compiler must know how much to be reserve before the programs runs and size cannot change once the program starts running.

- Data with an unknown size at compile time or might change means if the data needs to be a dynamic in size( grow or shrink while the program runs ) then data must be stored on the heap instead.

- When you put data on the heap, Rust asks the memory allocator for some space, find an empty space in the heap and marks it as being in use and gets a pointer (address of the location ), and stores that pointer (of fixed size) on the stack., when you want data you follow the pointer, also use pointer to add new data in the heap.

- Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data, and cleaning up unused data on the heap so that you don't run out of space are all the problems that **ownership** addresses.