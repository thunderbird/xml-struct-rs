/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This crate provides a mechanism for serializing Rust data structures as
//! well-formed XML with a minimum of boilerplate.
//!
//! Consumers can provide manual implementations of the [`XmlSerialize`] and
//! [`XmlSerializeAttr`] traits if desired, but the primary intent of this crate
//! is to provide automated derivation of these traits in order to facilitate
//! serialization of complex XML structures.
//!
//! # Limitations
//!
//! At present, derived implementations of these traits are designed to handle
//! the specific case of Microsoft Exchange Web Services. As such, all XML
//! elements and attributes are named in PascalCase and certain behaviors are
//! not supported (such as serializing enum variants without enclosing XML
//! elements derived from the variant name).
//!
//! There is also currently no provision for deserialization from XML, as the
//! support offered by `quick_xml`'s serde implementation has been found to be
//! sufficient for the time being.
//!
//! In recognition of these limitations, this crate should not be published to
//! crates.io at this time. If a generalized implementation generates interest
//! or is thought to have merit, these limitations may be addressed at a later
//! time.

mod impls;
mod tests;

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};
use thiserror::Error;

pub use derive::*;

/// A data structure which can be serialized as XML content nodes.
pub trait XmlSerialize {
    /// Serializes this value's child content nodes within an enclosing XML element.
    fn serialize_as_element<W>(&self, writer: &mut Writer<W>, name: &str) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Start(BytesStart::new(name)))?;

        self.serialize_child_nodes(writer)?;

        writer.write_event(Event::End(BytesEnd::new(name)))?;

        Ok(())
    }

    /// Serializes the child content nodes of this value.
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write;
}

/// A data structure which can be serialized as the value of an XML attribute.
pub trait XmlSerializeAttr {
    /// Serializes this value as the value of an XML attribute.
    fn serialize_as_attribute(&self, start_tag: &mut BytesStart, name: &str);
}

#[derive(Debug, Error)]
/// An error generated during the XML serialization process.
pub enum Error {
    #[error("failed to process XML document")]
    Xml {
        #[from]
        source: quick_xml::Error,
    },
}
