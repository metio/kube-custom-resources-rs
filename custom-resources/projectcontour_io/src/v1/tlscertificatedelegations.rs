// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/projectcontour/contour/projectcontour.io/v1/tlscertificatedelegations.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TLSCertificateDelegationSpec defines the spec of the CRD
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "projectcontour.io", version = "v1", kind = "TLSCertificateDelegation", plural = "tlscertificatedelegations")]
#[kube(namespaced)]
#[kube(status = "TLSCertificateDelegationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TLSCertificateDelegationSpec {
    pub delegations: Vec<TLSCertificateDelegationDelegations>,
}

/// CertificateDelegation maps the authority to reference a secret
/// in the current namespace to a set of namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TLSCertificateDelegationDelegations {
    /// required, the name of a secret in the current namespace.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// required, the namespaces the authority to reference the
    /// secret will be delegated to.
    /// If TargetNamespaces is nil or empty, the CertificateDelegation'
    /// is ignored. If the TargetNamespace list contains the character, "*"
    /// the secret will be delegated to all namespaces.
    #[serde(rename = "targetNamespaces")]
    pub target_namespaces: Vec<String>,
}

/// TLSCertificateDelegationStatus allows for the status of the delegation
/// to be presented to the user.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TLSCertificateDelegationStatus {
    /// Conditions contains information about the current status of the HTTPProxy,
    /// in an upstream-friendly container.
    /// Contour will update a single condition, `Valid`, that is in normal-true polarity.
    /// That is, when `currentStatus` is `valid`, the `Valid` condition will be `status: true`,
    /// and vice versa.
    /// Contour will leave untouched any other Conditions set in this block,
    /// in case some other controller wants to add a Condition.
    /// If you are another controller owner and wish to add a condition, you *should*
    /// namespace your condition with a label, like `controller.domain.com\ConditionName`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

