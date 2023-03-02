#![allow(clippy::never_loop)]
#![allow(clippy::unit_arg)]

trait Wacky {
    const CHAR: char;

    fn wack(&self) {
        println!("{}", Self::CHAR);
    }
}

impl Wacky for () {
    const CHAR: char = 'a';
}

impl Wacky for bool {
    const CHAR: char = 'b';
}

fn main() {
    {
        // This block evaluates to () .
        // Run rustfmt to break the formatting of the loop.
        // This block will then evaluate to a bool.
        // Why? See https://github.com/rust-lang/rustfmt/issues/5377
        loop {
            break false
        };
    }
    .wack();
}
