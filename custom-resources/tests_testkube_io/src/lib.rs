/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# tests_testkube_io

Custom resources in this crate belong to the `tests.testkube.io` group. The following versions and custom resources are available:

## tests.testkube.io/v1
- `Script`
- `TestExecution`
- `Test`
- `TestSource`
- `TestSuiteExecution`
- `TestSuite`
- `TestTrigger`
## tests.testkube.io/v2
- `Script`
- `Test`
- `TestSuite`
## tests.testkube.io/v3
- `Test`
- `TestSuite`
*/
pub mod v1;
pub mod v2;
pub mod v3;
