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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualState {
    Exists, //Exists only in real FS
    ExistsVirtually, //Directly added
    ExistsThroughVirtualParent, //Indirectly added
    Replaced,
    NotExists, //Does not exists in virtual fs or real, indirectly or not
    Removed, //Does exists in real fs and should be deleted
    RemovedVirtually, //Does exists in virtual but is also virtually deleted
}
