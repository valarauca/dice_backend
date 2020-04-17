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
            match cfgcoll.get_expr(None,&args[0]).unwrap() {
                &HashedExpression::ConstantValue(Literal::Number(4),TypeData::Int) => { },
                anything_else => panic!("expected a constant of 4, found {:?}"),
            };
            // check the function's arguments
            // `analyze lol_add(4,0)` means this should be `0`
            match cfgcoll.get_expr(None,&args[1]).unwrap() {
                &HashedExpression::ConstantValue(Literal::Number(0),TypeData::Int) => { },
                anything_else => panic!("expected a constant of 0, found {:?}", anything_else),
            };
            id
        },
        anything_else => panic!("expected a function found:'{:?}'", anything_else)
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
                Option::Some(HashedExpression::ExternalConstant(ref id, TypeData::Int)) => { },
                anything_else => panic!("expected a constant, found {:?}", anything_else)
            };
            // the left is another addition
            // ( lol_sub(x) + lol_sub(y) )
            match lol_add_ctx.get_expr(Some(*lol_add_id), left) {
                Option::Some(HashedExpression::Op(ref left, Operation::Add, ref right, TypeData::Int)) => {

                    // left is a call of `lol_sub(x)`
                    let (left_id, left_args) = match lol_add_ctx.get_expr(Some(*lol_add_id), left) {
                        Option::Some(HashedExpression::Func(ref left_id, ref left_args, TypeData::Int)) => {
                            (left_id,left_args)
                        },
                        anything_else => panic!("{:?}", anything_else)
                    };
                    // left is a call of `lol_sub(y)`
                    let (right_id, right_args) = match lol_add_ctx.get_expr(Some(*lol_add_id), right) {
                        Option::Some(HashedExpression::Func(ref right_id, ref right_args, TypeData::Int)) => {
                            (right_id, right_args)
                        },
                        anything_else => panic!("{:?}", anything_else)
                    };
                    // assert the 2 functions are the same
                    assert!(right_id == left_id);
                    // assert the arguments are different
                    assert!(left_args != right_args);
                },
                anything_else => panic!("{:?}", anything_else)
            };
        }
        anything_else => panic!("expected an op, found {:?}", anything_else)
    };
}

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

analyze sum(reroll_1(roll(MAX,MIN,10)));
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

    /*
     * Ensure the `analyze` statement points to `sum`
     *
     */

    let (id, args, kind) = match cfgcoll.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            assert_eq!(cfgcoll.get_function_name(id), Some("sum"));
            assert!(cfgcoll.is_function_stdlib(id));
            assert_eq!(args.len(), 1);
            assert!(kind.clone() == TypeData::Int);
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };

    /*
     * Ensure `sum` has 1 argument which is `reroll_1`
     *
     */

    let (id, args, kind) = match cfgcoll.get_expr(None, &args[0]) {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            // returns a collection of int
            assert_eq!(kind.clone(), TypeData::CollectionOfInt);
            // named reroll 1
            assert_eq!(cfgcoll.get_function_name(id), Some("reroll_1"));
            // not in stdlib
            assert!( !cfgcoll.is_function_stdlib(id)); 
            // has 1 argument
            assert_eq!(args.len(), 1);
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };

    /*
     * Look at the `roll` function
     *
     */

    let (id, args, kind) = match cfgcoll.get_expr(None,&args[0]) {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            assert_eq!(kind.clone(), TypeData::CollectionOfInt);
            assert_eq!(cfgcoll.get_function_name(id), Some("roll"));
            assert!(cfgcoll.is_function_stdlib(id));
            assert_eq!(args.len(), 3);
            (id.clone(), args.clone(), kind.clone())
        },
        anything_else => panic!("Expected roll function. Found: {:?}", anything_else)
    };


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
     * Get our `analyze` statement and step into the structure
     * here we re-veify that we're returning `sum`
     *
     */
    let (id, args, kind) = match cfgcoll.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            assert_eq!(cfgcoll.get_function_name(id), Some("sum"));
            assert!(cfgcoll.is_function_stdlib(id));
            assert_eq!(args.len(), 1);
            assert!(kind.clone() == TypeData::Int);
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };
    /*
     * we again chase `sum` into `re_roll1`
     *
     */
    let (id, args, kind) = match cfgcoll.get_expr(None, &args[0]) {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            // returns a collection of int
            assert_eq!(kind.clone(), TypeData::CollectionOfInt);
            // named reroll 1
            assert_eq!(cfgcoll.get_function_name(id), Some("reroll_1"));
            // not in stdlib
            assert!( !cfgcoll.is_function_stdlib(id)); 
            // has 1 argument
            assert_eq!(args.len(), 1);
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };

    /*
     * Changing context!!!!
     *
     * Here we step into the `reroll_1` function
     *
     */
    let ctx = match cfgcoll.get_function_context(&id) {
        Option::Some(x) => x,
        _ => panic!("expected a function context"),
    };

    /* 
     * Our returned result flows from `join`
     *
     */
    let (id, args, kind) = match ctx.get_return() {
        Option::Some(HashedExpression::Func(ref id, ref args, ref kind)) => {
            // strange?
            assert_eq!(cfgcoll.get_function_name(id), Some("join"));
            assert!(cfgcoll.is_function_stdlib(id));
            assert_eq!(args.len(), 2);
            assert!(kind.clone() == TypeData::CollectionOfInt);
            (id.clone(), args.clone(), kind.clone())
        }
        anything_else => panic!("Expected a function. Found: {:?}", anything_else),
    };

    /*
     * first argument of `join` is a variable
     *
     */
    match ctx.get_expr(Some(id),&args[0]).unwrap() {
        HashedExpression::Var(ref id, ref kind) => {
            assert_eq!(kind.clone(), TypeData::CollectionOfInt);
        },
        anything_else => panic!("expecting a var {:?}", anything_else)
    };



}
