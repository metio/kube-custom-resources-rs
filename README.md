<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# Kubernetes Custom Resource Bindings for Rust [![Chat](https://img.shields.io/badge/matrix-%23talk.metio:matrix.org-brightgreen.svg?style=social&label=Matrix)](https://matrix.to/#/#talk.metio:matrix.org)

This repository contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium) and updated weekly.

Feel free to add your own CRD to the [catalog](code-generator/src/catalog.rs)!

## Installation

```toml
[dependencies]
kube-custom-resources-rs = { version = "<version>", features = ["<features>"] }
```

Replace `<version>` with the latest available [release](https://crates.io/crates/kube-custom-resources-rs).

### Features

Each group/version of a Kubernetes custom resource has a corresponding Cargo feature in this crate. The group/version of a custom resource can be seen in the `apiVersion` field of a resource, e.g.:

```yaml
apiVersion: cert-manager.io/v1
kind: Issuer
metadata:
  ...
```

Since Cargo imposes certain rules on how features can be named, `.`, `-`, and `/` are all mapped to `_`. Therefore, the feature that contains the custom resource from the example above is called `cert_manager_io_v1` and can be enabled like this:

```toml
[dependencies]
kube-custom-resources-rs = { version = "<version>", features = ["cert_manager_io_v1"] }
```

## Versioning

This crate uses a calendar based versioning scheme because resources in Kubernetes are versioned themselves.

Updates to all CRDs are fetched on friday and released a day later on saturday if any changes were detected.

## Usage

The generated Rust code can be used as a [kube::Resource](https://docs.rs/kube/*/kube/trait.Resource.html) similar to this:

```rust
let issuers: Api<Issuer> = Api::default_namespaced(client);
let issuer = Issuer::new("example", IssuerSpec::default());
println!("doc: {:?}", issuer);
```

Take a look at the [kube-rs documentation](https://docs.rs/kube/) for more information.
