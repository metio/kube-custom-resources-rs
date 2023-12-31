<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# Kubernetes Custom Resource Bindings for Rust [![Chat](https://img.shields.io/badge/matrix-%23talk.metio:matrix.org-brightgreen.svg?style=social&label=Matrix)](https://matrix.to/#/#talk.metio:matrix.org)

This repository contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) generated with [kopium](https://github.com/kube-rs/kopium)

Feel free to add your own CRD to the [catalog](https://github.com/metio/kube-custom-resources-rs/blob/main/code-generator/src/catalog.rs)!

## Installation

```toml
[dependencies]
kube-custom-resources-rs = { version = "<version>", features = ["<features>"] }
```

Replace `<version>` with the latest available [release](https://crates.io/crates/kube-custom-resources-rs).

### Features

Each group of a Kubernetes custom resource has a corresponding Cargo feature in this crate. The group of a custom resource can be seen in the `apiVersion` field of a resource, e.g.:

```yaml
apiVersion: chaos-mesh.org/v1alpha1
kind: PodNetworkChaos
metadata:
  ...
```

In the above example, `chaos-mesh.org` is the group and `v1alpha1` is the version. Since Cargo imposes certain rules on how features can be named, `.`, `-`, and `/` are all mapped to `_`. Therefore, the feature that contains the custom resource from the example above is called `chaos_mesh_org` and can be enabled like this:

```toml
[dependencies]
kube-custom-resources-rs = { version = "<version>", features = ["chaos_mesh_org"] }
```

Each version within a group has a corresponding module in that feature, e.g. there is a module called `v1alpha1` in the feature `chaos_mesh_org`.

Take a look at the [docs](https://docs.rs/kube-custom-resources-rs/latest/kube_custom_resources_rs/) to see all available features and the group/version/kinds they contain.

## Versioning

This crate uses a calendar based versioning scheme because resources in Kubernetes are versioned themselves.

Updates to all CRDs are fetched [automatically](https://github.com/metio/kube-custom-resources-rs/blob/main/.github/workflows/update-crds.yml) and released on the [first of each month](https://github.com/metio/kube-custom-resources-rs/blob/main/.github/workflows/release.yml) if any changes were detected.

## Usage

The generated Rust code can be used as a [kube::Resource](https://docs.rs/kube/*/kube/trait.Resource.html) similar to this:

```rust
let api: Api<PodNetworkChaos> = Api::default_namespaced(client);
let resource = PodNetworkChaos::new("example", PodNetworkChaosSpec::default());
println!("doc: {:?}", issuer);
```

Take a look at the [kube-rs documentation](https://docs.rs/kube/) for more information.
