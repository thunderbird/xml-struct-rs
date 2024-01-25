/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

#[derive(XmlSerialize)]
#[xml_struct(defaut_ns = "http://foo.example/", ns = ("bar", "http://bar.example/"))]
struct MisspelledNameValueAttribute;

#[derive(XmlSerialize)]
#[xml_struct(everybody_loves_xml)]
struct UnrecognizedPathAttribute;

fn main() {}
