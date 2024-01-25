/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;
use xml_struct_tests::{serialize_value_as_element, serialize_value_children};

#[derive(XmlSerialize)]
#[xml_struct(text)]
enum TextEnum {
    A,
    Two,
    Gamma,
}

#[derive(XmlSerialize)]
struct StructWithTextEnumFields {
    child_field: String,

    #[xml_struct(attribute)]
    string_attr: String,

    #[xml_struct(attribute)]
    enum_attr: TextEnum,

    enum_child: TextEnum,
}

#[test]
fn text_enum() {
    let content = TextEnum::Two;

    let expected = "Two";

    let actual = serialize_value_children(content).expect("Failed to write enum");

    assert_eq!(
        actual, expected,
        "Variants of text enums should be serialized as a text node"
    );
}

#[test]
fn text_enum_as_element() {
    let content = TextEnum::Two;

    let expected = r#"<foo>Two</foo>"#;

    let actual = serialize_value_as_element(content, "foo").expect("Failed to write enum");

    assert_eq!(
        actual, expected,
        "Variants of text enums should be serialized as a parented text node"
    );
}

#[test]
fn struct_with_text_enum_fields() {
    let content = StructWithTextEnumFields {
        child_field: String::from("this is a regular string field"),
        string_attr: String::from("this is a regular attr field"),
        enum_attr: TextEnum::Gamma,
        enum_child: TextEnum::A,
    };

    let expected =
        "<ChildField>this is a regular string field</ChildField><EnumChild>A</EnumChild>";

    let actual = serialize_value_children(content).expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Text enum fields should be serialized as text nodes"
    )
}

#[test]
fn struct_with_text_enum_fields_as_element() {
    let content = StructWithTextEnumFields {
        child_field: String::from("this is a regular string field"),
        string_attr: String::from("this is a regular attr field"),
        enum_attr: TextEnum::Gamma,
        enum_child: TextEnum::A,
    };

    let expected = r#"<namehere StringAttr="this is a regular attr field" EnumAttr="Gamma"><ChildField>this is a regular string field</ChildField><EnumChild>A</EnumChild></namehere>"#;

    let actual = serialize_value_as_element(content, "namehere").expect("Failed to write struct");

    assert_eq!(
        actual, expected,
        "Text enum attributes should be serialized as text values"
    )
}
