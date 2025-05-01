<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# Contributor Guide

:+1::tada: Thanks a lot for taking your time to contribute! :tada::+1:

## Adding new CRDs

To add new custom resource definitions to this project, please follow the steps outlined below. If anything is unclear or does not work on your computer, please don't hesitate to reach out or just create a partial pull request on GitHub with your current progress.

### 0. Required Software

To run all the commands mentioned below, you will need the following software installed on your computer:

- `rust`/`cargo`: https://www.rust-lang.org/tools/install
- `kopium`: https://github.com/kube-rs/kopium
- `yq`: https://github.com/mikefarah/yq
- Some POSIX compliant shell like [bash](https://www.gnu.org/software/bash/) or [zsh](https://www.zsh.org/).

### 1. Add CRD to catalog

All CRDs of this project are managed in the [catalog](https://github.com/metio/kube-custom-resources-rs/blob/main/code-generator/src/catalog.rs) which contains a long list of projects along with the CRDs they are producing. We try to sort this list alphabetically to make it easier finding things, but this is not a hard requirement for your contribution. Each entry requires the following details:

- `project_name`: The organization and name of a project, e.g. `prometheus-operator/prometheus-operator` is used for the project at https://github.com/prometheus-operator/prometheus-operator.
- `license`: The SPDX license identifier for the CRD files. This is usually the same license as the project, and the catalog file already contains constants for the most common licenses.
- `urls`: The list of URLs where CRDs are located. It does not matter if that file contains other Kubernetes resources, our tooling will only extract CRDs from those files.

### 2. Generate Custom Resources

Once you have added all CRDs to the catalog, call the following shell script from the root of this project:

```console
$ ./code-generator/generate.sh <project_name>
```

The `<project_name>` argument is the same value you added to the catalog in step 1, e.g.:

```console
$ ./code-generator/generate.sh prometheus-operator/prometheus-operator
```

If no `<project_name>` argument was given, code for all CRDs in the catalog will be generated.

### 3. Fix Cargo Warnings

The generated code might trigger Cargo warnings, fix those automatically with:

```console
$ ./code-generator/fix-cargo-warnings.sh <feature_name>
```

### 4. Adjust Output

Some CRDs cannot be converted to Rust code at all. In those cases, add an `.ignore` file next to the downloaded `.yaml` file in the catalog and write the reason why this CRD was ignored into the file, e.g.:

```
CRD has no spec field
```

### 5. Open Pull Request

Commit your changes, push them into your fork and open a pull request. Don't worry if certain steps did not work on your machine, we will sort them out during review. Thanks again for contributing :tada::+1:
