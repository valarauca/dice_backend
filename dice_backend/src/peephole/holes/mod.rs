use super::super::itertools::Itertools;
use super::super::ordering::{OrdTrait, OrdType, OrderedCollection, OrderedExpression};
use super::super::parser_output::TypeData;
use super::graphs::{Modifications, ModifyGraph};

pub mod inline_add;
pub mod inline_and;
pub mod inline_div;
pub mod inline_mul;
pub mod inline_or;
pub mod inline_sub;
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
    use super::graphs::{Modifications, RemoveSink, Remover, SwapSource};

    let mut mods = Modifications::default();
    let mut new_constant = OrdType::new(new_id, kind, s_v![]);

    // where ever we sink the result of the add,
    // we need to sink the result of the new constant
    for sink in root.get_sinks() {
        mods.push(SwapSource::new(
            sink,
            root.get_matcher_tuple(),
            new_constant.get_matcher_tuple(),
        ));
        new_constant.add_sink(sink.0, sink.1);
    }

    // determine if we can drop our sink?
    mods.push(RemoveSink::new(x_args, root));
    if x_args.get_sinks().len() == 1 {
        mods.push(Remover::new(x_args));
    }
    mods.push(RemoveSink::new(y_args, root));
    if y_args.get_sinks().len() == 1 {
        mods.push(Remover::new(y_args));
    }

    (new_constant, mods)
}
