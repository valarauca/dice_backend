
mod ord;
pub use self::ord::{OrdTrait,OrdType};
mod expr;
pub use self::expr::{OrderedExpression,StdLibraryFunc,ConstantValue,Op};
mod coll;
pub use self::coll::{OrderedCollection};



#[cfg(test)]
mod test {
    use super::{OrderedCollection,OrderedExpression,StdLibraryFunc,ConstantValue,Op,OrdTrait,OrdType};

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
        let (left, right) = match coll[coll.get_return()] {
            OrderedExpression::Op(Op::Sub(ref args)) => {
                assert_eq!(args.own_id(), coll.get_return()); 
                assert!(args.has_sources());
                assert_eq!(args.get_sources().len(), 2);

                (args.get_sources()[0].0,args.get_sources()[1].0)
            },
            _ => panic!("expected subtraction"),
        };

        // assert left is the sum expression
        match coll[left] {
            OrderedExpression::StdLib(StdLibraryFunc::Sum(ref args)) => {
                assert_eq!(args.own_id(), left);
                assert_eq!(args.get_sources().len(), 1);
                match coll[args.get_sources()[0].0] {
                    OrderedExpression::StdLib(StdLibraryFunc::D6(ref arg)) => {
                        // it should be aware it is used twice
                        assert_eq!(arg.get_sinks().len(), 2);
                    }
                    _ => panic!("expected a d6 function"),
                };
            }
            _ => panic!("expected sum function")
        };

        match coll[right] {
            OrderedExpression::StdLib(StdLibraryFunc::Min(ref args)) => {
                assert_eq!(args.own_id(), right);
                assert_eq!(args.get_sources().len(), 1);
                match coll[args.get_sources()[0].0] {
                    OrderedExpression::StdLib(StdLibraryFunc::D6(ref arg)) => {
                        // it should be aware it is used twice
                        assert_eq!(arg.get_sinks().len(), 2);
                    }
                    _ => panic!("expected a d6 function"),
                };
            }
            _ => panic!("expected sum function")
        }
    
    }

}
