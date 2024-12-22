/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `tests.testkube.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;
