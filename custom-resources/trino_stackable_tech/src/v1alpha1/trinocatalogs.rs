// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/stackabletech/trino-operator/trino.stackable.tech/v1alpha1/trinocatalogs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// The TrinoCatalog resource can be used to define catalogs in Kubernetes objects. Read more about it in the [Trino operator concept docs](https://docs.stackable.tech/home/nightly/trino/concepts) and the [Trino operator usage guide](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/). The documentation also contains a list of all the supported backends.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "trino.stackable.tech", version = "v1alpha1", kind = "TrinoCatalog", plural = "trinocatalogs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TrinoCatalogSpec {
    /// The `configOverrides` allow overriding arbitrary Trino settings. For example, for Hive you could add `hive.metastore.username: trino`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configOverrides")]
    pub config_overrides: Option<BTreeMap<String, String>>,
    /// The `connector` defines which connector is used.
    pub connector: TrinoCatalogConnector,
}

/// The `connector` defines which connector is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnector {
    /// A [Black Hole](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/black-hole) connector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blackHole")]
    pub black_hole: Option<TrinoCatalogConnectorBlackHole>,
    /// An [Delta Lake](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/delta-lake) connector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deltaLake")]
    pub delta_lake: Option<TrinoCatalogConnectorDeltaLake>,
    /// A [generic](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/generic) connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic: Option<TrinoCatalogConnectorGeneric>,
    /// A [Google sheets](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/google-sheets) connector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "googleSheet")]
    pub google_sheet: Option<TrinoCatalogConnectorGoogleSheet>,
    /// An [Apache Hive](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/hive) connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hive: Option<TrinoCatalogConnectorHive>,
    /// An [Apache Iceberg](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/iceberg) connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<TrinoCatalogConnectorIceberg>,
    /// A [TPC-DS](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/tpcds) connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tpcds: Option<TrinoCatalogConnectorTpcds>,
    /// A [TPC-H](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/tpch) connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tpch: Option<TrinoCatalogConnectorTpch>,
}

/// A [Black Hole](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/black-hole) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorBlackHole {
}

/// An [Delta Lake](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/delta-lake) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLake {
    /// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<TrinoCatalogConnectorDeltaLakeHdfs>,
    /// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
    pub metastore: TrinoCatalogConnectorDeltaLakeMetastore,
    /// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<TrinoCatalogConnectorDeltaLakeS3>,
}

/// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeHdfs {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the HDFS cluster.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeMetastore {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the Hive metastore.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3 {
    /// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<TrinoCatalogConnectorDeltaLakeS3Inline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3Inline {
    /// Which access style to use. Defaults to virtual hosted-style as most of the data products out there. Have a look at the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessStyle")]
    pub access_style: Option<TrinoCatalogConnectorDeltaLakeS3InlineAccessStyle>,
    /// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TrinoCatalogConnectorDeltaLakeS3InlineCredentials>,
    /// Host of the S3 server without any protocol or port. For example: `west1.my-cloud.com`.
    pub host: String,
    /// Port the S3 server listens on. If not specified the product will determine the port to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Use a TLS connection. If not specified no TLS will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TrinoCatalogConnectorDeltaLakeS3InlineTls>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrinoCatalogConnectorDeltaLakeS3InlineAccessStyle {
    Path,
    VirtualHosted,
}

/// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineCredentials {
    /// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TrinoCatalogConnectorDeltaLakeS3InlineCredentialsScope>,
    /// [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) containing the LDAP bind credentials.
    #[serde(rename = "secretClass")]
    pub secret_class: String,
}

/// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineCredentialsScope {
    /// The listener volume scope allows Node and Service scopes to be inferred from the applicable listeners. This must correspond to Volume names in the Pod that mount Listeners.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerVolumes")]
    pub listener_volumes: Option<Vec<String>>,
    /// The node scope is resolved to the name of the Kubernetes Node object that the Pod is running on. This will typically be the DNS name of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<bool>,
    /// The pod scope is resolved to the name of the Kubernetes Pod. This allows the secret to differentiate between StatefulSet replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<bool>,
    /// The service scope allows Pod objects to specify custom scopes. This should typically correspond to Service objects that the Pod participates in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// Use a TLS connection. If not specified no TLS will be used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTls {
    /// The verification method used to verify the certificates of the server and/or the client.
    pub verification: TrinoCatalogConnectorDeltaLakeS3InlineTlsVerification,
}

/// The verification method used to verify the certificates of the server and/or the client.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTlsVerification {
    /// Use TLS but don't verify certificates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationNone>,
    /// Use TLS and a CA certificate to verify the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServer>,
}

/// Use TLS but don't verify certificates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationNone {
}

/// Use TLS and a CA certificate to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServer {
    /// CA cert to verify the server.
    #[serde(rename = "caCert")]
    pub ca_cert: TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServerCaCert,
}

/// CA cert to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServerCaCert {
    /// Name of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) which will provide the CA certificate. Note that a SecretClass does not need to have a key but can also work with just a CA certificate, so if you got provided with a CA cert but don't have access to the key you can still use this method.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretClass")]
    pub secret_class: Option<String>,
    /// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webPki")]
    pub web_pki: Option<TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServerCaCertWebPki>,
}

/// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorDeltaLakeS3InlineTlsVerificationServerCaCertWebPki {
}

/// A [generic](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/generic) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGeneric {
    /// Name of the Trino connector. Will be passed to `connector.name`.
    #[serde(rename = "connectorName")]
    pub connector_name: String,
    /// A map of properties to put in the connector configuration file. They can be specified either as a raw value or be read from a Secret or ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, TrinoCatalogConnectorGenericProperties>>,
}

/// A map of properties to put in the connector configuration file. They can be specified either as a raw value or be read from a Secret or ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGenericProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Selects a key from a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFromConfigMap")]
    pub value_from_config_map: Option<TrinoCatalogConnectorGenericPropertiesValueFromConfigMap>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFromSecret")]
    pub value_from_secret: Option<TrinoCatalogConnectorGenericPropertiesValueFromSecret>,
}

/// Selects a key from a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGenericPropertiesValueFromConfigMap {
    /// The key to select.
    pub key: String,
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGenericPropertiesValueFromSecret {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// A [Google sheets](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/google-sheets) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGoogleSheet {
    /// Cache the contents of sheets. This is used to reduce Google Sheets API usage and latency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<TrinoCatalogConnectorGoogleSheetCache>,
    /// The Secret containing the Google API JSON key file. The key used from the Secret is `credentials`.
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    /// Sheet ID of the spreadsheet, that contains the table mapping.
    #[serde(rename = "metadataSheetId")]
    pub metadata_sheet_id: String,
}

/// Cache the contents of sheets. This is used to reduce Google Sheets API usage and latency.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorGoogleSheetCache {
    /// How long to cache spreadsheet data or metadata, defaults to `5m`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sheetsDataExpireAfterWrite")]
    pub sheets_data_expire_after_write: Option<String>,
    /// Maximum number of spreadsheets to cache, defaults to 1000.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sheetsDataMaxCacheSize")]
    pub sheets_data_max_cache_size: Option<String>,
}

/// An [Apache Hive](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/hive) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHive {
    /// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<TrinoCatalogConnectorHiveHdfs>,
    /// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
    pub metastore: TrinoCatalogConnectorHiveMetastore,
    /// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<TrinoCatalogConnectorHiveS3>,
}

/// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveHdfs {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the HDFS cluster.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveMetastore {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the Hive metastore.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3 {
    /// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<TrinoCatalogConnectorHiveS3Inline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3Inline {
    /// Which access style to use. Defaults to virtual hosted-style as most of the data products out there. Have a look at the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessStyle")]
    pub access_style: Option<TrinoCatalogConnectorHiveS3InlineAccessStyle>,
    /// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TrinoCatalogConnectorHiveS3InlineCredentials>,
    /// Host of the S3 server without any protocol or port. For example: `west1.my-cloud.com`.
    pub host: String,
    /// Port the S3 server listens on. If not specified the product will determine the port to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Use a TLS connection. If not specified no TLS will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TrinoCatalogConnectorHiveS3InlineTls>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrinoCatalogConnectorHiveS3InlineAccessStyle {
    Path,
    VirtualHosted,
}

/// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineCredentials {
    /// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TrinoCatalogConnectorHiveS3InlineCredentialsScope>,
    /// [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) containing the LDAP bind credentials.
    #[serde(rename = "secretClass")]
    pub secret_class: String,
}

/// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineCredentialsScope {
    /// The listener volume scope allows Node and Service scopes to be inferred from the applicable listeners. This must correspond to Volume names in the Pod that mount Listeners.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerVolumes")]
    pub listener_volumes: Option<Vec<String>>,
    /// The node scope is resolved to the name of the Kubernetes Node object that the Pod is running on. This will typically be the DNS name of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<bool>,
    /// The pod scope is resolved to the name of the Kubernetes Pod. This allows the secret to differentiate between StatefulSet replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<bool>,
    /// The service scope allows Pod objects to specify custom scopes. This should typically correspond to Service objects that the Pod participates in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// Use a TLS connection. If not specified no TLS will be used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTls {
    /// The verification method used to verify the certificates of the server and/or the client.
    pub verification: TrinoCatalogConnectorHiveS3InlineTlsVerification,
}

/// The verification method used to verify the certificates of the server and/or the client.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTlsVerification {
    /// Use TLS but don't verify certificates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<TrinoCatalogConnectorHiveS3InlineTlsVerificationNone>,
    /// Use TLS and a CA certificate to verify the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<TrinoCatalogConnectorHiveS3InlineTlsVerificationServer>,
}

/// Use TLS but don't verify certificates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTlsVerificationNone {
}

/// Use TLS and a CA certificate to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTlsVerificationServer {
    /// CA cert to verify the server.
    #[serde(rename = "caCert")]
    pub ca_cert: TrinoCatalogConnectorHiveS3InlineTlsVerificationServerCaCert,
}

