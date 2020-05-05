use super::super::super::ordering::*;
use super::super::super::parser_output::TypeData;

use super::super::graphs::*;

pub fn join_roll(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    // check that this is in fact a join expression
    let join_op = match coll.get_expr(expr) {
        Option::Some(OrderedExpression::StdLib(StdLibraryFunc::Join(ref args)))
            if args == TypeData::CollectionOfInt =>
        {
            args
        }
        _ => return None,
    };

    let (left, right, expect) = match (
        coll.get_expr(join_op.get_sources()[0].0).unwrap(),
        coll.get_expr(join_op.get_sources()[1].0).unwrap(),
    ) {
        // source code is roughly `join(roll_d6(_),roll_d6(_))`
        (
            OrderedExpression::StdLib(StdLibraryFunc::D6(ref left_d6)),
            OrderedExpression::StdLib(StdLibraryFunc::D6(ref right_d6)),
        ) => (left_d6, right_d6, Expect::D6),

        // source code is roughy `join(roll_d3(_),roll_d3(_))`
        (
            OrderedExpression::StdLib(StdLibraryFunc::D3(ref left_d3)),
            OrderedExpression::StdLib(StdLibraryFunc::D3(ref right_d3)),
        ) => (left_d3, right_d3, Expect::D3),
        _ => return None,
    };

    // now inspect the arguments of the `roll_d3(_)` or `roll_d6(_)` calls.
    match (
        coll.get_expr(left.get_sources()[0].0).unwrap(),
        coll.get_expr(right.get_sources()[0].0).unwrap(),
    ) {
        (
            OrderedExpression::Constant(ConstantValue::Int(ref l_val, ref l_args)),
            OrderedExpression::Constant(ConstantValue::Int(ref r_val, ref r_args)),
        ) => {
            // fetch a new identifier
            let new_id_const = coll.next_free_id(None);
            // fetch a new identifier for the roll.
            let new_id_roll = coll.next_free_id(Some(new_id_const));
            Some(internal_boilerplate(
                expect,
                join_op,
                new_id_const,
                new_id_roll,
                left,
                l_args,
                *l_val,
                right,
                r_args,
                *r_val,
            ))
        }
        _ => None,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Expect {
    D6,
    D3,
}

/// all of this is identical no matter what type of dice
/// we're dealing with.
///
/// we just need to tell this function what kind of dice
/// it needs to construct
#[inline(always)]
fn internal_boilerplate<A, B, C, D, E>(
    expect: Expect,
    join_op: &A,
    new_const_id: u64,
    new_roll_id: u64,
    l_roll: &B,
    l_const: &C,
    l_val: i8,
    r_roll: &D,
    r_const: &E,
    r_val: i8,
) -> Modifications<OrderedExpression>
where
    A: OrdTrait,
    B: OrdTrait,
    C: OrdTrait,
    D: OrdTrait,
    E: OrdTrait,
{
    // we can patch the AST
    let mut mods = Modifications::default();

    // create our new constant
    let mut new_const = OrdType::new(new_const_id, TypeData::Int, s_v![]);
    // create or new roll invocation
    let mut new_roll = OrdType::new(
        new_roll_id,
        TypeData::CollectionOfInt,
        s_v![(new_const_id, TypeData::Int)],
    );
    new_const.add_sink(new_roll.get_own_id(), new_roll.get_own_type());

    // for every `join(_,_)`'s result flows, we need to update that.
    for sink in join_op.get_sinks() {
        mods.push(SwapSource::new(
            sink,
            join_op.get_matcher_tuple(),
            new_roll.get_matcher_tuple(),
        ));
        new_roll.add_sink(sink.0, sink.1);
    }
    // handle if we have some `analyze join(_,_)`
    mods.push(SwapSource::new(
        Match::default(),
        join_op.get_matcher_tuple(),
        new_roll.get_matcher_tuple(),
    ));

    // remove the join operation itself
    mods.push(Remover::new(join_op));
    // insert the new arguments
    match expect {
        Expect::D3 => {
            mods.push(Inserter::new(OrderedExpression::StdLib(
                StdLibraryFunc::D3(new_roll),
            )));
        }
        Expect::D6 => {
            mods.push(Inserter::new(OrderedExpression::StdLib(
                StdLibraryFunc::D6(new_roll),
            )));
        }
    };
    mods.push(Inserter::new(OrderedExpression::Constant(
        ConstantValue::Int(l_val + r_val, new_const),
    )));

    // tell our constants they aren't being consumed
    // if possible, remove them.
    mods.push(RemoveSink::new(l_const, l_roll));
    if l_const.get_sinks().len() == 1 {
        mods.push(Remover::new(l_const));
    }
    mods.push(RemoveSink::new(r_const, r_roll));
    if r_const.get_sinks().len() == 1 {
        mods.push(Remover::new(r_const));
    }

    // tell our `roll_d3|d6(_)` they aren't being consumed
    // if possible, remove them.
    mods.push(RemoveSink::new(l_roll, join_op));
    if l_roll.get_sinks().len() == 1 {
        mods.push(Remover::new(l_roll));
    }
    mods.push(RemoveSink::new(r_roll, join_op));
    if r_roll.get_sinks().len() == 1 {
        mods.push(Remover::new(r_roll));
    }

    // return the vector of operations needed to patch the graph
    mods
}
