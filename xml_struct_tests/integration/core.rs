/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;
use xml_struct_tests::serialize_value_as_element;

#[test]
fn r#bool() {
    // Test `true` as a content node.
    let content = true;

    let expected = "<element>true</element>";
    let actual = serialize_value_as_element(content, "element").expect("Failed to serialize value");

    assert_eq!(
        actual, expected,
        "`true` should be serialized as string value 'true'"
    );

    // Test `false` as a content node.
    let content = false;

    let expected = "<element>false</element>";
    let actual = serialize_value_as_element(content, "element").expect("Failed to serialize value");

    assert_eq!(
        actual, expected,
        "`false` should be serialized as string value 'false'"
    );

    // Define a struct with a boolean attribute.
    #[derive(XmlSerialize)]
    struct Element {
        #[xml_struct(attribute)]
        content: bool,
    }

    // Test `true` as an attribute value.
    let content = Element { content: true };

    let expected = r#"<element Content="true"/>"#;
    let actual = serialize_value_as_element(content, "element").expect("Failed to serialize value");

    assert_eq!(
        actual, expected,
        "`true` should be serialized as string value 'true'"
    );

    // Test `false` as an attribute value.
    let content = Element { content: false };

    let expected = r#"<element Content="false"/>"#;
    let actual = serialize_value_as_element(content, "element").expect("Failed to serialize value");

    assert_eq!(
        actual, expected,
        "`false` should be serialized as string value 'false'"
    );
}
