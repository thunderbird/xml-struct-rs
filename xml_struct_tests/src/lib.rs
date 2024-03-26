/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use quick_xml::Writer;
use thiserror::Error;
use xml_struct::XmlSerialize;

pub fn serialize_value_as_element<T>(value: T, root_name: &str) -> Result<String, TestError>
where
    T: XmlSerialize,
{
    let buf = Vec::default();
    let mut writer = Writer::new(buf);

    value.serialize_as_element(&mut writer, root_name)?;

    let out = String::from_utf8(writer.into_inner())?;

    Ok(out)
}

pub fn serialize_value_children<T>(value: T) -> Result<String, TestError>
where
    T: XmlSerialize,
{
    let buf = Vec::default();
    let mut writer = Writer::new(buf);

    value.serialize_child_nodes(&mut writer)?;

    let out = String::from_utf8(writer.into_inner())?;

    Ok(out)
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TestError {
    #[error("error in processing XML document")]
    XmlStruct(#[from] xml_struct::Error),

    #[error("serialization produced invalid UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}
