/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
#[xml_struct(text, default_ns = "http://foo.example/")]
enum UnitVariantsWithDefaultNamespace {
    Foo,
    Bar,
    Baz,
    FooBar,
}

#[derive(XmlSerialize)]
#[xml_struct(default_ns = "http://foo.example/", text)]
enum UnitVariantsWithDefaultNamespaceInDifferentOrder {
    Foo,
    Bar,
    Baz,
    FooBar,
}

#[derive(XmlSerialize)]
#[xml_struct(text, ns = ("foo", "http://foo.example/"))]
enum UnitVariantsWithNamespace {
    Foo,
    Bar,
    Baz,
    FooBar,
}

#[derive(XmlSerialize)]
#[xml_struct(ns = ("foo", "http://foo.example/"), text)]
enum UnitVariantsWithNamespaceInDifferentOrder {
    Foo,
    Bar,
    Baz,
    FooBar,
}

fn main() {}
