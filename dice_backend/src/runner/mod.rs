mod coll;
pub use self::coll::InlinedCollection;
mod expr;
pub use self::expr::InlinedExpression;

#[test]
fn test_trivial_program() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#"
const A: int = 5;
const B: int = 5;
const C: int = 10;
analyze ((A + B) + C);
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
     * Trivial Test of constant propigation
     *
     */

    // start inspecting the program
    let return_id = coll.get_return().unwrap();
    match coll.get_expr(&return_id) {
        Option::Some(InlinedExpression::ConstantInt(20)) => {
        },
        anything_else => {
            panic!("expected a constant value, found '{:?}'", anything_else);
        }
    };
}

#[test]
fn test_inlining_proper() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#"
fn lol_add(x: int, y: int) -> int {
    return ((x + y) + 10);
}

analyze lol_add(5,lol_add(5,5));
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
     * This program should reduce to a single statement
     *
     * Just a constant value
     *
     */

    let return_id = coll.get_return().unwrap();
    match coll.get_expr(&return_id) {
        Option::Some(InlinedExpression::ConstantInt(35)) => {
        },
        anything_else => {
            panic!("expected a constant value, found '{:?}'");
        }
    };
}


#[test]
fn complex_inling() {
    use super::cfgbuilder::{ExpressionCollection, HashedExpression};
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, Operation, TypeData};

    let trivial_program = r#"

const FIVE: int = 5;
const TEN: int = 10;

fn lol_sub(x: int) -> int {
    return (x - FIVE);
}

fn lol_add(x: int, y: int) -> int {
    return ( ( lol_sub(x) + lol_sub(y) ) + TEN);
}

analyze lol_add(4,0);
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
     * This program should reduce to a single statement
     *
     * Just a constant value
     *
     */

    let return_id = coll.get_return().unwrap();
    match coll.get_expr(&return_id) {
        Option::Some(InlinedExpression::ConstantInt(4)) => {
            // TODO: fix this test
        },
        anything_else => {
            panic!("expected a constant value, found '{:?}'", anything_else);
        }
    };
}
