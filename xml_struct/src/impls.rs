/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This module provides implementations of serialization for common types from
//! the standard library.

use quick_xml::{
    events::{BytesText, Event},
    Writer,
};

use crate::{Error, XmlSerialize, XmlSerializeAttr};

impl XmlSerialize for str {
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Text(BytesText::new(self)))?;

        Ok(())
    }
}

impl<T> XmlSerialize for &T
where
    T: AsRef<str>,
{
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Text(BytesText::new(self.as_ref())))?;

        Ok(())
    }
}

impl XmlSerialize for String {
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Text(BytesText::new(self.as_str())))?;

        Ok(())
    }
}

impl XmlSerialize for &str {
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Text(BytesText::new(self)))?;

        Ok(())
    }
}

impl<T> XmlSerialize for Option<T>
where
    T: XmlSerialize,
{
    fn serialize_as_element<W>(&self, writer: &mut Writer<W>, name: &str) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        match self {
            Some(value) => <T as XmlSerialize>::serialize_as_element(value, writer, name),
            None => Ok(()),
        }
    }

    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        match self {
            Some(value) => <T as XmlSerialize>::serialize_child_nodes(value, writer),
            None => Ok(()),
        }
    }
}

impl<T> XmlSerialize for Vec<T>
where
    T: XmlSerialize,
{
    fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        if self.is_empty() {
            return Ok(());
        }

        for value in self {
            <T as XmlSerialize>::serialize_child_nodes(value, writer)?;
        }

        Ok(())
    }
}

impl XmlSerializeAttr for str {
    fn serialize_as_attribute(&self, start_tag: &mut quick_xml::events::BytesStart, name: &str) {
        start_tag.push_attribute((name, self));
    }
}

impl<T> XmlSerializeAttr for &T
where
    T: AsRef<str>,
{
    fn serialize_as_attribute(&self, start_tag: &mut quick_xml::events::BytesStart, name: &str) {
        start_tag.push_attribute((name, self.as_ref()));
    }
}

impl XmlSerializeAttr for String {
    fn serialize_as_attribute(&self, start_tag: &mut quick_xml::events::BytesStart, name: &str) {
        start_tag.push_attribute((name, self.as_str()));
    }
}

impl XmlSerializeAttr for &str {
    fn serialize_as_attribute(&self, start_tag: &mut quick_xml::events::BytesStart, name: &str) {
        start_tag.push_attribute((name, *self));
    }
}

impl<T> XmlSerializeAttr for Option<T>
where
    T: XmlSerializeAttr,
{
    fn serialize_as_attribute(&self, start_tag: &mut quick_xml::events::BytesStart, name: &str) {
        match self {
            Some(value) => value.serialize_as_attribute(start_tag, name),
            None => (),
        }
    }
}

/// Implements serialization of a type as either an XML text node or attribute
/// value.
///
/// This is a convenience macro intended for implementing basic serialization of
/// primitive/standard library types. This is done per-type rather than
/// wholesale for `ToString` in order to avoid requiring that `Display` and
/// `XmlSerialize`/`XmlSerializeAttr` share a form.
macro_rules! impl_as_text_for {
    ($( $ty:ty ),*) => {
        $(
        impl XmlSerialize for $ty {
            fn serialize_child_nodes<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
            where
                W: std::io::Write,
            {
                let string = self.to_string();
                writer.write_event(Event::Text(BytesText::new(&string)))?;

                Ok(())
            }
        }

        impl XmlSerializeAttr for $ty {
            fn serialize_as_attribute(
                &self,
                start_tag: &mut quick_xml::events::BytesStart,
                name: &str,
            ) {
                start_tag.push_attribute((name, self.to_string().as_str()));
            }
        })*
    };
}

impl_as_text_for!(i8, u8, i16, u16, i32, u32, i64, u64);
