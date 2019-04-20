/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

#[allow(unused_imports)]
use crate::operation::Operation;

use crate::{
    OperationError,
    VirtualFileSystem,
    RealFileSystem,
    hybrid::{
        Transaction
    }
};

#[derive(Debug, Default)]
pub struct HybridFileSystem {
    virtual_file_system: VirtualFileSystem,
    real_file_system: RealFileSystem,
    transaction: Transaction<RealFileSystem>
}

impl HybridFileSystem {
    pub fn vfs(&self) -> &VirtualFileSystem {
        &self.virtual_file_system
    }

    pub fn transaction(&self) -> &Transaction<RealFileSystem> {
        &self.transaction
    }

    pub fn mut_vfs(&mut self) -> &mut VirtualFileSystem {
        &mut self.virtual_file_system
    }

    pub fn mut_transaction(&mut self) -> &mut Transaction<RealFileSystem> {
        &mut self.transaction
    }

    pub fn reset(&mut self) {
        self.virtual_file_system.reset();
        self.transaction = Transaction::default();
    }

    pub fn apply(&mut self) -> Result<(), OperationError> {
        self.transaction.apply(&mut self.real_file_system)?;
        self.reset();
        Ok(())
    }
}
