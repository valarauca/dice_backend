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
}
