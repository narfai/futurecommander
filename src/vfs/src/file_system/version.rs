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

use std::sync::atomic::{AtomicUsize, Ordering};

static VIRTUAL_VERSION: AtomicUsize = AtomicUsize::new(0);
static REAL_VERSION: AtomicUsize = AtomicUsize::new(0);


pub struct VirtualVersion;


impl VirtualVersion {
    pub fn increment() -> usize {
        VIRTUAL_VERSION.fetch_add(1, Ordering::SeqCst);
        VIRTUAL_VERSION.load(Ordering::SeqCst)
    }

    pub fn get() -> usize {
        VIRTUAL_VERSION.load(Ordering::SeqCst)
    }
}

pub struct RealVersion;


impl RealVersion {
    pub fn increment() -> usize {
        REAL_VERSION.fetch_add(1, Ordering::SeqCst);
        VIRTUAL_VERSION.load(Ordering::SeqCst)
    }

    pub fn get() -> usize {
        REAL_VERSION.load(Ordering::SeqCst)
    }
}
