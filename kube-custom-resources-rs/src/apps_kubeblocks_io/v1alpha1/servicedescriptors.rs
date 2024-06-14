// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/servicedescriptors.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ServiceDescriptorSpec defines the desired state of ServiceDescriptor.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "ServiceDescriptor", plural = "servicedescriptors")]
#[kube(namespaced)]
#[kube(status = "ServiceDescriptorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ServiceDescriptorSpec {
    /// Specifies the authentication credentials required for accessing an external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<ServiceDescriptorAuth>,
    /// Specifies the endpoint of the external service.
    /// 
    /// 
    /// If the service is exposed via a cluster, the endpoint will be provided in the format of `host:port`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<ServiceDescriptorEndpoint>,
    /// Specifies the service or IP address of the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<ServiceDescriptorHost>,
    /// Specifies the port of the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<ServiceDescriptorPort>,
    /// Describes the type of database service provided by the external service.
    /// For example, "mysql", "redis", "mongodb".
    /// This field categorizes databases by their functionality, protocol and compatibility, facilitating appropriate
    /// service integration based on their unique capabilities.
    /// 
    /// 
    /// This field is case-insensitive.
    /// 
    /// 
    /// It also supports abbreviations for some well-known databases:
    /// - "pg", "pgsql", "postgres", "postgresql": PostgreSQL service
    /// - "zk", "zookeeper": ZooKeeper service
    /// - "es", "elasticsearch": Elasticsearch service
    /// - "mongo", "mongodb": MongoDB service
    /// - "ch", "clickhouse": ClickHouse service
    #[serde(rename = "serviceKind")]
    pub service_kind: String,
    /// Describes the version of the service provided by the external service.
    /// This is crucial for ensuring compatibility between different components of the system,
    /// as different versions of a service may have varying features.
    #[serde(rename = "serviceVersion")]
    pub service_version: String,
}

/// Specifies the authentication credentials required for accessing an external service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuth {
    /// Specifies the password for the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<ServiceDescriptorAuthPassword>,
    /// Specifies the username for the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<ServiceDescriptorAuthUsername>,
}

/// Specifies the password for the external service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPassword {
    /// Holds a direct string or an expression that can be evaluated to a string.
    /// 
    /// 
    /// It can include variables denoted by $(VAR_NAME).
    /// These variables are expanded to the value of the environment variables defined in the container.
    /// If a variable cannot be resolved, it remains unchanged in the output.
    /// 
    /// 
    /// To escape variable expansion and retain the literal value, use double $ characters.
    /// 
    /// 
    /// For example:
    /// 
    /// 
    /// - "$(VAR_NAME)" will be expanded to the value of the environment variable VAR_NAME.
    /// - "$$(VAR_NAME)" will result in "$(VAR_NAME)" in the output, without any variable expansion.
    /// 
    /// 
    /// Default value is an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Specifies the source for the variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ServiceDescriptorAuthPasswordValueFrom>,
}

/// Specifies the source for the variable's value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPasswordValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ServiceDescriptorAuthPasswordValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ServiceDescriptorAuthPasswordValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ServiceDescriptorAuthPasswordValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ServiceDescriptorAuthPasswordValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPasswordValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPasswordValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPasswordValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthPasswordValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Specifies the username for the external service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsername {
    /// Holds a direct string or an expression that can be evaluated to a string.
    /// 
    /// 
    /// It can include variables denoted by $(VAR_NAME).
    /// These variables are expanded to the value of the environment variables defined in the container.
    /// If a variable cannot be resolved, it remains unchanged in the output.
    /// 
    /// 
    /// To escape variable expansion and retain the literal value, use double $ characters.
    /// 
    /// 
    /// For example:
    /// 
    /// 
    /// - "$(VAR_NAME)" will be expanded to the value of the environment variable VAR_NAME.
    /// - "$$(VAR_NAME)" will result in "$(VAR_NAME)" in the output, without any variable expansion.
    /// 
    /// 
    /// Default value is an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Specifies the source for the variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ServiceDescriptorAuthUsernameValueFrom>,
}

