mod ord;
pub use self::ord::{OrdTrait, OrdType};
mod matcher;
pub use self::matcher::{Match, MatchTrait};
mod expr;
pub use self::expr::{ConstantValue, Op, OrderedExpression, StdLibraryFunc};
mod graphs;
pub use self::graphs::{
    AddSink, Graph, Inserter, ModifyGraph, Operation, RemoveSink, Remover, SwapSource,
};
mod coll;

/*
#[cfg(test)]
mod test {
    use super::{
        ConstantValue, Op, OrdTrait, OrdType, OrderedCollection, OrderedExpression, StdLibraryFunc,
    };

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
    fn test_removing_min_dice_roll() {
        let dut = r#"
const dice: vec<int> = roll_d6(3);
analyze (sum(dice) - min(dice));
"#;
        let coll = build_coll(dut);

        // assert the first operation is subtraction
        let (left, right) = match coll.get_expr(coll.get_return()).unwrap() {
            OrderedExpression::Op(Op::Sub(ref args)) => {
                assert_eq!(args.get_own_id(), coll.get_return());
                assert!(args.has_sources());
                assert_eq!(args.get_sources().len(), 2);

                (args.get_sources()[0].0, args.get_sources()[1].0)
            }
            _ => panic!("expected subtraction"),
        };

        // assert left is the sum expression
        match coll.get_expr(left).unwrap() {
            OrderedExpression::StdLib(StdLibraryFunc::Sum(ref args)) => {
                assert_eq!(args.get_own_id(), left);
                assert_eq!(args.get_sources().len(), 1);
                match coll.get_expr(args.get_sources()[0].0).unwrap() {
                    OrderedExpression::StdLib(StdLibraryFunc::D6(ref arg)) => {
                        // it should be aware it is used twice
                        assert_eq!(arg.get_sinks().len(), 2);
                    }
                    _ => panic!("expected a d6 function"),
                };
            }
            _ => panic!("expected sum function"),
        };

        match coll.get_expr(right).unwrap() {
            OrderedExpression::StdLib(StdLibraryFunc::Min(ref args)) => {
                assert_eq!(args.get_own_id(), right);
                assert_eq!(args.get_sources().len(), 1);
                match coll.get_expr(args.get_sources()[0].0).unwrap() {
                    OrderedExpression::StdLib(StdLibraryFunc::D6(ref arg)) => {
                        // it should be aware it is used twice
                        assert_eq!(arg.get_sinks().len(), 2);
                    }
                    _ => panic!("expected a d6 function"),
                };
            }
            _ => panic!("expected sum function"),
        }
    }
}
*/
