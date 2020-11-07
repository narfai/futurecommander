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

use std::{
    fmt::Debug
};

use serde::{Serialize, Deserialize};

use futurecommander_representation::Kind;

use crate::{
    event::{
        VirtualEvent,
        RealEvent,
        *
    }
};

#[typetag::serde(tag = "type")]
pub trait SerializableEvent : Debug {
    fn serializable(&self) -> Box<dyn SerializableEvent>;
    fn virt(&self) -> VirtualEvent;
    fn real(&self) -> RealEvent;
}

#[typetag::serde]
impl SerializableEvent for CopyEvent {
    fn serializable(&self) -> Box<dyn SerializableEvent> {
        Box::new(self.clone())
    }
    fn virt(&self) -> VirtualEvent { VirtualEvent(Box::new(self.clone())) }
    fn real(&self) -> RealEvent { RealEvent(Box::new(self.clone())) }
}

#[typetag::serde]
impl SerializableEvent for CreateEvent {
    fn serializable(&self) -> Box<dyn SerializableEvent> {
        Box::new(self.clone())
    }
    fn virt(&self) -> VirtualEvent { VirtualEvent(Box::new(self.clone())) }
    fn real(&self) -> RealEvent { RealEvent(Box::new(self.clone())) }
}

#[typetag::serde]
impl SerializableEvent for MoveEvent {
    fn serializable(&self) -> Box<dyn SerializableEvent> {
        Box::new(self.clone())
    }
    fn virt(&self) -> VirtualEvent { VirtualEvent(Box::new(self.clone())) }
    fn real(&self) -> RealEvent { RealEvent(Box::new(self.clone())) }
}

#[typetag::serde]
impl SerializableEvent for RemoveEvent {
    fn serializable(&self) -> Box<dyn SerializableEvent> {
        Box::new(self.clone())
    }
    fn virt(&self) -> VirtualEvent { VirtualEvent(Box::new(self.clone())) }
    fn real(&self) -> RealEvent { RealEvent(Box::new(self.clone())) }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SerializableKind {
    File,
    Directory,
    Unknown
}

impl From<Kind> for SerializableKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::File => SerializableKind::File,
            Kind::Directory => SerializableKind::Directory,
            Kind::Unknown => SerializableKind::Unknown,
        }
    }
}

impl From<SerializableKind> for Kind {
    fn from(kind: SerializableKind) -> Self {
        match kind {
            SerializableKind::File => Kind::File,
            SerializableKind::Directory => Kind::Directory,
            SerializableKind::Unknown => Kind::Unknown,
        }
    }
}
