mod expression;
pub use self::expression::HashedExpression;
mod collection;
pub use self::collection::ExpressionCollection;
mod identifier;
pub use self::identifier::Identifier;
mod stack;
pub use self::stack::CallStack;

#[test]
fn test_complicated_cfg() {
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

    // `analyze lol_add(4,0)` means we should see that function
    let lol_add_id = match cfgcoll.get_return().unwrap() {
        &HashedExpression::Func(ref id, ref args, ref kind) => {
            assert_eq!(args.len(), 2);
            assert_eq!(kind.clone(), TypeData::Int);

            // check the function's arguments
            // `analyze lol_add(4,0)` means this should be `4`
            match cfgcoll.get_expr(None, &args[0]).unwrap() {
                &HashedExpression::ConstantValue(Literal::Number(4), TypeData::Int) => {}
                anything_else => panic!("expected a constant of 4, found {:?}"),
            };
            // check the function's arguments
            // `analyze lol_add(4,0)` means this should be `0`
            match cfgcoll.get_expr(None, &args[1]).unwrap() {
                &HashedExpression::ConstantValue(Literal::Number(0), TypeData::Int) => {}
                anything_else => panic!("expected a constant of 0, found {:?}", anything_else),
            };
            id
        }
        anything_else => panic!("expected a function found:'{:?}'", anything_else),
    };

    // step into the context of `lol_add(x,y)`
    let lol_add_ctx = cfgcoll.get_function_context(lol_add_id).unwrap();
    // get the expression from
    // return ( ( lol_sub(x) + lol_sub(y) ) + TEN);
    match lol_add_ctx.get_return() {
        Option::Some(HashedExpression::Op(ref left, Operation::Add, ref right, TypeData::Int)) => {
            // the right hand side should be 10
            // return ( ( lol_sub(x) + lol_sub(y) ) + TEN);
            // ten is a global var so we need to use the full context
            match lol_add_ctx.get_expr(Some(*lol_add_id), right) {
                Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => {}
                anything_else => panic!("expected a constant, found {:?}", anything_else),
            };
            // the left is another addition
            // ( lol_sub(x) + lol_sub(y) )
            match lol_add_ctx.get_expr(Some(*lol_add_id), left) {
                Option::Some(HashedExpression::Op(
                    ref left,
                    Operation::Add,
                    ref right,
                    TypeData::Int,
                )) => {
                    // left is a call of `lol_sub(x)`
                    let (left_id, left_args) = match lol_add_ctx.get_expr(Some(*lol_add_id), left) {
                        Option::Some(HashedExpression::Func(
                            ref left_id,
                            ref left_args,
                            TypeData::Int,
                        )) => (left_id, left_args),
                        anything_else => panic!("{:?}", anything_else),
                    };
                    // left is a call of `lol_sub(y)`
                    let (right_id, right_args) =
                        match lol_add_ctx.get_expr(Some(*lol_add_id), right) {
                            Option::Some(HashedExpression::Func(
                                ref right_id,
                                ref right_args,
                                TypeData::Int,
                            )) => (right_id, right_args),
                            anything_else => panic!("{:?}", anything_else),
                        };
                    // assert the 2 functions are the same
                    assert!(right_id == left_id);
                    // assert the arguments are different
                    assert!(left_args != right_args);
                }
                anything_else => panic!("{:?}", anything_else),
            };
        }
        anything_else => panic!("expected an op, found {:?}", anything_else),
    };
}
