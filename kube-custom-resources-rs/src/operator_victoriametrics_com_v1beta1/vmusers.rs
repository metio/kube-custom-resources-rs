// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/VictoriaMetrics/operator/operator.victoriametrics.com/v1beta1/vmusers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// VMUserSpec defines the desired state of VMUser
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.victoriametrics.com", version = "v1beta1", kind = "VMUser", plural = "vmusers")]
#[kube(namespaced)]
#[kube(status = "VMUserStatus")]
#[kube(schema = "disabled")]
pub struct VMUserSpec {
    /// BearerToken Authorization header value for accessing protected endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bearerToken")]
    pub bearer_token: Option<String>,
    /// DefaultURLs backend url for non-matching paths filter usually used for default backend with error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_url: Option<Vec<String>>,
    /// DisableSecretCreation skips related secret creation for vmuser
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_secret_creation: Option<bool>,
    /// GeneratePassword instructs operator to generate password for user if spec.password if empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatePassword")]
    pub generate_password: Option<bool>,
    /// Headers represent additional http headers, that vmauth uses in form of ["header_key: header_value"] multiple values for header key: ["header_key: value1,value2"] it's available since 1.68.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// IPFilters defines per target src ip filters supported only with enterprise version of vmauth https://docs.victoriametrics.com/vmauth.html#ip-filters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_filters: Option<VMUserIpFilters>,
    /// MaxConcurrentRequests defines max concurrent requests per user 300 is default value for vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests: Option<i64>,
    /// Name of the VMUser object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Password basic auth password for accessing protected endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// PasswordRef allows fetching password from user-create secret by its name and key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordRef")]
    pub password_ref: Option<VMUserPasswordRef>,
    /// ResponseHeaders represent additional http headers, that vmauth adds for request response in form of ["header_key: header_value"] multiple values for header key: ["header_key: value1,value2"] it's available since 1.93.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<String>>,
    /// RetryStatusCodes defines http status codes in numeric format for request retries e.g. [429,503]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_status_codes: Option<Vec<i64>>,
    /// TargetRefs - reference to endpoints, which user may access.
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<VMUserTargetRefs>,
    /// TokenRef allows fetching token from user-created secrets by its name and key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenRef")]
    pub token_ref: Option<VMUserTokenRef>,
    /// UserName basic auth user name for accessing protected endpoint, will be replaced with metadata.name of VMUser if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// IPFilters defines per target src ip filters supported only with enterprise version of vmauth https://docs.victoriametrics.com/vmauth.html#ip-filters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserIpFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_list: Option<Vec<String>>,
}

/// PasswordRef allows fetching password from user-create secret by its name and key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserPasswordRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TargetRef describes target for user traffic forwarding. one of target types can be chosen: crd or static per targetRef. user can define multiple targetRefs with different ref Types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefs {
    /// CRD describes exist operator's CRD object, operator generates access url based on CRD params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd: Option<VMUserTargetRefsCrd>,
    /// Headers represent additional http headers, that vmauth uses in form of ["header_key: header_value"] multiple values for header key: ["header_key: value1,value2"] it's available since 1.68.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// Paths - matched path to route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// ResponseHeaders represent additional http headers, that vmauth adds for request response in form of ["header_key: header_value"] multiple values for header key: ["header_key: value1,value2"] it's available since 1.93.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<String>>,
    /// RetryStatusCodes defines http status codes in numeric format for request retries Can be defined per target or at VMUser.spec level e.g. [429,503]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_status_codes: Option<Vec<i64>>,
    /// Static - user defined url for traffic forward, for instance http://vmsingle:8429
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "static")]
    pub r#static: Option<VMUserTargetRefsStatic>,
    /// QueryParams []string `json:"queryParams,omitempty"` TargetPathSuffix allows to add some suffix to the target path It allows to hide tenant configuration from user with crd as ref. it also may contain any url encoded params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_path_suffix: Option<String>,
}

/// CRD describes exist operator's CRD object, operator generates access url based on CRD params.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsCrd {
    /// Kind one of: VMAgent VMAlert VMCluster VMSingle or VMAlertManager
    pub kind: String,
    /// Name target CRD object name
    pub name: String,
    /// Namespace target CRD object namespace.
    pub namespace: String,
}

/// Static - user defined url for traffic forward, for instance http://vmsingle:8429
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsStatic {
    /// URL http url for given staticRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URLs allows setting multiple urls for load-balancing at vmauth-side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// TokenRef allows fetching token from user-created secrets by its name and key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTokenRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// VMUserStatus defines the observed state of VMUser
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserStatus {
}

