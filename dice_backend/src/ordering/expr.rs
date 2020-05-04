use super::super::inliner::{
    BoolArg as BArg, BoolOrInt as BI, InlinedCollection, InlinedExpression, IntArg as IArg,
    Op as IOp,
};
use super::super::parser_output::TypeData;

use super::coll::OrderingCollection;
use super::ord::{ExprVec, OrdTrait, OrdType};

const I: TypeData = TypeData::Int;
const B: TypeData = TypeData::Bool;
const C_I: TypeData = TypeData::CollectionOfInt;
const C_B: TypeData = TypeData::CollectionOfBool;

/// Various Expressions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderedExpression {
    StdLib(StdLibraryFunc),
    Constant(ConstantValue),
    Op(Op),
}
impl OrderedExpression {
    /// converts an inlined expression into an ordered expression
    pub fn new(
        expr: &InlinedExpression,
        old_coll: &InlinedCollection,
        new_coll: &mut OrderingCollection,
    ) {
        // short circuit on already processed items
        let self_id = expr.get_hash();
        if new_coll.contains(&self_id) {
            // expression already exists in collection
            // so we already processed it
            return;
        }

        // build our new expression, for inserting it
        let item = match expr {
            &InlinedExpression::D6(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, I);
                OrderedExpression::StdLib(StdLibraryFunc::D6(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*arg, I)],
                )))
            }
            &InlinedExpression::D3(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, I);
                OrderedExpression::StdLib(StdLibraryFunc::D3(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*arg, I)],
                )))
            }
            &InlinedExpression::Sum(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, C_I);
                OrderedExpression::StdLib(StdLibraryFunc::Sum(OrdType::new(
                    self_id,
                    I,
                    s_v![(*arg, C_I)],
                )))
            }
            &InlinedExpression::Max(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, C_I);
                OrderedExpression::StdLib(StdLibraryFunc::Max(OrdType::new(
                    self_id,
                    I,
                    s_v![(*arg, C_I)],
                )))
            }
            &InlinedExpression::Min(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, C_I);
                OrderedExpression::StdLib(StdLibraryFunc::Min(OrdType::new(
                    self_id,
                    I,
                    s_v![(*arg, C_I)],
                )))
            }
            &InlinedExpression::Count(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, C_B);
                OrderedExpression::StdLib(StdLibraryFunc::Count(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*arg, C_B)],
                )))
            }
            &InlinedExpression::Len(ref arg) => {
                // look up our argument
                let old_arg = old_coll.get_expr(arg).unwrap();
                // ensure that it is inserted into our new collection.
                OrderedExpression::new(old_arg, old_coll, new_coll);
                // mark that we use it, and how we use it.
                new_coll.set_expr_sink(arg, self_id, C_I);
                OrderedExpression::StdLib(StdLibraryFunc::Len(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*arg, C_I)],
                )))
            }
            &InlinedExpression::Filter(ref a, ref b) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::StdLib(StdLibraryFunc::Filter(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_B), (*b, C_I)],
                )))
            }
            &InlinedExpression::Join(ref a, ref b) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::StdLib(StdLibraryFunc::Join(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_I), (*b, C_I)],
                )))
            }
            &InlinedExpression::ConstantInt(ref i) => {
                // no dependent expressions
                OrderedExpression::Constant(ConstantValue::Int(
                    *i,
                    OrdType::new(self_id, I, s_v![]),
                ))
            }
            &InlinedExpression::ConstantBool(ref b) => {
                // no dependent expressions
                OrderedExpression::Constant(ConstantValue::Bool(
                    *b,
                    OrdType::new(self_id, B, s_v![]),
                ))
            }
            InlinedExpression::Op(IOp::Add(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Add(OrdType::new(self_id, I, s_v![(*a, I), (*b, I)])))
            }
            InlinedExpression::Op(IOp::Add(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::Add(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::Add(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Add(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::Sub(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Sub(OrdType::new(self_id, I, s_v![(*a, I), (*b, I)])))
            }
            InlinedExpression::Op(IOp::Sub(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::Sub(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::Sub(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Sub(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::Mul(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Mul(OrdType::new(self_id, I, s_v![(*a, I), (*b, I)])))
            }
            InlinedExpression::Op(IOp::Mul(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::Mul(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::Mul(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Mul(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::Div(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Div(OrdType::new(self_id, I, s_v![(*a, I), (*b, I)])))
            }
            InlinedExpression::Op(IOp::Div(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::Div(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::Div(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Div(OrdType::new(
                    self_id,
                    C_I,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThan(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::GreaterThan(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThan(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::GreaterThan(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThan(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::GreaterThan(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThan(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::LessThan(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThan(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::LessThan(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThan(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::LessThan(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThanEqual(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::LessThanEqual(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThanEqual(IArg::Int_CollectionOfInt(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::LessThanEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::LessThanEqual(IArg::CollectionOfInt_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::LessThanEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThanEqual(IArg::Int_Int(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::GreaterThanEqual(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThanEqual(IArg::Int_CollectionOfInt(
                ref a,
                ref b,
            ))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::GreaterThanEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::GreaterThanEqual(IArg::CollectionOfInt_Int(
                ref a,
                ref b,
            ))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::GreaterThanEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::Equal(BI::Int(IArg::Int_Int(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Equal(OrdType::new(self_id, B, s_v![(*a, I), (*b, I)])))
            }
            InlinedExpression::Op(IOp::Equal(BI::Int(IArg::Int_CollectionOfInt(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::Equal(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::Equal(BI::Int(IArg::CollectionOfInt_Int(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::Equal(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::Equal(BI::Bool(BArg::Bool_Bool(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::Equal(OrdType::new(self_id, B, s_v![(*a, B), (*b, B)])))
            }
            InlinedExpression::Op(IOp::Equal(BI::Bool(BArg::Bool_CollectionOfBool(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_B);

                OrderedExpression::Op(Op::Equal(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, B), (*b, C_B)],
                )))
            }
            InlinedExpression::Op(IOp::Equal(BI::Bool(BArg::CollectionOfBool_Bool(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::Equal(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_B), (*b, B)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Int(IArg::Int_Int(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Int(IArg::Int_CollectionOfInt(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_I);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, I), (*b, C_I)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Int(IArg::CollectionOfInt_Int(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_I);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, I);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_I), (*b, I)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Bool(BArg::Bool_Bool(ref a, ref b)))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    B,
                    s_v![(*a, B), (*b, B)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Bool(BArg::Bool_CollectionOfBool(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_B);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, B), (*b, C_B)],
                )))
            }
            InlinedExpression::Op(IOp::NotEqual(BI::Bool(BArg::CollectionOfBool_Bool(
                ref a,
                ref b,
            )))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::NotEqual(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_B), (*b, B)],
                )))
            }
            InlinedExpression::Op(IOp::And(BArg::Bool_Bool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::And(OrdType::new(self_id, B, s_v![(*a, C_B), (*b, B)])))
            }
            InlinedExpression::Op(IOp::And(BArg::Bool_CollectionOfBool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_B);

                OrderedExpression::Op(Op::And(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_B), (*b, B)],
                )))
            }
            InlinedExpression::Op(IOp::And(BArg::CollectionOfBool_Bool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::And(OrdType::new(
                    self_id,
                    C_B,
                    s_v![(*a, C_B), (*b, B)],
                )))
            }
            InlinedExpression::Op(IOp::Or(BArg::Bool_Bool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::Or(OrdType::new(self_id, B, s_v![(*a, C_B), (*b, B)])))
            }
            InlinedExpression::Op(IOp::Or(BArg::Bool_CollectionOfBool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, C_B);

                OrderedExpression::Op(Op::Or(OrdType::new(self_id, C_B, s_v![(*a, C_B), (*b, B)])))
            }
            InlinedExpression::Op(IOp::Or(BArg::CollectionOfBool_Bool(ref a, ref b))) => {
                // ensure `a` is inserted
                let old_arg_a = old_coll.get_expr(a).unwrap();
                OrderedExpression::new(old_arg_a, old_coll, new_coll);
                new_coll.set_expr_sink(a, self_id, C_B);

                // ensure `b` is inserted
                let old_arg_b = old_coll.get_expr(b).unwrap();
                OrderedExpression::new(old_arg_b, old_coll, new_coll);
                new_coll.set_expr_sink(b, self_id, B);

                OrderedExpression::Op(Op::Or(OrdType::new(self_id, C_B, s_v![(*a, C_B), (*b, B)])))
            }
        };
        // insert the new item
        new_coll.insert(item);
    }
}
impl PartialEq<TypeData> for OrderedExpression {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        self.as_ref().eq(other)
    }
}
impl AsRef<OrdType> for OrderedExpression {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a OrdType {
        match self {
            &OrderedExpression::StdLib(ref s) => s.as_ref(),
            &OrderedExpression::Constant(ref c) => c.as_ref(),
            &OrderedExpression::Op(ref o) => o.as_ref(),
        }
    }
}
impl AsMut<OrdType> for OrderedExpression {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut OrdType {
        match self {
            &mut OrderedExpression::StdLib(ref mut s) => s.as_mut(),
            &mut OrderedExpression::Constant(ref mut c) => c.as_mut(),
            &mut OrderedExpression::Op(ref mut o) => o.as_mut(),
        }
    }
}
impl OrdTrait for OrderedExpression {}

/// Constant Values
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConstantValue {
    Bool(bool, OrdType),
    Int(i8, OrdType),
}
impl AsRef<OrdType> for ConstantValue {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a OrdType {
        match self {
            &ConstantValue::Bool(_, ref x) | &ConstantValue::Int(_, ref x) => x.as_ref(),
        }
    }
}
impl AsMut<OrdType> for ConstantValue {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut OrdType {
        match self {
            &mut ConstantValue::Bool(_, ref mut x) | &mut ConstantValue::Int(_, ref mut x) => {
                x.as_mut()
            }
        }
    }
}
impl PartialEq<TypeData> for ConstantValue {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        self.as_ref().eq(other)
    }
}
impl OrdTrait for ConstantValue {}

/// StandardLibrary functions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StdLibraryFunc {
    D6(OrdType),
    D3(OrdType),
    Filter(OrdType),
    Count(OrdType),
    Len(OrdType),
    Join(OrdType),
    Sum(OrdType),
    Max(OrdType),
    Min(OrdType),
}
impl AsRef<OrdType> for StdLibraryFunc {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a OrdType {
        match self {
            &StdLibraryFunc::D6(ref x)
            | &StdLibraryFunc::D3(ref x)
            | &StdLibraryFunc::Filter(ref x)
            | &StdLibraryFunc::Count(ref x)
            | &StdLibraryFunc::Len(ref x)
            | &StdLibraryFunc::Join(ref x)
            | &StdLibraryFunc::Sum(ref x)
            | &StdLibraryFunc::Max(ref x)
            | &StdLibraryFunc::Min(ref x) => x.as_ref(),
        }
    }
}
impl AsMut<OrdType> for StdLibraryFunc {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut OrdType {
        match self {
            &mut StdLibraryFunc::D6(ref mut x)
            | &mut StdLibraryFunc::D3(ref mut x)
            | &mut StdLibraryFunc::Filter(ref mut x)
            | &mut StdLibraryFunc::Count(ref mut x)
            | &mut StdLibraryFunc::Len(ref mut x)
            | &mut StdLibraryFunc::Join(ref mut x)
            | &mut StdLibraryFunc::Sum(ref mut x)
            | &mut StdLibraryFunc::Max(ref mut x)
            | &mut StdLibraryFunc::Min(ref mut x) => x.as_mut(),
        }
    }
}
impl PartialEq<TypeData> for StdLibraryFunc {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        self.as_ref().eq(other)
    }
}
impl OrdTrait for StdLibraryFunc {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Add(OrdType),
    Sub(OrdType),
    Mul(OrdType),
    Div(OrdType),
    Equal(OrdType),
    NotEqual(OrdType),
    GreaterThan(OrdType),
    GreaterThanEqual(OrdType),
    LessThan(OrdType),
    LessThanEqual(OrdType),
    Or(OrdType),
    And(OrdType),
}
impl AsRef<OrdType> for Op {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a OrdType {
        match self {
            &Op::Add(ref x)
            | &Op::Sub(ref x)
            | &Op::Mul(ref x)
            | &Op::Div(ref x)
            | &Op::Equal(ref x)
            | &Op::NotEqual(ref x)
            | &Op::GreaterThan(ref x)
            | &Op::GreaterThanEqual(ref x)
            | &Op::LessThan(ref x)
            | &Op::LessThanEqual(ref x)
            | &Op::Or(ref x)
            | &Op::And(ref x) => x.as_ref(),
        }
    }
}
impl AsMut<OrdType> for Op {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut OrdType {
        match self {
            &mut Op::Add(ref mut x)
            | &mut Op::Sub(ref mut x)
            | &mut Op::Mul(ref mut x)
            | &mut Op::Div(ref mut x)
            | &mut Op::Equal(ref mut x)
            | &mut Op::NotEqual(ref mut x)
            | &mut Op::GreaterThan(ref mut x)
            | &mut Op::GreaterThanEqual(ref mut x)
            | &mut Op::LessThan(ref mut x)
            | &mut Op::LessThanEqual(ref mut x)
            | &mut Op::Or(ref mut x)
            | &mut Op::And(ref mut x) => x.as_mut(),
        }
    }
}
impl PartialEq<TypeData> for Op {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        self.as_ref().eq(other)
    }
}
impl OrdTrait for Op {}
