use super::super::itertools::Itertools;
use super::super::parser_output::TypeData;

use super::{
    AddSink, Graph, Inserter, Match, MatchTrait, Modifications, ModifyGraph, Operation, OrdTrait,
    OrdType, OrderedCollection, OrderedExpression, RemoveSink, Remover, SwapSource,
};

pub mod inline_add;
pub mod inline_and;
pub mod inline_div;
pub mod inline_mul;
pub mod inline_or;
pub mod inline_sub;
pub mod join_rolls;
pub mod len_dice;

/// Peeping is a basic signature to define an operation
pub type Peeping = &'static (dyn Fn(u64, &OrderedCollection) -> Option<Modifications<OrderedExpression>>
              + 'static);

const OPT: &'static [Peeping] = &[
    &inline_add::add_inline,
    &inline_sub::sub_inline,
    &inline_div::div_inline,
    &inline_mul::mul_inline,
    &inline_and::and_inline,
    &inline_or::or_inline,
    &len_dice::len_dice,
    &join_rolls::join_roll,
];

pub fn brute_force_optimize(coll: &mut OrderedCollection) {
    loop {
        // did we find something to change?
        let found_opt = coll
            .keys()
            .cartesian_product(OPT)
            .flat_map(|(key, lambda)| lambda(*key, coll))
            .next();

        // change it
        match found_opt {
            Option::None => {
                break;
            }
            Option::Some(item) => {
                item.apply(coll);
            }
        };
    }
}

/// handles the boilerplate of propigating constants
pub fn interior<A, B, C>(
    root: &A,
    new_id: u64,
    kind: TypeData,
    x_args: &B,
    y_args: &C,
) -> (OrdType, Modifications<OrderedExpression>)
where
    A: OrdTrait,
    B: OrdTrait,
    C: OrdTrait,
{
    let root_m: Match = Match::from(root);
    let x_args_m: Match = Match::from(x_args);
    let y_args_m: Match = Match::from(y_args);

    let mut mods = Modifications::default();
    let mut new_constant: OrdType = OrdType::new((new_id, kind), Option::<Match>::None);

    // where ever we sink the result of the add,
    // we need to sink the result of the new constant
    for sink in root.get_sinks() {
        mods.push(SwapSource::new::<_, _, _>(
            *sink,
            root_m,
            new_constant.clone(),
        ));
        new_constant.add_sink(sink);
    }

    // determine if we can drop our sink?
    mods.push(RemoveSink::new(x_args_m, root_m));
    if x_args.get_sinks().len() == 1 {
        mods.push(Remover::new(x_args_m));
    }
    mods.push(RemoveSink::new(y_args_m, root_m));
    if y_args.get_sinks().len() == 1 {
        mods.push(Remover::new(y_args_m));
    }

    (new_constant, mods)
}
