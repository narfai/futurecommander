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

use crate::{
    context::{
        ContextError,
        ContextType
    }
};


#[derive(Clone)]
pub struct ContextString {
    inner: String
}

impl From<String> for ContextString {
    fn from(s : String) -> ContextString {
        ContextString {
            inner: s
        }
    }
}

impl ContextType for ContextString {
    fn to_bool(&self) -> Result<bool, ContextError> {
        if self.inner == "1" {
            Ok(true)
        } else if self.inner == "0" {
            Ok(false)
        } else {
            Err(ContextError::CannotCast("String".to_string(), "bool".to_string()))
        }
    }

    fn to_string(&self) -> Result<String, ContextError> {
        Ok(self.inner.clone())
    }

    fn box_clone(&self) -> Box<dyn ContextType> {
        Box::new(self.clone())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        context::{
            ContextContainer
        }
    };

    #[test]
    fn fill_and_query_context_with_context_strings(){
        let mut context = ContextContainer::default();
        let value_a = "valueA".to_string();
        let value_b = "valueB".to_string();
        context.set("keyA", Box::new(ContextString::from(value_a.clone())));
        context.set("keyB", Box::new(ContextString::from(value_b.clone())));

        assert_eq!(context.get("keyA").unwrap().to_string().unwrap(), value_a);
        assert_eq!(context.get("keyB").unwrap().to_string().unwrap(), value_b);
        assert!(context.debug_keys().contains(&"keyA".to_string()));
        assert!(context.debug_keys().contains(&"keyB".to_string()));
    }
}
