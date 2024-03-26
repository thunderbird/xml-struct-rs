/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
#[xml_struct(text)]
enum UnitVariants {
    Foo,
    Bar,
    Baz,
    FooBar,
}

fn main() -> Result<(), xml_struct::Error> {
    let bytes: Vec<u8> = Vec::new();
    let mut writer = quick_xml::writer::Writer::new(bytes);

    let content = UnitVariants::Bar;
    content.serialize_as_element(&mut writer, "foo")?;

    Ok(())
}
