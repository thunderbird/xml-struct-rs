/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;
use xml_struct_tests::{serialize_value_as_element, serialize_value_children};

#[derive(XmlSerialize)]
enum MixedEnum {
    UnitVariant,
    StructVariant {
        some_field: String,
    },
    TupleVariant(&'static str),
    StructVariantWithAttributes {
        child_field: String,

        #[xml_struct(attribute)]
        attr_field: String,
    },
}

#[derive(XmlSerialize)]
enum AllUnitEnum {
    A,
    Two,
    Gamma,
}

#[derive(XmlSerialize)]
struct StructWithAllUnitEnumFields {
    child_field: String,
    enum_child: AllUnitEnum,

    #[xml_struct(ns_prefix = "foo")]
    enum_child_with_prefix: AllUnitEnum,
}

#[derive(XmlSerialize)]
#[xml_struct(variant_ns_prefix = "foo")]
enum EnumWithNamespacePrefix {
    SomeValue,
}

#[test]
fn mixed_enum_unit_variant() {
    let content = MixedEnum::UnitVariant;

    let expected = "<UnitVariant/>";
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Unit variants should serialize as an element with no content"
    );
}

#[test]
fn mixed_enum_unit_variant_as_element() {
    let content = MixedEnum::UnitVariant;

    let expected = "<parent_name><UnitVariant/></parent_name>";
    let actual =
        serialize_value_as_element(content, "parent_name").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Unit variants should serialize as a parented element with no content"
    );
}

#[test]
fn mixed_enum_struct_variant() {
    let content = MixedEnum::StructVariant {
        some_field: String::from("some content"),
    };

    let expected = "<StructVariant><SomeField>some content</SomeField></StructVariant>";
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Struct variants should serialize as an element with serialized fields as content"
    );
}

#[test]
fn mixed_enum_struct_variant_as_element() {
    let content = MixedEnum::StructVariant {
        some_field: String::from("some content"),
    };

    let expected =
        "<FooBar><StructVariant><SomeField>some content</SomeField></StructVariant></FooBar>";
    let actual =
        serialize_value_as_element(content, "FooBar").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Struct variants should serialize as a parented element with serialized fields as content"
    );
}

#[test]
fn mixed_enum_tuple_variant() {
    let content = MixedEnum::TupleVariant("something in a tuple");

    let expected = "<TupleVariant>something in a tuple</TupleVariant>";
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Tuple variants should serialize as an element with serialized fields as content"
    );
}

#[test]
fn mixed_enum_tuple_variant_as_element() {
    let content = MixedEnum::TupleVariant("something in a tuple");

    let expected = "<banana><TupleVariant>something in a tuple</TupleVariant></banana>";
    let actual =
        serialize_value_as_element(content, "banana").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Tuple variants should serialize as a parented element with serialized fields as content"
    );
}

#[test]
fn mixed_enum_struct_variant_with_attributes() {
    let content = MixedEnum::StructVariantWithAttributes {
        child_field: String::from("some child content"),
        attr_field: String::from("an attribute"),
    };

    let expected = r#"<StructVariantWithAttributes AttrField="an attribute"><ChildField>some child content</ChildField></StructVariantWithAttributes>"#;
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Attributes should be applied to the variant element"
    );
}

#[test]
fn mixed_enum_struct_variant_with_attributes_as_element() {
    let content = MixedEnum::StructVariantWithAttributes {
        child_field: String::from("some child content"),
        attr_field: String::from("an attribute"),
    };

    let expected = r#"<Arbitrary><StructVariantWithAttributes AttrField="an attribute"><ChildField>some child content</ChildField></StructVariantWithAttributes></Arbitrary>"#;
    let actual =
        serialize_value_as_element(content, "Arbitrary").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Attributes should be applied to the variant element rather than the parent"
    );
}

#[test]
fn all_unit_enum() {
    let content = AllUnitEnum::Two;

    let expected = "<Two/>";
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Unit variants should be serialized as an empty element"
    );
}

#[test]
fn all_unit_enum_as_element() {
    let content = AllUnitEnum::Two;

    let expected = r#"<foo><Two/></foo>"#;
    let actual =
        serialize_value_as_element(content, "foo").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Unit variants should be serialized as a parented empty element"
    );
}

#[test]
fn struct_with_all_unit_enum_fields() {
    let content = StructWithAllUnitEnumFields {
        child_field: String::from("this is a regular string field"),
        enum_child: AllUnitEnum::A,
        enum_child_with_prefix: AllUnitEnum::Gamma,
    };

    let expected =
        "<ChildField>this is a regular string field</ChildField><EnumChild><A/></EnumChild><foo:EnumChildWithPrefix><Gamma/></foo:EnumChildWithPrefix>";
    let actual = serialize_value_children(content).expect("Failed to serialize struct value");

    assert_eq!(
        actual, expected,
        "Unit enum fields should be serialized as empty elements"
    )
}

#[test]
fn struct_with_all_unit_enum_fields_as_element() {
    let content = StructWithAllUnitEnumFields {
        child_field: String::from("this is a regular string field"),
        enum_child: AllUnitEnum::A,
        enum_child_with_prefix: AllUnitEnum::Gamma,
    };

    let expected = r#"<TAGNAME><ChildField>this is a regular string field</ChildField><EnumChild><A/></EnumChild><foo:EnumChildWithPrefix><Gamma/></foo:EnumChildWithPrefix></TAGNAME>"#;
    let actual =
        serialize_value_as_element(content, "TAGNAME").expect("Failed to serialize struct value");

    assert_eq!(
        actual, expected,
        "Unit enum fields should be serialized as parented empty elements"
    )
}

#[test]
fn enum_with_namespace_prefix() {
    let content = EnumWithNamespacePrefix::SomeValue;

    let expected = "<foo:SomeValue/>";
    let actual = serialize_value_children(content).expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Enum variants should be serialized with specified prefix"
    );
}

#[test]
fn enum_with_namespace_prefix_as_element() {
    let content = EnumWithNamespacePrefix::SomeValue;

    let expected = "<outer_foo><foo:SomeValue/></outer_foo>";
    let actual =
        serialize_value_as_element(content, "outer_foo").expect("Failed to serialize enum value");

    assert_eq!(
        actual, expected,
        "Enum variants should be serialized with specified prefix"
    );
}
