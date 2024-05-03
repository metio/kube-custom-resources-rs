// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/keycloak/keycloak-operator/keycloak.org/v1alpha1/keycloakclients.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// KeycloakClientSpec defines the desired state of KeycloakClient.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keycloak.org", version = "v1alpha1", kind = "KeycloakClient", plural = "keycloakclients")]
#[kube(namespaced)]
#[kube(status = "KeycloakClientStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KeycloakClientSpec {
    /// Keycloak Client REST object.
    pub client: KeycloakClientClient,
    /// Selector for looking up KeycloakRealm Custom Resources.
    #[serde(rename = "realmSelector")]
    pub realm_selector: KeycloakClientRealmSelector,
    /// Client Roles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<KeycloakClientRoles>>,
    /// Scope Mappings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopeMappings")]
    pub scope_mappings: Option<KeycloakClientScopeMappings>,
    /// Service account client roles for this client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountClientRoles")]
    pub service_account_client_roles: Option<BTreeMap<String, String>>,
    /// Service account realm roles for this client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountRealmRoles")]
    pub service_account_realm_roles: Option<Vec<String>>,
}

/// Keycloak Client REST object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClient {
    /// Access options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<BTreeMap<String, bool>>,
    /// Application Admin URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminUrl")]
    pub admin_url: Option<String>,
    /// Client Attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// Authentication Flow Binding Overrides.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authenticationFlowBindingOverrides")]
    pub authentication_flow_binding_overrides: Option<BTreeMap<String, String>>,
    /// True if fine-grained authorization support is enabled for this client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizationServicesEnabled")]
    pub authorization_services_enabled: Option<bool>,
    /// Authorization settings for this resource server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizationSettings")]
    pub authorization_settings: Option<KeycloakClientClientAuthorizationSettings>,
    /// Application base URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseUrl")]
    pub base_url: Option<String>,
    /// True if a client supports only Bearer Tokens.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bearerOnly")]
    pub bearer_only: Option<bool>,
    /// What Client authentication type to use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientAuthenticatorType")]
    pub client_authenticator_type: Option<String>,
    /// Client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// True if Consent Screen is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consentRequired")]
    pub consent_required: Option<bool>,
    /// A list of default client scopes. Default client scopes are always applied when issuing OpenID Connect tokens or SAML assertions for this client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultClientScopes")]
    pub default_client_scopes: Option<Vec<String>>,
    /// Default Client roles.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRoles")]
    pub default_roles: Option<Vec<String>>,
    /// Client description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// True if Direct Grant is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directAccessGrantsEnabled")]
    pub direct_access_grants_enabled: Option<bool>,
    /// Client enabled flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// True if this client supports Front Channel logout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frontchannelLogout")]
    pub frontchannel_logout: Option<bool>,
    /// True if Full Scope is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullScopeAllowed")]
    pub full_scope_allowed: Option<bool>,
    /// Client ID. If not specified, automatically generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// True if Implicit flow is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "implicitFlowEnabled")]
    pub implicit_flow_enabled: Option<bool>,
    /// Client name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Node registration timeout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeReRegistrationTimeout")]
    pub node_re_registration_timeout: Option<i64>,
    /// Not Before setting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBefore")]
    pub not_before: Option<i64>,
    /// A list of optional client scopes. Optional client scopes are applied when issuing tokens for this client, but only when they are requested by the scope parameter in the OpenID Connect authorization request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optionalClientScopes")]
    pub optional_client_scopes: Option<Vec<String>>,
    /// Protocol used for this Client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Protocol Mappers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolMappers")]
    pub protocol_mappers: Option<Vec<KeycloakClientClientProtocolMappers>>,
    /// True if this is a public Client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicClient")]
    pub public_client: Option<bool>,
    /// A list of valid Redirection URLs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectUris")]
    pub redirect_uris: Option<Vec<String>>,
    /// Application root URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootUrl")]
    pub root_url: Option<String>,
    /// Client Secret. The Operator will automatically create a Secret based on this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// True if Service Accounts are enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountsEnabled")]
    pub service_accounts_enabled: Option<bool>,
    /// True if Standard flow is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "standardFlowEnabled")]
    pub standard_flow_enabled: Option<bool>,
    /// Surrogate Authentication Required option.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "surrogateAuthRequired")]
    pub surrogate_auth_required: Option<bool>,
    /// True to use a Template Config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useTemplateConfig")]
    pub use_template_config: Option<bool>,
    /// True to use Template Mappers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useTemplateMappers")]
    pub use_template_mappers: Option<bool>,
    /// True to use Template Scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useTemplateScope")]
    pub use_template_scope: Option<bool>,
    /// A list of valid Web Origins.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webOrigins")]
    pub web_origins: Option<Vec<String>>,
}

