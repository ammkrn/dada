use crate::{op::Op, span::Span, storage_mode::StorageMode, word::Word};
use dada_id::{id, tables};

/// Stores the ast for a function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree {
    /// Interning tables for expressions and the like.
    pub tables: Tables,

    /// The root
    pub root_expr: Expr,
}

tables! {
    /// Tables that store the data for expr in the AST.
    /// You can use `tables[expr]` (etc) to access the data.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Tables {
        exprs: alloc Expr => ExprData,
        named_exprs: alloc NamedExpr => NamedExprData,
        blocks: alloc Block => BlockData,
    }
}

span_table! {
    /// Side table that contains the spans for everything in an AST.
    /// This isn't normally needed except for diagnostics, so it's
    /// kept separate to avoid reducing incremental reuse.
    /// You can request it by invoking the `spans`
    /// method in the `dada_parse` prelude.
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Spans {
        expr_spans: Expr => Span,
        named_expr_spans: NamedExpr => NamedExprSpan,
        block_spans: Block => Span,
    }
}

id!(pub struct Expr);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum ExprData {
    Id(Word),

    /// true, false
    BooleanLiteral(bool),

    /// `22`, `22_222`, etc
    IntegerLiteral(Word),

    /// `"foo"` with no format strings
    ///
    /// FIXME: We should replace the FormatString token with a Concatenate
    /// that has parsed expressions.
    StringLiteral(Word),

    /// `expr.ident`
    Dot(Expr, Word),

    /// `expr.await`
    Await(Expr),

    /// `expr(id: expr, ...)`
    Call(Expr, Vec<NamedExpr>),

    /// `expr.share`
    Share(Expr),

    /// `expr.lease`
    Lease(Expr),

    /// `expr.give`
    Give(Expr),

    /// `[shared|var|atomic] x = expr`
    Var(StorageMode, Word, Expr),

    /// `(expr)`
    Parenthesized(Expr),

    /// `if condition { block } [else { block }]`
    If(Expr, Expr, Option<Expr>),

    /// `atomic { block }`
    Atomic(Expr),

    /// `loop { block }`
    Loop(Expr),

    /// `while condition { block }`
    While(Expr, Expr),

    // { ... } ==> closure?
    Block(Block),

    /// `a + b`
    Op(Expr, Op, Expr),

    /// `a += b`
    OpEq(Expr, Op, Expr),

    /// `a := b`
    Assign(Expr, Expr),

    /// parse or other error
    Error,
}

id!(pub struct NamedExpr);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct NamedExprData {
    pub name: Word,
    pub expr: Expr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NamedExprSpan {
    pub span: Span,
    pub name_span: Span,
}

id!(pub struct Block);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct BlockData {
    pub exprs: Vec<Expr>,
}