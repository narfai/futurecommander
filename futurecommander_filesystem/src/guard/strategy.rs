use crate::{
    guard::Capability,
    operation::{
        CreateStrategy,
        MoveStrategy,
        CopyStrategy,
        RemoveStrategy
    }
};

impl From<CreateStrategy> for Option<Capability> {
    fn from(strategy: CreateStrategy) -> Self {
        use CreateStrategy::*;
        match strategy {
            FileCreationOverwrite => Some(Capability::Overwrite),
            DirectoryCreationOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl From<MoveStrategy> for Option<Capability> {
    fn from(strategy: MoveStrategy) -> Self {
        use MoveStrategy::*;
        match strategy {
            DirectoryMerge => Some(Capability::Merge),
            FileOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl From<CopyStrategy> for Option<Capability> {
    fn from(strategy: CopyStrategy) -> Self {
        use CopyStrategy::*;
        match strategy {
            DirectoryMerge => Some(Capability::Merge),
            FileOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl From<RemoveStrategy> for Option<Capability> {
    fn from(strategy: RemoveStrategy) -> Self {
        use RemoveStrategy::*;
        match strategy {
            RecursiveDirectoryRemoval => Some(Capability::Recursive),
            _ => None
        }
    }
}