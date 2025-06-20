struct Undebuggable(fn());

#[derive(derive_more::Debug)]
enum Expr<A> {
    #[debug("Atom(?)")]
    Atom(A),
    If(Branches<A>),
}

#[derive(derive_more::Debug)]
struct Branches<A> {
    // #[debug("[{}]", exprs.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>().join(", "))]
    exprs: Vec<Expr<A>>,
}

fn main() {
    let x = Undebuggable(||{});

    let expr = Branches { exprs: vec![Expr::Atom(x)] };

    dbg!(expr);
}
