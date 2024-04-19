#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    C, H, O, B, Br, Cl, F, Mg, N, P, S,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // str of the element symbol.
        match self {
            Element::B => Ok("B"),
            Element::Br => Ok("Br"),
            Element::C => Ok("C"),
            Element::Cl => Ok("Cl"),
            _ => ChemError,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChemError {
    EmptyMolecule,
    LockedMolecule,
    InvalidBond,
    UnlockedMolecule,
}

pub type ChemResult<T> = Result<T, ChemError>;