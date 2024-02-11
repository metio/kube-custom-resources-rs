// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1alpha1/tenants.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// TenantSpec defines the desired state of Tenant
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1alpha1", kind = "Tenant", plural = "tenants")]
#[kube(namespaced)]
#[kube(status = "TenantStatus")]
#[kube(schema = "disabled")]
pub struct TenantSpec {
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "financeSupportEmail")]
    pub finance_support_email: Option<String>,
    /// additional parameters, used for Update, as in master portal Api Docs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromEmail")]
    pub from_email: Option<String>,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    #[serde(rename = "masterCredentialsRef")]
    pub master_credentials_ref: TenantMasterCredentialsRef,
    #[serde(rename = "organizationName")]
    pub organization_name: String,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    #[serde(rename = "passwordCredentialsRef")]
    pub password_credentials_ref: TenantPasswordCredentialsRef,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "siteAccessCode")]
    pub site_access_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportEmail")]
    pub support_email: Option<String>,
    #[serde(rename = "systemMasterUrl")]
    pub system_master_url: String,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    #[serde(rename = "tenantSecretRef")]
    pub tenant_secret_ref: TenantTenantSecretRef,
    pub username: String,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantMasterCredentialsRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantPasswordCredentialsRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantTenantSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// TenantStatus defines the observed state of Tenant
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantStatus {
    #[serde(rename = "adminId")]
    pub admin_id: i64,
    /// Current state of the tenant resource. Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TenantStatusConditions>>,
    #[serde(rename = "tenantId")]
    pub tenant_id: i64,
}

/// Condition represents an observation of an object's state. Conditions are an extension mechanism intended to be used when the details of an observation are not a priori known or would not apply to all instances of a given Kind. 
///  Conditions should be added to explicitly convey properties that users and components care about rather than requiring those properties to be inferred from other observations. Once defined, the meaning of a Condition can not be changed arbitrarily - it becomes part of the API, and has the same backwards- and forwards-compatibility concerns of any other part of the API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ConditionReason is intended to be a one-word, CamelCase representation of the category of cause of the current status. It is intended to be used in concise output, such as one-line kubectl get output, and in summarizing occurrences of causes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    /// ConditionType is the type of the condition and is typically a CamelCased word or short phrase. 
    ///  Condition types should indicate state in the "abnormal-true" polarity. For example, if the condition indicates when a policy is invalid, the "is valid" case is probably the norm, so the condition should be called "Invalid".
    #[serde(rename = "type")]
    pub r#type: String,
}

