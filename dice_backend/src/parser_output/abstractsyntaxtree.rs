use std::fmt;

use super::structures::Structures;

use super::super::lalrpop_util::ParseError;
use super::super::syntaxhelper::CharacterLookup;

/// AbstractSyntaxTree is the top level of parse.
///
/// Additional passes are made before a "parse" is
/// complete to ensure that literals are well formed.
pub struct AbstractSyntaxTree<'a> {
    pub ast: Box<[Structures<'a>]>,
}
impl<'a> fmt::Display for AbstractSyntaxTree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.ast.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
impl<'a> AbstractSyntaxTree<'a> {
    pub fn new(args: Vec<(Structures<'a>)>) -> AbstractSyntaxTree<'a> {
        let ast = args.into_boxed_slice();
        AbstractSyntaxTree { ast }
    }

    /*
    /// Parse will attempt to construct an abstract syntax tree from the input
    pub fn parse<'b>(input: &'b str) -> Result<AbstractSyntaxTree<'b>,String> {
        let index = CharacterLookup::new(input);
        match TreeParser::new().parse(input) {
            Ok(tree) => Ok(tree),
            Err(ParseError::InvalidToken{ location }) => {
                Err(format!("Unable to parse: InvalidToken.\n character: {} line: {} \n {} \n", index.get_char(location), index.get_line_number(location), index.get_line(location)))
            },
            Err(ParseError::UnrecognizedEOF{ location: _, expected: _}) => {
                Err(format!("File terminated before it should"))
            },
            Err(ParseError::UnrecognizedToken{token: (a,_,b), expected }) => {
                Err(format!("Unable to parse: UnreconginzedToken.\n start_line: {} ending_line: {}\n Offending section:\"{}\"\n{}", index.get_line_number(a), index.get_line_number(b), index.get_span(a,b), index.get_span_lines(a,b)))
            },
            Err(_) => {
                unreachable!()
            }
        }
    }
    */
}
