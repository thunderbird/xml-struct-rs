/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::PathBuf;

#[test]
fn type_properties() {
    let base_path = test_case_base_path().join("type_properties");

    let t = trybuild::TestCases::new();
    t.pass(base_path.join("no_properties.rs"));
    t.pass(base_path.join("valid_namespaces.rs"));
    t.compile_fail(base_path.join("multiple_defaults.rs"));
    t.pass(base_path.join("text_enum.rs"));
    t.compile_fail(base_path.join("text_struct.rs"));
    t.compile_fail(base_path.join("text_enum_with_non_unit_variants.rs"));
    t.compile_fail(base_path.join("text_enum_with_namespaces.rs"));
    t.compile_fail(base_path.join("invalid_attributes.rs"));
}

fn test_case_base_path() -> PathBuf {
    PathBuf::from("ui/test_cases")
}
