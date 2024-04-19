use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    C, H, O, B, Br, Cl, F, Mg, N, P, S,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Element::B => "B",
            Element::Br => "Br",
            Element::C => "C",
            Element::Cl => "Cl",
            Element::F => "F",
            Element::H => "H",
            Element::Mg => "Mg",
            Element::N => "N",
            Element::O => "O",
            Element::P => "P",
            Element::S => "S",
        };
        write!(f, "{}", symbol)
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