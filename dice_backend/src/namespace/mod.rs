//! Here we resolve namespacing, and prune some low-hanging
//! fruit errors. The type system is extremely trivial so
//! there is not much to check.
mod block;
pub use self::block::BasicBlock;
mod blockexpression;
pub use self::blockexpression::BlockExpression;
mod namespace;
pub use self::namespace::Namespace;

#[test]
fn test_namspace_with_trivial_program_1() {
    use super::parser_output::AbstractSyntaxTree;

    let trivial_program = "analyze roll_d6(10);";
    let ast = match AbstractSyntaxTree::parse(trivial_program) {
        Ok(ast) => ast,
        Err(e) => panic!("ast error: {:?}", e),
    };
    let namespace = match Namespace::new(&ast.ast) {
        Ok(namespace) => namespace,
        Err(e) => panic!("namespace error: {:?}", e),
    };

    // assert the stdlib is populated
    assert!(namespace.get_function("roll_d6").is_some());
    assert!(namespace.get_function("roll_d3").is_some());
    assert!(namespace.get_function("roll").is_some());
    assert!(namespace.get_function("filter").is_some());
    assert!(namespace.get_function("sum").is_some());

    // assert not constants are declared
    assert!(namespace.get_all_constants().count() == 0);

    // assert an analysis exists
    assert!(namespace.get_analysis().is_some());
}

#[test]
fn test_namspace_with_trivial_program_2() {
    use super::parser_output::AbstractSyntaxTree;
    let trivial_program = "
const value:int = 10;
analyze roll_d6(value);
";
    let ast = match AbstractSyntaxTree::parse(trivial_program) {
        Ok(ast) => ast,
        Err(e) => panic!("ast error: {:?}", e),
    };
    let namespace = match Namespace::new(&ast.ast) {
        Ok(namespace) => namespace,
        Err(e) => panic!("namespace error: {:?}", e),
    };

    // assert the stdlib is populated
    assert!(namespace.get_function("roll_d6").is_some());
    assert!(namespace.get_function("roll_d3").is_some());
    assert!(namespace.get_function("roll").is_some());
    assert!(namespace.get_function("filter").is_some());
    assert!(namespace.get_function("sum").is_some());

    // assert not constants are declared
    assert!(namespace.get_all_constants().count() == 1);
    assert!(namespace.get_constant("value").is_some());

    // assert an analysis exists
    assert!(namespace.get_analysis().is_some());
}
