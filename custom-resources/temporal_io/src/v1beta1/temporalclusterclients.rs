// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/alexandrevilain/temporal-operator/temporal.io/v1beta1/temporalclusterclients.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TemporalClusterClientSpec defines the desired state of ClusterClient.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "temporal.io", version = "v1beta1", kind = "TemporalClusterClient", plural = "temporalclusterclients")]
#[kube(namespaced)]
#[kube(status = "TemporalClusterClientStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TemporalClusterClientSpec {
    /// Reference to the temporal cluster the client will get access to.
    #[serde(rename = "clusterRef")]
    pub cluster_ref: TemporalClusterClientClusterRef,
}

/// Reference to the temporal cluster the client will get access to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalClusterClientClusterRef {
    /// The name of the temporal object to reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The namespace of the temporal object to reference.
    /// Defaults to the namespace of the requested resource if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// TemporalClusterClientStatus defines the observed state of ClusterClient.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalClusterClientStatus {
    /// Reference to the Kubernetes Secret containing the certificate for the client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<TemporalClusterClientStatusSecretRef>,
    /// ServerName is the hostname returned by the certificate.
    #[serde(rename = "serverName")]
    pub server_name: String,
}

/// Reference to the Kubernetes Secret containing the certificate for the client.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalClusterClientStatusSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

