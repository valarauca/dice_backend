mod expression;
pub use self::expression::HashedExpression;
mod collection;
pub use self::collection::ExpressionCollection;
mod identifier;
pub use self::identifier::Identifier;
mod stack;
pub use self::stack::CallStack;


#[test]
fn test_non_trivial_program_1() {
    use super::namespace::Namespace;
    use super::parser_output::{AbstractSyntaxTree, Literal, TypeData};

    let trivial_program = r#"
const MAX:int = 6;
const MIN:int = 1;

fn reroll_1(dice: vec<int>) -> vec<int> {
    let max: int = MAX;
    let remaining: vec<int> = filter( (1 != dice), dice);
    return join(remaining, roll(max,MIN,(len(dice) - len(remaining))));
}

analyze sum(roll(MAX,MIN,10));
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

    // check that our analyze statement points to a function
    let (id, args, kind) = match cfgcoll.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };
    // assert the function returing analyze has 1 arg
    assert!(args.len() == 1);
    // assert the function is returnning a `vec<int>`
    assert!(kind == TypeData::CollectionOfInt);

    // walk down our function to the next
    let (id, args, kind) = match cfgcoll.get_expr(None, &args[0]) {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };
    // assert we have 3 args
    assert_eq!(args.len(), 3);
    // assert we are truning a collection of ints
    assert!(kind == TypeData::CollectionOfInt);

    /*
     * Inspect the arguments of roll(MAX, MIN, 10)
     *
     */
    match cfgcoll.get_expr(None, &args[0]) {
        Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => {
            match cfgcoll.get_variable(id) {
                Option::Some(HashedExpression::ConstantValue(
                    Literal::Number(6),
                    TypeData::Int,
                )) => {}
                anything_else => panic!(
                    "Expected literal constant of value 6 int. Found: {:?}",
                    anything_else
                ),
            };
        }
        anything_else => panic!("Expected constant value. Found: {:?}", anything_else),
    };
    match cfgcoll.get_expr(None, &args[1]) {
        Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => {
            match cfgcoll.get_variable(id) {
                Option::Some(HashedExpression::ConstantValue(
                    Literal::Number(1),
                    TypeData::Int,
                )) => {}
                anything_else => panic!(
                    "Expected literal constant of value 1 int. Found: {:?}",
                    anything_else
                ),
            };
        }
        anything_else => panic!("Expected constant value. Found: {:?}", anything_else),
    };
    match cfgcoll.get_expr(None, &args[2]) {
        Option::Some(HashedExpression::ConstantValue(Literal::Number(10), TypeData::Int)) => {}
        anything_else => panic!("Expected constant value. Found: {:?}", anything_else),
    };

    /*
     * Work our way through the recursiveness of this structure
     *
     */
    let return_func_id = match cfgcoll.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, TypeData::CollectionOfInt)) => {
            // this should be our `analyze reroll_1(roll(MAX, MIN, 10));
            // statement
            assert!(args.len() == 1);
            id.clone()
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };
    // get the context of `reroll_1`
    let return_function_body = cfgcoll.get_function_context(&return_func_id).unwrap();
    // inspect the `return reroll(max, MIN, (1 == dice), dice);
    let returns_functions_args = match return_function_body.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, TypeData::CollectionOfInt)) => {
            assert!(cfgcoll.is_function_stdlib(id));
            assert!(args.len() == 4);
            args.clone()
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };
    // look a the `max` variable
    match cfgcoll.get_expr(Some(return_func_id.clone()), &returns_functions_args[0]) {
        Option::Some(HashedExpression::Var(ref id, TypeData::Int)) => {
            match cfgcoll.get_variable(id) {
                Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => {
                    match cfgcoll.get_variable(id) {
                        Option::Some(HashedExpression::ConstantValue(
                            Literal::Number(6),
                            TypeData::Int,
                        )) => {}
                        anything_else => {
                            panic!("Expected a constant value of 6, found: {:?}", anything_else)
                        }
                    }
                }
                anything_else => panic!("Expected external constant, Found: {:?}", anything_else),
            };
        }
        anything_else => panic!("expected a variable. Found: {:?}", anything_else),
    };
    // look at that `MIN` variable, we should resolve a value of 1
    match cfgcoll.get_expr(Some(return_func_id.clone()), &returns_functions_args[1]) {
        Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => {
            match cfgcoll.get_variable(id) {
                Option::Some(HashedExpression::ConstantValue(
                    Literal::Number(1),
                    TypeData::Int,
                )) => {}
                anything_else => panic!("Expected literal value, found: {:?}", anything_else),
            }
        }
        anything_else => panic!("expected external constant. Found: {:?}", anything_else),
    };
}
