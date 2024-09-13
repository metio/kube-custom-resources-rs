// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/managementclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ManagementClusterSpec defines the desired state of a ManagementCluster
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "ManagementCluster", plural = "managementclusters")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ManagementClusterSpec {
    /// This field specifies the externally reachable address to which your managed cluster will connect. When a managed
    /// cluster is added, this field is used to populate an easy-to-apply manifest that will connect both clusters.
    /// Valid examples are: "0.0.0.0:31000", "example.com:32000", "[::1]:32500"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ManagementClusterTls>,
}

/// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagementClusterTls {
    /// SecretName indicates the name of the secret in the tigera-operator namespace that contains the private key and certificate that the management cluster uses when it listens for incoming connections.
    /// When set to tigera-management-cluster-connection voltron will use the same cert bundle which Guardian client certs are signed with.
    /// When set to manager-tls, voltron will use the same cert bundle which Manager UI is served with.
    /// This cert bundle must be a publicly signed cert created by the user.
    /// Note that Tigera Operator will generate a self-signed manager-tls cert if one does not exist,
    /// and use of that cert will result in Guardian being unable to verify Voltron's identity.
    /// If changed on a running cluster with connected managed clusters, all managed clusters will disconnect as they will no longer be able to verify Voltron's identity.
    /// To reconnect existing managed clusters, change the tls.ca of the  managed clusters' ManagementClusterConnection resource.
    /// One of: tigera-management-cluster-connection, manager-tls
    /// Default: tigera-management-cluster-connection
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<ManagementClusterTlsSecretName>,
}

/// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagementClusterTlsSecretName {
    #[serde(rename = "tigera-management-cluster-connection")]
    TigeraManagementClusterConnection,
    #[serde(rename = "manager-tls")]
    ManagerTls,
}

