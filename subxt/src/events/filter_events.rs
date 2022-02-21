// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Filtering individual events from subscriptions.

use codec::Decode;
use super::{ Events, EventSubscription };
use crate::{ Config, BasicError };

/// A stream which returns zero or more of the filtered event
/// types back.
pub struct FilterEvents<'a, T: Config, Evs: 'static, Filter> {
    sub: EventSubscription<'a, T, Evs>,
    filter: std::marker::PhantomData<Filter>,
}

impl <'a, T: Config, Evs, Filter: EventFilter> FilterEvents<'a, T, Evs, Filter> {
    pub (crate) fn new(sub: EventSubscription<'a, T, Evs>) -> Self {
        Self {
            sub,
            filter: std::marker::PhantomData
        }
    }
}

/// This trait is implemented for tuples of Event types; any
/// such tuple can be used to filter an event subscription to return
/// only the specified events.
pub trait EventFilter {
    /// The type we'll be handed back from filtering.
    type ReturnType;
    /// Filter the events based on the type implementing this trait.
    fn filter<T: Config, Evs: Decode>(events: &Events<'_, T, Evs>) -> Result<Self::ReturnType, BasicError>;
}