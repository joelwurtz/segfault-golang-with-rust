# Segfault example with golang signal + rust html5ever

This is the smallest example that i can build to reproduce this segfault

## Compiling rust

You will need `x86_64-unknown-linux-musl` target installed

```
cargo build --lib --target x86_64-unknown-linux-musl
``` 

## Compiling & running go

You will need `musl-gcc` compiler

```
CC=/usr/bin/musl-gcc go run --ldflags "-linkmode external -extldflags '-static'" main.go
```

Doing this will result in a segfault