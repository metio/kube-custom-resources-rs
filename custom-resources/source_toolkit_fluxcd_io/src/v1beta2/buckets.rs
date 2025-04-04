// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta2/buckets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BucketSpec specifies the required configuration to produce an Artifact for
/// an object storage bucket.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta2", kind = "Bucket", plural = "buckets")]
#[kube(namespaced)]
#[kube(status = "BucketStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BucketSpec {
    /// AccessFrom specifies an Access Control List for allowing cross-namespace
    /// references to this object.
    /// NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<BucketAccessFrom>,
    /// BucketName is the name of the object storage bucket.
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// CertSecretRef can be given the name of a Secret containing
    /// either or both of
    /// 
    /// - a PEM-encoded client certificate (`tls.crt`) and private
    /// key (`tls.key`);
    /// - a PEM-encoded CA certificate (`ca.crt`)
    /// 
    /// and whichever are supplied, will be used for connecting to the
    /// bucket. The client cert and key are useful if you are
    /// authenticating with a certificate; the CA cert is useful if
    /// you are using a self-signed server certificate. The Secret must
    /// be of type `Opaque` or `kubernetes.io/tls`.
    /// 
    /// This field is only supported for the `generic` provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<BucketCertSecretRef>,
    /// Endpoint is the object storage address the BucketName is located at.
    pub endpoint: String,
    /// Ignore overrides the set of excluded patterns in the .sourceignore format
    /// (which is the same as .gitignore). If not provided, a default will be used,
    /// consult the documentation for your version to find out what those are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<String>,
    /// Insecure allows connecting to a non-TLS HTTP Endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Interval at which the Bucket Endpoint is checked for updates.
    /// This interval is approximate and may be subject to jitter to ensure
    /// efficient use of resources.
    pub interval: String,
    /// Prefix to use for server-side filtering of files in the Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Provider of the object storage bucket.
    /// Defaults to 'generic', which expects an S3 (API) compatible object
    /// storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<BucketProvider>,
    /// ProxySecretRef specifies the Secret containing the proxy configuration
    /// to use while communicating with the Bucket server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxySecretRef")]
    pub proxy_secret_ref: Option<BucketProxySecretRef>,
    /// Region of the Endpoint where the BucketName is located in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// SecretRef specifies the Secret containing authentication credentials
    /// for the Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<BucketSecretRef>,
    /// STS specifies the required configuration to use a Security Token
    /// Service for fetching temporary credentials to authenticate in a
    /// Bucket provider.
    /// 
    /// This field is only supported for the `aws` and `generic` providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sts: Option<BucketSts>,
    /// Suspend tells the controller to suspend the reconciliation of this
    /// Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for fetch operations, defaults to 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// AccessFrom specifies an Access Control List for allowing cross-namespace
/// references to this object.
/// NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies.
    /// Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<BucketAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies.
/// An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// CertSecretRef can be given the name of a Secret containing
/// either or both of
/// 
/// - a PEM-encoded client certificate (`tls.crt`) and private
/// key (`tls.key`);
/// - a PEM-encoded CA certificate (`ca.crt`)
/// 
/// and whichever are supplied, will be used for connecting to the
/// bucket. The client cert and key are useful if you are
/// authenticating with a certificate; the CA cert is useful if
/// you are using a self-signed server certificate. The Secret must
/// be of type `Opaque` or `kubernetes.io/tls`.
/// 
/// This field is only supported for the `generic` provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// BucketSpec specifies the required configuration to produce an Artifact for
/// an object storage bucket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BucketProvider {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "azure")]
    Azure,
}

/// ProxySecretRef specifies the Secret containing the proxy configuration
/// to use while communicating with the Bucket server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketProxySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// SecretRef specifies the Secret containing authentication credentials
/// for the Bucket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// STS specifies the required configuration to use a Security Token
/// Service for fetching temporary credentials to authenticate in a
/// Bucket provider.
/// 
/// This field is only supported for the `aws` and `generic` providers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BucketSts {
    /// CertSecretRef can be given the name of a Secret containing
    /// either or both of
    /// 
    /// - a PEM-encoded client certificate (`tls.crt`) and private
    /// key (`tls.key`);
    /// - a PEM-encoded CA certificate (`ca.crt`)
    /// 
    /// and whichever are supplied, will be used for connecting to the
    /// STS endpoint. The client cert and key are useful if you are
    /// authenticating with a certificate; the CA cert is useful if
    /// you are using a self-signed server certificate. The Secret must
    /// be of type `Opaque` or `kubernetes.io/tls`.
    /// 
    /// This field is only supported for the `ldap` provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<BucketStsCertSecretRef>,
    /// Endpoint is the HTTP/S endpoint of the Security Token Service from
    /// where temporary credentials will be fetched.
    pub endpoint: String,
    /// Provider of the Security Token Service.
    pub provider: BucketStsProvider,
    /// SecretRef specifies the Secret containing authentication credentials
    /// for the STS endpoint. This Secret must contain the fields `username`
    /// and `password` and is supported only for the `ldap` provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<BucketStsSecretRef>,
}

/// CertSecretRef can be given the name of a Secret containing
/// either or both of
/// 
/// - a PEM-encoded client certificate (`tls.crt`) and private
/// key (`tls.key`);
/// - a PEM-encoded CA certificate (`ca.crt`)
/// 
/// and whichever are supplied, will be used for connecting to the
/// STS endpoint. The client cert and key are useful if you are
/// authenticating with a certificate; the CA cert is useful if
/// you are using a self-signed server certificate. The Secret must
/// be of type `Opaque` or `kubernetes.io/tls`.
/// 
/// This field is only supported for the `ldap` provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStsCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// STS specifies the required configuration to use a Security Token
/// Service for fetching temporary credentials to authenticate in a
/// Bucket provider.
/// 
/// This field is only supported for the `aws` and `generic` providers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BucketStsProvider {
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "ldap")]
    Ldap,
}

/// SecretRef specifies the Secret containing authentication credentials
/// for the STS endpoint. This Secret must contain the fields `username`
/// and `password` and is supported only for the `ldap` provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStsSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// BucketStatus records the observed state of a Bucket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStatus {
    /// Artifact represents the last successful Bucket reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<BucketStatusArtifact>,
    /// Conditions holds the conditions for the Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation of the Bucket object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ObservedIgnore is the observed exclusion patterns used for constructing
    /// the source artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedIgnore")]
    pub observed_ignore: Option<String>,
    /// URL is the dynamic fetch link for the latest Artifact.
    /// It is provided on a "best effort" basis, and using the precise
    /// BucketStatus.Artifact data is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the last successful Bucket reconciliation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStatusArtifact {
    /// Digest is the digest of the file in the form of '<algorithm>:<checksum>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of the
    /// Artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Metadata holds upstream information such as OCI annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Path is the relative file path of the Artifact. It can be used to locate
    /// the file in the root of the Artifact storage on the local file system of
    /// the controller managing the Source.
    pub path: String,
    /// Revision is a human-readable identifier traceable in the origin source
    /// system. It can be a Git commit SHA, Git tag, a Helm chart version, etc.
    pub revision: String,
    /// Size is the number of bytes in the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// URL is the HTTP address of the Artifact as exposed by the controller
    /// managing the Source. It can be used to retrieve the Artifact for
    /// consumption, e.g. by another controller applying the Artifact contents.
    pub url: String,
}

