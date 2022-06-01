use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum CanHaveChildren {
    Yes,
    No,
}

impl Default for CanHaveChildren {
    fn default() -> Self {
        Self::Yes
    }
}

impl CanHaveChildren {
    pub fn bool(&self) -> bool {
        match self {
            Self::Yes => true,
            Self::No => false
        }
    }
}