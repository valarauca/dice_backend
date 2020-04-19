mod consts;
pub use self::consts::{Dice3, Dice3Iter, Dice6, Dice6Iter};
mod datum;
pub use self::datum::{BoolVec, Datum, IntVec};
mod element;
pub use self::element::{Element, Rational};
mod lambda;
pub use self::lambda::{
    coalesce, const_bool, const_int, count, d3, d6, filter, join, len, sum, Chain, Coalesce,
    CoalesceChain, CoalesceCombinator, Combinator, Init, Iter, LambdaKind,
};
mod report;
pub use self::report::Report;
mod coll;
pub use self::coll::build_report;

#[test]
fn test_1d6() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::inliner::InlinedCollection;
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#" analyze sum(roll_d6(1)); "#;

    let ast = match AbstractSyntaxTree::parse(trivial_program) {
        Ok(ast) => ast,
        Err(e) => panic!("ast error: {:?}", e),
    };
    let namespace = match Namespace::new(&ast) {
        Ok(namespace) => namespace,
        Err(e) => panic!("namespace error: {:?}", e),
    };
    let cfgcoll = ExpressionCollection::new(&namespace);
    let inlinecoll = InlinedCollection::new(&cfgcoll);
    let report = build_report(&inlinecoll);

    // check if our report is correct
    let output = report.equal(&[
        (Datum::from(1), Rational::new(1, 6)),
        (Datum::from(2), Rational::new(1, 6)),
        (Datum::from(3), Rational::new(1, 6)),
        (Datum::from(4), Rational::new(1, 6)),
        (Datum::from(5), Rational::new(1, 6)),
        (Datum::from(6), Rational::new(1, 6)),
    ]);
    match output {
        Ok(()) => {}
        Err(e) => panic!("{:?}", e),
    };
}

#[test]
fn test_2d6() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::inliner::InlinedCollection;
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#" analyze sum(roll_d6(2)); "#;

    let ast = match AbstractSyntaxTree::parse(trivial_program) {
        Ok(ast) => ast,
        Err(e) => panic!("ast error: {:?}", e),
    };
    let namespace = match Namespace::new(&ast) {
        Ok(namespace) => namespace,
        Err(e) => panic!("namespace error: {:?}", e),
    };
    let cfgcoll = ExpressionCollection::new(&namespace);
    let inlinecoll = InlinedCollection::new(&cfgcoll);
    let report = build_report(&inlinecoll);

    // check if our report is correct
    let output = report.equal(&[
        (Datum::from(2), Rational::new(1, 36)),
        (Datum::from(3), Rational::new(2, 36)),
        (Datum::from(4), Rational::new(3, 36)),
        (Datum::from(5), Rational::new(4, 36)),
        (Datum::from(6), Rational::new(5, 36)),
        (Datum::from(7), Rational::new(6, 36)),
        (Datum::from(8), Rational::new(5, 36)),
        (Datum::from(9), Rational::new(4, 36)),
        (Datum::from(10), Rational::new(3, 36)),
        (Datum::from(11), Rational::new(2, 36)),
        (Datum::from(12), Rational::new(1, 36)),
    ]);
    match output {
        Ok(()) => {}
        Err(e) => panic!("{:?}", e),
    };
}
