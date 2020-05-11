pub mod graphs;
mod holes;

pub use self::holes::brute_force_optimize;

#[cfg(test)]
mod test {
    use super::super::ordering::*;
    use super::brute_force_optimize;

    // construct the optimized collection
    fn build_coll(source: &str) -> OrderedCollection {
        use super::super::cfgbuilder::ExpressionCollection;
        use super::super::inliner::InlinedCollection;
        use super::super::namespace::Namespace;
        use super::super::parser_output::AbstractSyntaxTree;

        let ast = AbstractSyntaxTree::parse(source).unwrap();
        let namespace = Namespace::new(&ast).unwrap();
        let cfgcoll = ExpressionCollection::new(&namespace);
        let inlinecoll = InlinedCollection::new(&cfgcoll);
        OrderedCollection::new(&inlinecoll)
    }

    #[test]
    fn test_inline_add() {
        /*
         * This test actually involves 2 optimization actions "doing the right thing"
         *
         */
        let dut = r#"analyze (2 + len(roll_d6(3)));"#;
        let mut coll = build_coll(dut);
        brute_force_optimize(&mut coll);
        match coll.get_expr(coll.get_return()) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(5,_))) => {
                // yay!
            }
            what_ever => panic!("{:?}", what_ever)
        };
    }

    #[test]
    fn test_join_roll() {
        let dut = r#"analyze join(roll_d6(2), roll_d6(3));"#;
        let mut coll = build_coll(dut);
        brute_force_optimize(&mut coll);
        match coll.get_expr(coll.get_return()) {
            Option::Some(OrderedExpression::StdLib(StdLibraryFunc::D6(ref args))) => {
                assert!(matches!( coll.get_expr(args.get_sources()[0].0), Option::Some(OrderedExpression::Constant(ConstantValue::Int(5,_)))));
            }
            what_ever => panic!("{:?}", what_ever)
        };
    }

    #[test]
    fn test_len_roll() {
        let dut = r#"analyze len(roll_d6(5));"#;
        let mut coll = build_coll(dut);

        // assert that it currently makes sense
        let old_return = coll.get_return();
        match coll.get_expr(coll.get_return()) {
            Option::Some(OrderedExpression::StdLib(StdLibraryFunc::Len(_))) => {
                // cool that is our length
            }
            what_ever => panic!("{:?}", what_ever),
        };

        brute_force_optimize(&mut coll);

        let post_opt_return = coll.get_return();
        assert!(post_opt_return != old_return, "returns are eq?");
        match coll.get_expr(coll.get_return()) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(5, _))) => {
                // yay!
            }
            what_ever => panic!("unexpected: {:?}", what_ever),
        };
    }
}
