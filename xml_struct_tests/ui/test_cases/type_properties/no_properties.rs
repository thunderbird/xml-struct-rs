/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
struct NoAttributes;

// There's no clear use case for this pattern at this time, but it shouldn't
// error anyhow.
#[derive(XmlSerialize)]
#[xml_struct()]
struct EmptyAttribute;

fn main() -> Result<(), xml_struct::Error> {
    let bytes: Vec<u8> = Vec::new();
    let mut writer = quick_xml::writer::Writer::new(bytes);

    let content = NoAttributes;
    content.serialize_as_element(&mut writer, "foo")?;

    let content = EmptyAttribute;
    content.serialize_as_element(&mut writer, "foo")?;

    Ok(())
}
