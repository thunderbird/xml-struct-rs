/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;
use xml_struct_tests::{serialize_value_as_element, serialize_value_children};

#[derive(XmlSerialize)]
struct UnitStruct;

const BAR_PREFIX: &str = "bar";
const BAZ_NAME: &str = "http://baz.example/";

#[derive(XmlSerialize)]
#[xml_struct(default_ns = "http://foo.example/", ns = (BAR_PREFIX, "http://bar.example/"), ns = ("baz", BAZ_NAME))]
struct UnitStructWithNamespaces;

#[test]
fn unit_struct() {
    let content = UnitStruct;

    let expected = "";

    let actual = serialize_value_children(content).expect("Failed to write unit struct");

    assert_eq!(actual, expected, "Unit struct should have no content");
}

#[test]
fn unit_struct_as_element() {
    let content = UnitStruct;

    let expected = "<foo/>";

    let actual = serialize_value_as_element(content, "foo").expect("Failed to write unit struct");

    assert_eq!(
        actual, expected,
        "Unit struct should serialize as empty element"
    );
}

#[test]
fn unit_struct_with_namespaces() {
    let content = UnitStructWithNamespaces;

    let expected = "";

    let actual = serialize_value_children(content).expect("Failed to write unit struct");

    assert_eq!(actual, expected, "Unit struct should have no content");
}

#[test]
fn unit_struct_with_namespaces_as_element() {
    let content = UnitStructWithNamespaces;

    let expected = r#"<foo xmlns="http://foo.example/" xmlns:bar="http://bar.example/" xmlns:baz="http://baz.example/"/>"#;

    let actual = serialize_value_as_element(content, "foo").expect("Failed to write unit struct");

    assert_eq!(
        actual, expected,
        "Unit struct should serialize as empty element with namespace attributes"
    );
}
