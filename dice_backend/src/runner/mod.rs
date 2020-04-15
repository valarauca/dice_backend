mod coll;
pub use self::coll::InlinedCollection;
mod expr;
pub use self::expr::InlinedExpression;

#[test]
fn test_nontrivial_program() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#"
const MAX:int = 6;
const MIN:int = 1;

fn reroll_1(dice: vec<int>) -> vec<int> {
    let max: int = MAX;
    return reroll(max, MIN, (1 == dice), dice);
}

analyze reroll_1(roll(MAX,MIN,10));
"#;

    let ast = match AbstractSyntaxTree::parse(trivial_program) {
        Ok(ast) => ast,
        Err(e) => panic!("ast error: {:?}", e),
    };
    let namespace = match Namespace::new(&ast) {
        Ok(namespace) => namespace,
        Err(e) => panic!("namespace error: {:?}", e),
    };
    let cfgcoll = ExpressionCollection::new(&namespace);
    let coll = InlinedCollection::new(&cfgcoll);

    /*
     * Let us start the program analysis.
     * Our optimizing compiler should be
     * fully deterministic so this test
     * should always be valid
     *
     */

    // start inspecting the program
    let return_id = coll.get_return().unwrap();
    // our return expression should be erased, and rolled all the way
    // to our STDLIB function.
    let return_expr_args = match coll.get_expr(&return_id) {
        Option::Some(InlinedExpression::StdLibFunc("reroll", ref args)) => {
            assert_eq!(args.len(), 4);
            args.clone()
        }
        anything_else => panic!("expected reroll found {:?}", anything_else),
    };
    // the "max" local which is our first argument should be erased
    // down to the constant, so let us test that
    match coll.get_expr(&return_expr_args[0]) {
        Option::Some(InlinedExpression::ConstantInt(6)) => {}
        anything_else => panic!("expected a constant number, found: {:?}", anything_else),
    };
    // the MIN is a constant which should just always exist
    match coll.get_expr(&return_expr_args[1]) {
        Option::Some(InlinedExpression::ConstantInt(1)) => {}
        anything_else => panic!("expected a constant number, found: {:?}", anything_else),
    };
    // the 3^rd argument should be a filter ooperation
    // let us resolve that
    let (left, right) = match coll.get_expr(&return_expr_args[2]) {
        Option::Some(InlinedExpression::Operation(ref left, Operation::Equal, ref right, TypeData::CollectionOfBool)) => {
            (left.clone(), right.clone())
        }
        anything_else => panic!(
            "expected a streaming filter operation, found: {:?}",
            anything_else
        ),
    };
    // the left expression _should_ be identical to our second
    // argument (index: 1) as they are both constants of 1
    // we can assert equality like this.
    // this is because the
    // `InlinedExpression::Constant(Literal::Number(1))
    // have an identical hash and should resolve to each other.
    assert_eq!(left, return_expr_args[1].clone());
    // this properity also allows us to assert that the last
    // argument in our function is idntical to `right`
    // one should note both of these are the `dice` variable.
    assert_eq!(right, return_expr_args[3].clone());
    // now our argument should resolve to the stdlib function
    let initial_args = match coll.get_expr(&return_expr_args[3]) {
        Option::Some(InlinedExpression::StdLibFunc("roll", ref args)) => {
            assert_eq!(args.len(), 3);
            args.clone()
        }
        anything_else => panic!("expected stdlib function, found {:?}", anything_else),
    };
    // our max & min is already on the stack
    // so let us assert those are identical right now
    assert_eq!(initial_args[0].clone(), return_expr_args[0].clone());
    assert_eq!(left, initial_args[1].clone());
    // last step, ensure that value `10` exists.
    match coll.get_expr(&initial_args[2]) {
        Option::Some(InlinedExpression::ConstantInt(10)) => {
            // yay!
        }
        anything_else => panic!("expected constant of 10, found: {:?}", anything_else),
    };
    // :) full program analysis complete
}
