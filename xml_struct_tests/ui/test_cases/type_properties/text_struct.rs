/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
#[xml_struct(text)]
struct TextUnitStruct;

#[derive(XmlSerialize)]
#[xml_struct(text)]
struct TextTupleStruct(String);

#[derive(XmlSerialize)]
#[xml_struct(text)]
struct TextStruct {
    value: String,
}

fn main() {}
