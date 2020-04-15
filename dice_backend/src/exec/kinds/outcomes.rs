
use std::collections::BTreeMap;
use super::{Tuple,DataElement};


/*
 * Internal functions for combining the outcome of a parallel job
 *
 */

// tree_push acts like a `fold` system where we can merge
// incoming data types
fn tree_push(
    tree: BTreeMap<DataElement,f64>,
    element: Tuple,
) -> BTreeMap<DataElement,f64> {

    // ensure the element is consistently laid out.
    let mut element = element;
    element.sort_internal();
    let (element,prob) = element.split();

    let mut tree = tree;
    match tree.get_mut(&element) {
        Option::Some(el) => {
            *el += prob;
            return tree;
        }
        _ => {}
    };
    tree.insert(element,prob);
    tree
}

fn tree_merge(
    tree1: BTreeMap<DataElement,f64>,
    tree2: BTreeMap<DataElement,f64>
) -> BTreeMap<DataElement,f64> {
    let (mut bigger, smaller) = if tree1.len() >= tree2.len() {
        (tree1,tree2)
    } else {
        (tree2,tree1)
    };
    for (element,prob) in smaller {
        match bigger.get_mut(&element) {
            Option::Some(el) => {
                *el += prob;
                continue;
            },
            _ => { },
        };
        bigger.insert(element,prob);
    }
    bigger
}

