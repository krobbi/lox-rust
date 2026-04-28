use std::fmt::{self, Display, Formatter};

use super::Symbol;

// TODO: Implement a system to display values using a symbol table.
impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "symbol_{}", self.0)
    }
}