/// CA cert to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTlsVerificationServerCaCert {
    /// Name of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) which will provide the CA certificate. Note that a SecretClass does not need to have a key but can also work with just a CA certificate, so if you got provided with a CA cert but don't have access to the key you can still use this method.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretClass")]
    pub secret_class: Option<String>,
    /// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webPki")]
    pub web_pki: Option<TrinoCatalogConnectorHiveS3InlineTlsVerificationServerCaCertWebPki>,
}

/// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorHiveS3InlineTlsVerificationServerCaCertWebPki {
}

/// An [Apache Iceberg](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/iceberg) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIceberg {
    /// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<TrinoCatalogConnectorIcebergHdfs>,
    /// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
    pub metastore: TrinoCatalogConnectorIcebergMetastore,
    /// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<TrinoCatalogConnectorIcebergS3>,
}

/// Connection to an HDFS cluster. Please make sure that the underlying Hive metastore also has access to the HDFS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergHdfs {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the HDFS cluster.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Mandatory connection to a Hive Metastore, which will be used as a storage for metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergMetastore {
    /// Name of the [discovery ConfigMap](https://docs.stackable.tech/home/nightly/concepts/service_discovery) providing information about the Hive metastore.
    #[serde(rename = "configMap")]
    pub config_map: String,
}

/// Connection to an S3 store. Please make sure that the underlying Hive metastore also has access to the S3 store. Learn more about S3 configuration in the [S3 concept docs](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3 {
    /// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<TrinoCatalogConnectorIcebergS3Inline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3Inline {
    /// Which access style to use. Defaults to virtual hosted-style as most of the data products out there. Have a look at the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessStyle")]
    pub access_style: Option<TrinoCatalogConnectorIcebergS3InlineAccessStyle>,
    /// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TrinoCatalogConnectorIcebergS3InlineCredentials>,
    /// Host of the S3 server without any protocol or port. For example: `west1.my-cloud.com`.
    pub host: String,
    /// Port the S3 server listens on. If not specified the product will determine the port to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Use a TLS connection. If not specified no TLS will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TrinoCatalogConnectorIcebergS3InlineTls>,
}

/// S3 connection definition as a resource. Learn more on the [S3 concept documentation](https://docs.stackable.tech/home/nightly/concepts/s3).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrinoCatalogConnectorIcebergS3InlineAccessStyle {
    Path,
    VirtualHosted,
}

/// If the S3 uses authentication you have to specify you S3 credentials. In the most cases a [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) providing `accessKey` and `secretKey` is sufficient.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineCredentials {
    /// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TrinoCatalogConnectorIcebergS3InlineCredentialsScope>,
    /// [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) containing the LDAP bind credentials.
    #[serde(rename = "secretClass")]
    pub secret_class: String,
}

/// [Scope](https://docs.stackable.tech/home/nightly/secret-operator/scope) of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineCredentialsScope {
    /// The listener volume scope allows Node and Service scopes to be inferred from the applicable listeners. This must correspond to Volume names in the Pod that mount Listeners.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerVolumes")]
    pub listener_volumes: Option<Vec<String>>,
    /// The node scope is resolved to the name of the Kubernetes Node object that the Pod is running on. This will typically be the DNS name of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<bool>,
    /// The pod scope is resolved to the name of the Kubernetes Pod. This allows the secret to differentiate between StatefulSet replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<bool>,
    /// The service scope allows Pod objects to specify custom scopes. This should typically correspond to Service objects that the Pod participates in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// Use a TLS connection. If not specified no TLS will be used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTls {
    /// The verification method used to verify the certificates of the server and/or the client.
    pub verification: TrinoCatalogConnectorIcebergS3InlineTlsVerification,
}

/// The verification method used to verify the certificates of the server and/or the client.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTlsVerification {
    /// Use TLS but don't verify certificates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<TrinoCatalogConnectorIcebergS3InlineTlsVerificationNone>,
    /// Use TLS and a CA certificate to verify the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<TrinoCatalogConnectorIcebergS3InlineTlsVerificationServer>,
}

/// Use TLS but don't verify certificates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTlsVerificationNone {
}

/// Use TLS and a CA certificate to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTlsVerificationServer {
    /// CA cert to verify the server.
    #[serde(rename = "caCert")]
    pub ca_cert: TrinoCatalogConnectorIcebergS3InlineTlsVerificationServerCaCert,
}

/// CA cert to verify the server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTlsVerificationServerCaCert {
    /// Name of the [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) which will provide the CA certificate. Note that a SecretClass does not need to have a key but can also work with just a CA certificate, so if you got provided with a CA cert but don't have access to the key you can still use this method.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretClass")]
    pub secret_class: Option<String>,
    /// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webPki")]
    pub web_pki: Option<TrinoCatalogConnectorIcebergS3InlineTlsVerificationServerCaCertWebPki>,
}

/// Use TLS and the CA certificates trusted by the common web browsers to verify the server. This can be useful when you e.g. use public AWS S3 or other public available services.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorIcebergS3InlineTlsVerificationServerCaCertWebPki {
}

/// A [TPC-DS](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/tpcds) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorTpcds {
}

/// A [TPC-H](https://docs.stackable.tech/home/nightly/trino/usage-guide/catalogs/tpch) connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrinoCatalogConnectorTpch {
}

