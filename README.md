# `xml_struct`

The `xml_struct` crate is intended to provide simple, flexible, low-boilerplate
serialization of Rust data structures to XML.

## Limitations

In its current iteration, this project makes several behavioral assumptions
which make it unsuitable for general use. Primary among these are that
transformation of field/structure names to XML tag names is not configurable
(all names are transformed to PascalCase) and whether fields are serialized as
XML elements or attributes by default is not configurable.

Deserialization is likewise not supported at this time.

Due to the listed limitations, `xml_struct` is not currently published to
crates.io and no support is offered at this time. These limitations may be
addressed at a later time if there is general interest in this crate or if
workload allows.

For general-purpose XML serialization or deserialization, one of these crates
may better suit your needs at this time:

- [`xmlserde`](https://github.com/imjeremyhe/xmlserde)
- [`yaserde`](https://github.com/media-io/yaserde)