/// Specifies the source for the variable's value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsernameValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ServiceDescriptorAuthUsernameValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ServiceDescriptorAuthUsernameValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ServiceDescriptorAuthUsernameValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ServiceDescriptorAuthUsernameValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsernameValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsernameValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsernameValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorAuthUsernameValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Specifies the endpoint of the external service.
/// 
/// 
/// If the service is exposed via a cluster, the endpoint will be provided in the format of `host:port`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpoint {
    /// Holds a direct string or an expression that can be evaluated to a string.
    /// 
    /// 
    /// It can include variables denoted by $(VAR_NAME).
    /// These variables are expanded to the value of the environment variables defined in the container.
    /// If a variable cannot be resolved, it remains unchanged in the output.
    /// 
    /// 
    /// To escape variable expansion and retain the literal value, use double $ characters.
    /// 
    /// 
    /// For example:
    /// 
    /// 
    /// - "$(VAR_NAME)" will be expanded to the value of the environment variable VAR_NAME.
    /// - "$$(VAR_NAME)" will result in "$(VAR_NAME)" in the output, without any variable expansion.
    /// 
    /// 
    /// Default value is an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Specifies the source for the variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ServiceDescriptorEndpointValueFrom>,
}

/// Specifies the source for the variable's value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpointValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ServiceDescriptorEndpointValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ServiceDescriptorEndpointValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ServiceDescriptorEndpointValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ServiceDescriptorEndpointValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpointValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpointValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpointValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorEndpointValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Specifies the service or IP address of the external service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHost {
    /// Holds a direct string or an expression that can be evaluated to a string.
    /// 
    /// 
    /// It can include variables denoted by $(VAR_NAME).
    /// These variables are expanded to the value of the environment variables defined in the container.
    /// If a variable cannot be resolved, it remains unchanged in the output.
    /// 
    /// 
    /// To escape variable expansion and retain the literal value, use double $ characters.
    /// 
    /// 
    /// For example:
    /// 
    /// 
    /// - "$(VAR_NAME)" will be expanded to the value of the environment variable VAR_NAME.
    /// - "$$(VAR_NAME)" will result in "$(VAR_NAME)" in the output, without any variable expansion.
    /// 
    /// 
    /// Default value is an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Specifies the source for the variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ServiceDescriptorHostValueFrom>,
}

/// Specifies the source for the variable's value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHostValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ServiceDescriptorHostValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ServiceDescriptorHostValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ServiceDescriptorHostValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ServiceDescriptorHostValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHostValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHostValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHostValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorHostValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Specifies the port of the external service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPort {
    /// Holds a direct string or an expression that can be evaluated to a string.
    /// 
    /// 
    /// It can include variables denoted by $(VAR_NAME).
    /// These variables are expanded to the value of the environment variables defined in the container.
    /// If a variable cannot be resolved, it remains unchanged in the output.
    /// 
    /// 
    /// To escape variable expansion and retain the literal value, use double $ characters.
    /// 
    /// 
    /// For example:
    /// 
    /// 
    /// - "$(VAR_NAME)" will be expanded to the value of the environment variable VAR_NAME.
    /// - "$$(VAR_NAME)" will result in "$(VAR_NAME)" in the output, without any variable expansion.
    /// 
    /// 
    /// Default value is an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Specifies the source for the variable's value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ServiceDescriptorPortValueFrom>,
}

/// Specifies the source for the variable's value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPortValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ServiceDescriptorPortValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ServiceDescriptorPortValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ServiceDescriptorPortValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ServiceDescriptorPortValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPortValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPortValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPortValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorPortValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ServiceDescriptorStatus defines the observed state of ServiceDescriptor
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceDescriptorStatus {
    /// Provides a human-readable explanation detailing the reason for the current phase of the ServiceConnectionCredential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the generation number that has been processed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Indicates the current lifecycle phase of the ServiceDescriptor. This can be either 'Available' or 'Unavailable'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ServiceDescriptorStatusPhase>,
}

/// ServiceDescriptorStatus defines the observed state of ServiceDescriptor
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceDescriptorStatusPhase {
    Available,
    Unavailable,
}

