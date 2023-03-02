# Wacky rustfmt

Mara Bos posted an interesting challenge a while ago ([Link](https://twitter.com/m_ou_se/status/1440660680248467479)):

> Make a rust program that outputs 'a', but after running 'cargo fmt' outputs 'b' instead.

At the time of writing this code (2023-03-02) the code in `src/main.rs` does exactly that.

```
% cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/wacky-rustfmt`
a
maik@PC-MAIK (~/wacky-rustfmt)
% cargo fmt
maik@PC-MAIK (~/wacky-rustfmt)
% cargo r
   Compiling wacky-rustfmt v0.1.0 (/home/maik/wacky-rustfmt)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/wacky-rustfmt`
b
```