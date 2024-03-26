/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
#[xml_struct(ns = ("foo", "http://foo.example/"))]
struct SingleNamespace;

#[derive(XmlSerialize)]
#[xml_struct(ns = ("foo", "http://foo.example/"), ns = ("bar", "http://bar.example/"))]
struct MultipleNamespaces;

const FOO_PREFIX: &str = "foo";
const BAR_NAME: &str = "http://bar.example/";

#[derive(XmlSerialize)]
#[xml_struct(ns = (FOO_PREFIX, "http://foo.example/"), ns = ("bar", BAR_NAME))]
struct ConstsInNamespaceDecls;

#[derive(XmlSerialize)]
#[xml_struct(default_ns = "http://default.example/")]
struct DefaultNamespace;

#[derive(XmlSerialize)]
#[xml_struct(default_ns = "http://default.example/", ns = ("foo", "http://foo.example/"), ns = ("bar", BAR_NAME))]
struct DefaultNamespaceWithOthers;

fn main() -> Result<(), xml_struct::Error> {
    let bytes: Vec<u8> = Vec::new();
    let mut writer = quick_xml::writer::Writer::new(bytes);

    let content = SingleNamespace;
    content.serialize_as_element(&mut writer, "foo")?;

    let content = MultipleNamespaces;
    content.serialize_as_element(&mut writer, "foo")?;

    let content = ConstsInNamespaceDecls;
    content.serialize_as_element(&mut writer, "foo")?;

    let content = DefaultNamespace;
    content.serialize_as_element(&mut writer, "foo")?;

    let content = DefaultNamespaceWithOthers;
    content.serialize_as_element(&mut writer, "foo")?;

    Ok(())
}