/// Authorization settings for this resource server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettings {
    /// True if resources should be managed remotely by the resource server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowRemoteResourceManagement")]
    pub allow_remote_resource_management: Option<bool>,
    /// Client ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientId")]
    pub client_id: Option<String>,
    /// The decision strategy dictates how permissions are evaluated and how a final decision is obtained. 'Affirmative' means that at least one permission must evaluate to a positive decision in order to grant access to a resource and its scopes. 'Unanimous' means that all permissions must evaluate to a positive decision in order for the final decision to be also positive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decisionStrategy")]
    pub decision_strategy: Option<String>,
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<KeycloakClientClientAuthorizationSettingsPolicies>>,
    /// The policy enforcement mode dictates how policies are enforced when evaluating authorization requests. 'Enforcing' means requests are denied by default even when there is no policy associated with a given resource. 'Permissive' means requests are allowed even when there is no policy associated with a given resource. 'Disabled' completely disables the evaluation of policies and allows access to any resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyEnforcementMode")]
    pub policy_enforcement_mode: Option<String>,
    /// Resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<KeycloakClientClientAuthorizationSettingsResources>>,
    /// Authorization Scopes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<KeycloakClientClientAuthorizationSettingsScopes>>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_policyrepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsPolicies {
    /// Config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    /// The decision strategy dictates how the policies associated with a given permission are evaluated and how a final decision is obtained. 'Affirmative' means that at least one policy must evaluate to a positive decision in order for the final decision to be also positive. 'Unanimous' means that all policies must evaluate to a positive decision in order for the final decision to be also positive. 'Consensus' means that the number of positive decisions must be greater than the number of negative decisions. If the number of positive and negative is the same, the final decision will be negative.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decisionStrategy")]
    pub decision_strategy: Option<String>,
    /// A description for this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The logic dictates how the policy decision should be made. If 'Positive', the resulting effect (permit or deny) obtained during the evaluation of this policy will be used to perform a decision. If 'Negative', the resulting effect will be negated, in other words, a permit becomes a deny and vice-versa.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
    /// The name of this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// Resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Resources Data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcesData")]
    pub resources_data: Option<Vec<KeycloakClientClientAuthorizationSettingsPoliciesResourcesData>>,
    /// Scopes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Scopes Data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopesData")]
    pub scopes_data: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// Type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_resourcerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsPoliciesResourcesData {
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<String>,
    /// The attributes associated with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// An URI pointing to an icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// True if the access to this resource can be managed by the resource owner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerManagedAccess")]
    pub owner_managed_access: Option<bool>,
    /// The scopes associated with this resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// The type of this resource. It can be used to group different resource instances with the same type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Set of URIs which are protected by resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_resourcerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsResources {
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<String>,
    /// The attributes associated with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// An URI pointing to an icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// True if the access to this resource can be managed by the resource owner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerManagedAccess")]
    pub owner_managed_access: Option<bool>,
    /// The scopes associated with this resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// The type of this resource. It can be used to group different resource instances with the same type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Set of URIs which are protected by resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_scoperepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsScopes {
    /// A unique name for this scope. The name can be used to uniquely identify a scope, useful when querying for a specific scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// An URI pointing to an icon.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iconUri")]
    pub icon_uri: Option<String>,
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A unique name for this scope. The name can be used to uniquely identify a scope, useful when querying for a specific scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<KeycloakClientClientAuthorizationSettingsScopesPolicies>>,
    /// Resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<KeycloakClientClientAuthorizationSettingsScopesResources>>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_policyrepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsScopesPolicies {
    /// Config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    /// The decision strategy dictates how the policies associated with a given permission are evaluated and how a final decision is obtained. 'Affirmative' means that at least one policy must evaluate to a positive decision in order for the final decision to be also positive. 'Unanimous' means that all policies must evaluate to a positive decision in order for the final decision to be also positive. 'Consensus' means that the number of positive decisions must be greater than the number of negative decisions. If the number of positive and negative is the same, the final decision will be negative.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decisionStrategy")]
    pub decision_strategy: Option<String>,
    /// A description for this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The logic dictates how the policy decision should be made. If 'Positive', the resulting effect (permit or deny) obtained during the evaluation of this policy will be used to perform a decision. If 'Negative', the resulting effect will be negated, in other words, a permit becomes a deny and vice-versa.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
    /// The name of this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// Resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Resources Data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcesData")]
    pub resources_data: Option<Vec<KeycloakClientClientAuthorizationSettingsScopesPoliciesResourcesData>>,
    /// Scopes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Scopes Data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopesData")]
    pub scopes_data: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// Type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_resourcerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsScopesPoliciesResourcesData {
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<String>,
    /// The attributes associated with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// An URI pointing to an icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// True if the access to this resource can be managed by the resource owner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerManagedAccess")]
    pub owner_managed_access: Option<bool>,
    /// The scopes associated with this resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// The type of this resource. It can be used to group different resource instances with the same type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Set of URIs which are protected by resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

