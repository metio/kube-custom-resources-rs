// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clastix/kamaji/kamaji.clastix.io/v1alpha1/datastores.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DataStoreSpec defines the desired state of DataStore.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kamaji.clastix.io", version = "v1alpha1", kind = "DataStore", plural = "datastores")]
#[kube(status = "DataStoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DataStoreSpec {
    /// In case of authentication enabled for the given data store, specifies the username and password pair.
    /// This value is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<DataStoreBasicAuth>,
    /// The driver to use to connect to the shared datastore.
    pub driver: DataStoreDriver,
    /// List of the endpoints to connect to the shared datastore.
    /// No need for protocol, just bare IP/FQDN and port.
    pub endpoints: Vec<String>,
    /// Defines the TLS/SSL configuration required to connect to the data store in a secure way.
    /// This value is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsConfig")]
    pub tls_config: Option<DataStoreTlsConfig>,
}

/// In case of authentication enabled for the given data store, specifies the username and password pair.
/// This value is optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreBasicAuth {
    pub password: DataStoreBasicAuthPassword,
    pub username: DataStoreBasicAuthUsername,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreBasicAuthPassword {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreBasicAuthPasswordSecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreBasicAuthPasswordSecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreBasicAuthUsername {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreBasicAuthUsernameSecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreBasicAuthUsernameSecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// DataStoreSpec defines the desired state of DataStore.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataStoreDriver {
    #[serde(rename = "etcd")]
    Etcd,
    #[serde(rename = "MySQL")]
    MySql,
    #[serde(rename = "PostgreSQL")]
    PostgreSql,
    #[serde(rename = "NATS")]
    Nats,
}

/// Defines the TLS/SSL configuration required to connect to the data store in a secure way.
/// This value is optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfig {
    /// Retrieve the Certificate Authority certificate and private key, such as bare content of the file, or a SecretReference.
    /// The key reference is required since etcd authentication is based on certificates, and Kamaji is responsible in creating this.
    #[serde(rename = "certificateAuthority")]
    pub certificate_authority: DataStoreTlsConfigCertificateAuthority,
    /// Specifies the SSL/TLS key and private key pair used to connect to the data store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertificate")]
    pub client_certificate: Option<DataStoreTlsConfigClientCertificate>,
}

/// Retrieve the Certificate Authority certificate and private key, such as bare content of the file, or a SecretReference.
/// The key reference is required since etcd authentication is based on certificates, and Kamaji is responsible in creating this.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigCertificateAuthority {
    pub certificate: DataStoreTlsConfigCertificateAuthorityCertificate,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<DataStoreTlsConfigCertificateAuthorityPrivateKey>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigCertificateAuthorityCertificate {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreTlsConfigCertificateAuthorityCertificateSecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigCertificateAuthorityCertificateSecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigCertificateAuthorityPrivateKey {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreTlsConfigCertificateAuthorityPrivateKeySecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigCertificateAuthorityPrivateKeySecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Specifies the SSL/TLS key and private key pair used to connect to the data store.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigClientCertificate {
    pub certificate: DataStoreTlsConfigClientCertificateCertificate,
    #[serde(rename = "privateKey")]
    pub private_key: DataStoreTlsConfigClientCertificatePrivateKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigClientCertificateCertificate {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreTlsConfigClientCertificateCertificateSecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigClientCertificateCertificateSecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigClientCertificatePrivateKey {
    /// Bare content of the file, base64 encoded.
    /// It has precedence over the SecretReference value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretReference")]
    pub secret_reference: Option<DataStoreTlsConfigClientCertificatePrivateKeySecretReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreTlsConfigClientCertificatePrivateKeySecretReference {
    /// Name of the key for the given Secret reference where the content is stored.
    /// This value is mandatory.
    #[serde(rename = "keyPath")]
    pub key_path: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// DataStoreStatus defines the observed state of DataStore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataStoreStatus {
    /// List of the Tenant Control Planes, namespaced named, using this data store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usedBy")]
    pub used_by: Option<Vec<String>>,
}

