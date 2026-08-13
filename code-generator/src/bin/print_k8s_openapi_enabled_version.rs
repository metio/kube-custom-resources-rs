// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use code_generator::k8s_openapi_enabled_version;

fn main() {
    println!("{}", k8s_openapi_enabled_version());
}