/// https://www.keycloak.org/docs-api/12.0/rest-api/index.html#_resourcerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientAuthorizationSettingsScopesResources {
    /// ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<String>,
    /// The attributes associated with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// An URI pointing to an icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    /// A unique name for this resource. The name can be used to uniquely identify a resource, useful when querying for a specific resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// True if the access to this resource can be managed by the resource owner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerManagedAccess")]
    pub owner_managed_access: Option<bool>,
    /// The scopes associated with this resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// The type of this resource. It can be used to group different resource instances with the same type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Set of URIs which are protected by resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientClientProtocolMappers {
    /// Config options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    /// True if Consent Screen is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consentRequired")]
    pub consent_required: Option<bool>,
    /// Text to use for displaying Consent Screen.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consentText")]
    pub consent_text: Option<String>,
    /// Protocol Mapper ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Protocol Mapper Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Protocol Mapper to use
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolMapper")]
    pub protocol_mapper: Option<String>,
}

/// Selector for looking up KeycloakRealm Custom Resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientRealmSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<KeycloakClientRealmSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientRealmSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// https://www.keycloak.org/docs-api/11.0/rest-api/index.html#_rolerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientRoles {
    /// Role Attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// Client Role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientRole")]
    pub client_role: Option<bool>,
    /// Composite
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,
    /// Composites
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composites: Option<KeycloakClientRolesComposites>,
    /// Container Id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerId")]
    pub container_id: Option<String>,
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    pub name: String,
}

/// Composites
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientRolesComposites {
    /// Map client => []role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<BTreeMap<String, String>>,
    /// Realm roles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<Vec<String>>,
}

/// Scope Mappings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappings {
    /// Client Mappings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientMappings")]
    pub client_mappings: Option<BTreeMap<String, KeycloakClientScopeMappingsClientMappings>>,
    /// Realm Mappings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "realmMappings")]
    pub realm_mappings: Option<Vec<KeycloakClientScopeMappingsRealmMappings>>,
}

/// Client Mappings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappingsClientMappings {
    /// Client
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    /// ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Mappings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<KeycloakClientScopeMappingsClientMappingsMappings>>,
}

/// https://www.keycloak.org/docs-api/11.0/rest-api/index.html#_rolerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappingsClientMappingsMappings {
    /// Role Attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// Client Role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientRole")]
    pub client_role: Option<bool>,
    /// Composite
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,
    /// Composites
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composites: Option<KeycloakClientScopeMappingsClientMappingsMappingsComposites>,
    /// Container Id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerId")]
    pub container_id: Option<String>,
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    pub name: String,
}

/// Composites
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappingsClientMappingsMappingsComposites {
    /// Map client => []role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<BTreeMap<String, String>>,
    /// Realm roles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<Vec<String>>,
}

/// https://www.keycloak.org/docs-api/11.0/rest-api/index.html#_rolerepresentation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappingsRealmMappings {
    /// Role Attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// Client Role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientRole")]
    pub client_role: Option<bool>,
    /// Composite
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,
    /// Composites
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composites: Option<KeycloakClientScopeMappingsRealmMappingsComposites>,
    /// Container Id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerId")]
    pub container_id: Option<String>,
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    pub name: String,
}

/// Composites
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientScopeMappingsRealmMappingsComposites {
    /// Map client => []role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<BTreeMap<String, String>>,
    /// Realm roles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<Vec<String>>,
}

/// KeycloakClientStatus defines the observed state of KeycloakClient
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientStatus {
    /// Human-readable message indicating details about current operator phase or error.
    pub message: String,
    /// Current phase of the operator.
    pub phase: String,
    /// True if all resources are in a ready state and all work is done.
    pub ready: bool,
    /// A map of all the secondary resources types and names created for this CR. e.g "Deployment": [ "DeploymentName1", "DeploymentName2" ]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secondaryResources")]
    pub secondary_resources: Option<BTreeMap<String, String>>,
}

