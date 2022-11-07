# Scarlett Lang

This project is a tiny programming language written in Rust.

## Description

Scarlet is a passion project I'm making for practice and fun while I have time between classes/work.

This project will not be the next big language, obviously. It's just something I've built out of joy and interest. The language is high-level, expressive and dynamically typed.

Example: 
```
fun fib(n){
    if n <= 1{
        n
    }else{
        fib(n-1) + fib(n-2)
    }
}

let n = 9;

let fib_9 = fib(n);

println(fib_9); # 34
```

## How to run

### Build

```
git clone git@github.com:breazzzy/scarlet_lang.git
cd scarlet_lang
cargo build --release
```
### Run.
#### From Binary
```
scarlet.exe [filename].scrlt
```
#### Allternativley Cargo can be used to run. This is probably the easier way!
```
cargo run [file_name].scrlt
```

## Authors

Contributor's names and contact info

Ex. Billy Barrese  
ex. [@breazzzy]

## Citations

The book Crafting Interpreters by Robert Nystrom was a significant influence early on. As I've come to understand more concepts, I've slowly veered off it, but it's a really great book for anyone trying to get started with interpreters.


