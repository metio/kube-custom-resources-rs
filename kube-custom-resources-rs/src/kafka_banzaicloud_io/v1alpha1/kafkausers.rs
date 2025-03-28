// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/banzaicloud/koperator/kafka.banzaicloud.io/v1alpha1/kafkausers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// KafkaUserSpec defines the desired state of KafkaUser
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kafka.banzaicloud.io", version = "v1alpha1", kind = "KafkaUser", plural = "kafkausers")]
#[kube(namespaced)]
#[kube(status = "KafkaUserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KafkaUserSpec {
    /// Annotations defines the annotations placed on the certificate or certificate signing request object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// ClusterReference states a reference to a cluster for topic/user provisioning
    #[serde(rename = "clusterRef")]
    pub cluster_ref: KafkaUserClusterRef,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createCert")]
    pub create_cert: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsNames")]
    pub dns_names: Option<Vec<String>>,
    /// expirationSeconds is the requested duration of validity of the issued certificate. The minimum valid value for expirationSeconds is 3600 i.e. 1h. When it is not specified the default validation duration is 90 days
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expirationSeconds")]
    pub expiration_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeJKS")]
    pub include_jks: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pkiBackendSpec")]
    pub pki_backend_spec: Option<KafkaUserPkiBackendSpec>,
    /// secretName is used as the name of the K8S secret that contains the certificate of the KafkaUser. SecretName should be unique inside the namespace where KafkaUser is located.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicGrants")]
    pub topic_grants: Option<Vec<KafkaUserTopicGrants>>,
}

/// ClusterReference states a reference to a cluster for topic/user provisioning
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaUserClusterRef {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KafkaUserPkiBackendSpec {
    /// ObjectReference is a reference to an object with a given name, kind and group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issuerRef")]
    pub issuer_ref: Option<KafkaUserPkiBackendSpecIssuerRef>,
    #[serde(rename = "pkiBackend")]
    pub pki_backend: KafkaUserPkiBackendSpecPkiBackend,
    /// SignerName indicates requested signer, and is a qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signerName")]
    pub signer_name: Option<String>,
}

/// ObjectReference is a reference to an object with a given name, kind and group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaUserPkiBackendSpecIssuerRef {
    /// Group of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource being referred to.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaUserPkiBackendSpecPkiBackend {
    #[serde(rename = "cert-manager")]
    CertManager,
    #[serde(rename = "k8s-csr")]
    K8sCsr,
}

/// UserTopicGrant is the desired permissions for the KafkaUser
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KafkaUserTopicGrants {
    /// KafkaAccessType hold info about Kafka ACL
    #[serde(rename = "accessType")]
    pub access_type: KafkaUserTopicGrantsAccessType,
    /// KafkaPatternType hold the Resource Pattern Type of kafka ACL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "patternType")]
    pub pattern_type: Option<KafkaUserTopicGrantsPatternType>,
    #[serde(rename = "topicName")]
    pub topic_name: String,
}

/// UserTopicGrant is the desired permissions for the KafkaUser
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaUserTopicGrantsAccessType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

/// UserTopicGrant is the desired permissions for the KafkaUser
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaUserTopicGrantsPatternType {
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "prefixed")]
    Prefixed,
    #[serde(rename = "any")]
    Any,
}

/// KafkaUserStatus defines the observed state of KafkaUser
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaUserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
    /// UserState defines the state of a KafkaUser
    pub state: String,
}

