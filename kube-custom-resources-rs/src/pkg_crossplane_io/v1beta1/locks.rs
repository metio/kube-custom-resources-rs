// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/crossplane/crossplane/pkg.crossplane.io/v1beta1/locks.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// LockPackage is a package that is in the lock.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LockPackages {
    /// Dependencies are the list of dependencies of this package. The order of
    /// the dependencies will dictate the order in which they are resolved.
    pub dependencies: Vec<LockPackagesDependencies>,
    /// Name corresponds to the name of the package revision for this package.
    pub name: String,
    /// Source is the OCI image name without a tag or digest.
    pub source: String,
    /// Type is the type of package. Can be either Configuration or Provider.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Version is the tag or digest of the OCI image.
    pub version: String,
}

/// A Dependency is a dependency of a package in the lock.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LockPackagesDependencies {
    /// Constraints is a valid semver range or a digest, which will be used to select a valid
    /// dependency version.
    pub constraints: String,
    /// Package is the OCI image name without a tag or digest.
    pub package: String,
    /// Type is the type of package. Can be either Configuration or Provider.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Status of the Lock.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LockStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

