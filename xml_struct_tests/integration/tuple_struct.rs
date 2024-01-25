/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use quick_xml::events::{BytesText, Event};
use xml_struct::XmlSerialize;
use xml_struct_tests::{serialize_value_as_element, serialize_value_children};

#[derive(XmlSerialize)]
struct TupleStruct(ChildType, String);

#[derive(XmlSerialize)]
#[xml_struct(default_ns = "http://foo.example/this_ns", ns = ("other_ns", "http://bar.example/other_ns"))]
struct TupleStructWithNamespaces(ChildType, String);

struct ChildType {
    _grandchild: &'static str,
}

impl ChildType {
    #[allow(dead_code)]
    fn serialize_child_nodes<W>(
        &self,
        _writer: &mut quick_xml::Writer<W>,
    ) -> Result<(), xml_struct::Error>
    where
        W: std::io::Write,
    {
        panic!("`XmlSerialize` calls should not dispatch non-trait functions");
    }
}

// We explicitly implement `XmlSerialize` for this type in a way which doesn't
// match the default in order to verify that `ChildType`'s implementation is
// used rather than some other magic.
impl XmlSerialize for ChildType {
    fn serialize_child_nodes<W>(
        &self,
        writer: &mut quick_xml::Writer<W>,
    ) -> Result<(), xml_struct::Error>
    where
        W: std::io::Write,
    {
        writer.write_event(Event::Text(BytesText::new("bare text child node")))?;

        Ok(())
    }
}

#[test]
fn tuple_struct() {
    let content = TupleStruct(
        ChildType {
            _grandchild: "this text shouldn't show up",
        },
        String::from("bare text node"),
    );

    let expected = "bare text child nodebare text node";

    let actual = serialize_value_children(content).expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Struct fields should each be serialized as a child node"
    );
}

#[test]
fn tuple_struct_as_element() {
    let content = TupleStruct(
        ChildType {
            _grandchild: "this text shouldn't show up",
        },
        String::from("bare text node"),
    );

    let expected = r#"<parent>bare text child nodebare text node</parent>"#;

    let actual = serialize_value_as_element(content, "parent").expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Struct should be serialized as element with fields as attribute and children as appropriate"
    );
}

#[test]
fn tuple_struct_with_namespaces() {
    let content = TupleStructWithNamespaces(
        ChildType {
            _grandchild: "this text shouldn't show up",
        },
        String::from("bare text node"),
    );

    let expected = "bare text child nodebare text node";

    let actual = serialize_value_children(content).expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Struct fields should each be serialized as a child node"
    );
}

#[test]
fn tuple_struct_with_namespaces_as_element() {
    let content = TupleStructWithNamespaces(
        ChildType {
            _grandchild: "this text shouldn't show up",
        },
        String::from("bare text node"),
    );

    let expected = r#"<parent xmlns="http://foo.example/this_ns" xmlns:other_ns="http://bar.example/other_ns">bare text child nodebare text node</parent>"#;

    let actual = serialize_value_as_element(content, "parent").expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Struct should be serialized with namespaces as attributes"
    );
}
