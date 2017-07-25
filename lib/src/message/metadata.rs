// git-dit - the distributed issue tracker for git
// Copyright (C) 2016, 2017 Matthias Beyer <mail@beyermatthias.de>
// Copyright (C) 2016, 2017 Julian Ganz <neither@nut.email>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Metadata spcification
//!
//! This module provides a type for convenient metadata specification as well as
//! well as specifications for some dit metadata tags.
//!

use message::accumulation::AccumulationPolicy;


/// Metadata specification
///
/// Use instances of this type for specifying the names and accumulation rules
/// of pieces of metadata.
///
#[derive(Clone)]
pub struct MetadataSpecification<'k> {
    pub key: &'k str,
    pub accumulation: AccumulationPolicy,
}

