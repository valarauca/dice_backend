/*

mod consts;
pub use self::consts::{Dice3, Dice3Iter, Dice6, Dice6Iter};
mod datum;
pub use self::datum::{BoolVec, Datum, IntVec};
mod element;
pub use self::element::Element;
mod lambda;
pub use self::lambda::{
    coalesce, const_bool, const_int, count, d3, d6, filter, from_op, join, len, max, min, sum,
    Chain, Coalesce, CoalesceChain, CoalesceCombinator, Combinator, Init, Iter, LambdaKind,
};
mod report;
pub use self::report::Report;
mod coll;
pub use self::coll::build_report;
//mod math;

/// create report directly converts source code into a report.
pub fn create_report(source: &str) -> Result<Report, String> {
    use super::cfgbuilder::ExpressionCollection;
    use super::inliner::InlinedCollection;
    use super::namespace::Namespace;
    use super::parser_output::AbstractSyntaxTree;

    let ast = AbstractSyntaxTree::parse(source)?;
    let namespace = Namespace::new(&ast)?;
    let cfgcoll = ExpressionCollection::new(&namespace);
    let inlinecoll = InlinedCollection::new(&cfgcoll);
    Ok(build_report(&inlinecoll))
}

/*
#[test]
fn test_removing_min_dice_roll() {
    let dut = r#"
const dice: vec<int> = roll_d6(3);
analyze (sum(dice) - min(dice));
"#;
    let report = create_report(dut).unwrap();
    let output = report.equal(&[
        (Datum::from(2), 1.0 / 216.0),
        (Datum::from(3), 3.0 / 216.0),
        (Datum::from(4), 7.0 / 216.0),
        (Datum::from(5), 12.0 / 216.0),
        (Datum::from(6), 18.0 / 216.0),
        (Datum::from(7), 27.0 / 216.0),
        (Datum::from(8), 34.0 / 216.0),
        (Datum::from(9), 36.0 / 216.0),
        (Datum::from(10), 34.0 / 216.0),
        (Datum::from(11), 27.0 / 216.0),
        (Datum::from(12), 16.0 / 216.0),
    ]);
}

#[test]
fn test_removing_max_dice_roll() {
    let dut = r#"
const dice: vec<int> = roll_d6(3);
analyze (sum(dice) - max(dice));
"#;

    let report = create_report(dut).unwrap();
    let output = report.equal(&[
        (Datum::from(2), 16.0 / 216.0),
        (Datum::from(3), 27.0 / 216.0),
        (Datum::from(4), 34.0 / 216.0),
        (Datum::from(5), 36.0 / 216.0),
        (Datum::from(6), 34.0 / 216.0),
        (Datum::from(7), 27.0 / 216.0),
        (Datum::from(8), 16.0 / 216.0),
        (Datum::from(9), 12.0 / 216.0),
        (Datum::from(10), 7.0 / 216.0),
        (Datum::from(11), 3.0 / 216.0),
        (Datum::from(12), 1.0 / 216.0),
    ]);
}

#[test]
fn test_1d6() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::inliner::{InlinedCollection, InlinedExpression};
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
        (Datum::from(1), 1.0 / 6.0),
        (Datum::from(2), 1.0 / 6.0),
        (Datum::from(3), 1.0 / 6.0),
        (Datum::from(4), 1.0 / 6.0),
        (Datum::from(5), 1.0 / 6.0),
        (Datum::from(6), 1.0 / 6.0),
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
        (Datum::from(2), 1.0 / 36.0),
        (Datum::from(3), 2.0 / 36.0),
        (Datum::from(4), 3.0 / 36.0),
        (Datum::from(5), 4.0 / 36.0),
        (Datum::from(6), 5.0 / 36.0),
        (Datum::from(7), 6.0 / 36.0),
        (Datum::from(8), 5.0 / 36.0),
        (Datum::from(9), 4.0 / 36.0),
        (Datum::from(10), 3.0 / 36.0),
        (Datum::from(11), 2.0 / 36.0),
        (Datum::from(12), 1.0 / 36.0),
    ]);
    match output {
        Ok(()) => {}
        Err(e) => panic!("{:?}", e),
    };
}

#[test]
fn test_2d6_join() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::inliner::{InlinedCollection, InlinedExpression};
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#" analyze sum(join(roll_d6(1),roll_d6(1))); "#;

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

    /*
     * Validate the inline collection
     *
     */
    {
        let sum_expr = match inlinecoll
            .get_return()
            .into_iter()
            .flat_map(|expr| inlinecoll.get_expr(&expr))
            .next()
            .unwrap()
        {
            InlinedExpression::Sum(arg) => arg,
            anything_else => panic!("{:?}", anything_else),
        };
        match inlinecoll.get_expr(&sum_expr).unwrap() {
            InlinedExpression::Join(a, b) => {
                assert_eq!(a, b);
                match inlinecoll.get_expr(&a).unwrap() {
                    InlinedExpression::D6(_) => {}
                    anything_else => panic!("{:?}", anything_else),
                };
            }
            anything_else => panic!("{:?}", anything_else),
        };
    }

    let report = build_report(&inlinecoll);

    // check if our report is correct
    let output = report.equal(&[
        (Datum::from(2), 1.0 / 36.0),
        (Datum::from(3), 2.0 / 36.0),
        (Datum::from(4), 3.0 / 36.0),
        (Datum::from(5), 4.0 / 36.0),
        (Datum::from(6), 5.0 / 36.0),
        (Datum::from(7), 6.0 / 36.0),
        (Datum::from(8), 5.0 / 36.0),
        (Datum::from(9), 4.0 / 36.0),
        (Datum::from(10), 3.0 / 36.0),
        (Datum::from(11), 2.0 / 36.0),
        (Datum::from(12), 1.0 / 36.0),
    ]);
    match output {
        Ok(()) => {}
        Err(e) => panic!("{:?}", e),
    };
}
*/

*/
