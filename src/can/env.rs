use can::expr::Expr;
use can::pattern::Pattern;
use can::problem::Problem;
use can::procedure::{Procedure, References};
use can::symbol::Symbol;
use collections::{ImMap, MutMap};
use region::{Located, Region};
use subs::Variable;

/// The canonicalization environment for a particular module.
pub struct Env {
    /// The module's path. Unqualified references to identifiers and variant names are assumed
    /// to be relative to this path.
    pub home: Box<str>,

    /// Problems we've encountered along the way, which will be reported to the user at the end.
    pub problems: Vec<Problem>,

    /// Variants either declared in this module, or imported.
    pub variants: ImMap<Symbol, Located<Box<str>>>,
}

impl Env {
    pub fn new(home: Box<str>, declared_variants: ImMap<Symbol, Located<Box<str>>>) -> Env {
        Env {
            home,
            variants: declared_variants,
            problems: Vec::new(),
        }
    }

    pub fn problem(&mut self, problem: Problem) {
        self.problems.push(problem)
    }
}
