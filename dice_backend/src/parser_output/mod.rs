#![allow(unused_imports, dead_code)]

use std::fmt;

mod operation;
pub use self::operation::Operation;

mod typedata;
pub use self::typedata::TypeData;

mod literal;
pub use self::literal::Literal;

mod operationresult;
pub use self::operationresult::OperationResult;

mod varreference;
pub use self::varreference::VariableReference;

mod literalvalue;
pub use self::literalvalue::LiteralValue;

mod functioninvoke;
pub use self::functioninvoke::FunctionInvocation;

mod expression;
pub use self::expression::Expression;

mod constantdeclaration;
pub use self::constantdeclaration::ConstantDeclaration;

mod analysisdeclaration;
pub use self::analysisdeclaration::AnalysisDeclaration;

mod functiondeclaration;
pub use self::functiondeclaration::FunctionDeclaration;

mod structures;
pub use self::structures::Structures;

mod variabledeclaration;
pub use self::variabledeclaration::VariableDeclaration;

mod terminalexpression;
pub use self::terminalexpression::TerminalExpression;

mod statement;
pub use self::statement::Statement;

mod statements;
pub use self::statements::Statements;

mod abstractsyntaxtree;
pub use self::abstractsyntaxtree::AbstractSyntaxTree;

/// GetType is used for lower level expressions it offers a method of returning typing data.
pub trait GetType {
    /// used to signal that `get_type` will always fail until namspacing is complete.
    fn requires_namespace(&self) -> bool;

    /// returns typing information.
    fn get_type(&self) -> Result<TypeData, String>;
}
