use std::collections::HashMap;

use pxp_bytestring::ByteStr;
use serde::{Serialize, Deserialize};

pub type Symbol = usize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SymbolTable {
    map: HashMap<Vec<u8>, Symbol>,
    vec: Vec<Vec<u8>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self {
            map: HashMap::new(),
            vec: Vec::new(),
        };

        // This will reserve Symbol(0) for internal use (missing tokens, etc.)
        table.intern(b"");
        table
    }

    pub fn intern(&mut self, contents: &[u8]) -> Symbol {
        if let Some(symbol) = self.map.get(contents) {
            return *symbol;
        }

        let symbol = self.vec.len();

        self.map.insert(contents.to_vec(), symbol);
        self.vec.push(contents.to_vec());

        symbol
    }

    pub fn find(&self, contents: &[u8]) -> Option<Symbol> {
        self.map.get(contents).copied()
    }

    pub fn must_find(&self, contents: &[u8]) -> Symbol {
        self.find(contents).expect(&format!("Symbol for {} not found", ByteStr::from(contents)))
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<ByteStr> {
        self.vec.get(symbol).map(|s| ByteStr::new(s))
    }

    pub fn coagulate(&mut self, symbols: &[Symbol], with: Option<&[u8]>) -> Symbol {
        let mut contents = Vec::new();

        for (i, symbol) in symbols.iter().enumerate() {
            contents.extend_from_slice(&self.resolve(*symbol).unwrap());

            if i < symbols.len() - 1 {
                if let Some(with) = with {
                    contents.extend_from_slice(with);
                }
            }
        }

        self.intern(&contents)
    }
}

#[cfg(test)]
mod tests {
    use pxp_bytestring::ByteStr;

    use super::SymbolTable;

    #[test]
    fn it_can_be_created() {
        let _ = SymbolTable::new();
    }

    #[test]
    fn it_can_intern_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        assert_eq!(symbols.intern(sample_text), 1);
    }

    #[test]
    fn it_can_resolve_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        let symbol = symbols.intern(sample_text);

        assert_eq!(symbols.resolve(symbol), Some(ByteStr::new(sample_text)));
    }
}
