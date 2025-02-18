use pxp_span::Span;
use pxp_type::{Type, TypeWithSymbolTable};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataType {
    pub kind: Type,
    pub span: Span,
}

impl DataType {
    pub fn new(kind: Type, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn get_type(&self) -> &Type {
        &self.kind
    }

    pub fn get_span(&self) -> Span {
        self.span
    }

    pub fn standalone(&self) -> bool {
        self.kind.standalone()
    }

    pub fn nullable(&self) -> bool {
        self.kind.nullable()
    }

    pub fn includes_callable(&self) -> bool {
        self.kind.includes_callable()
    }

    pub fn is_bottom(&self) -> bool {
        self.kind.is_bottom()
    }

    pub fn with_symbol_table<'a>(&self, symbol_table: &'a pxp_symbol::SymbolTable) -> TypeWithSymbolTable<'a> {
        self.kind.with_symbol_table(symbol_table)
    }
}