struct Undebuggable(fn());

#[derive(derive_more::Debug)]
enum Expr<A> {
    #[debug("Atom(?)")]
    Atom(A),
    If(Branches<A>),
}

#[derive(derive_more::Debug)]
struct Branches<A> {
    // remove this attribute to cause compile error:
/*```
error[E0275]: overflow evaluating the requirement `Expr<Undebuggable>: Debug`
   --> playground/derive_more_debug_test/src/main.rs:25:5
    |
25  |     dbg!(expr);
    |     ^^^^^^^^^^
    |
    = note: required for `Vec<Expr<Undebuggable>>` to implement `Debug`
    = note: 2 redundant requirements hidden
    = note: required for `&Branches<Undebuggable>` to implement `Debug`
note: required by a bound in `derive_more::core::fmt::rt::Argument::<'_>::new_debug`
   --> /home/nius/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:120:25
    |
120 |     pub fn new_debug<T: Debug>(x: &T) -> Argument<'_> {
    |                         ^^^^^ required by this bound in `Argument::<'_>::new_debug`
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `dbg` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0275`.
error: could not compile `derive_more_debug_test` (bin "derive_more_debug_test") due to 1 previous error    
```*/
    #[debug("[{}]", exprs.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>().join(", "))]
    exprs: Vec<Expr<A>>,
}

fn main() {
    let x = Undebuggable(||{});

    let expr = Branches { exprs: vec![Expr::Atom(x)] };

    dbg!(expr);
}
