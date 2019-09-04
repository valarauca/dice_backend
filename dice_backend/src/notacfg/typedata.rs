

/// FullTypeData encodes much more information then the standard
/// typing information. It attempts to encode ranges as well as
/// other data, like collection size.
pub enum FullTypeData {
    IntegerBounded(isize,isize),
    Boolean,
    Unknown
}
