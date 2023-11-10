// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/hive/hive.openshift.io/v1/syncidentityproviders.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// SyncIdentityProviderSpec defines the SyncIdentityProviderCommonSpec identity providers to sync along with ClusterDeploymentRefs indicating which clusters the SyncIdentityProvider applies to in the SyncIdentityProvider's namespace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "SyncIdentityProvider", plural = "syncidentityproviders")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct SyncIdentityProviderSpec {
    /// ClusterDeploymentRefs is the list of LocalObjectReference indicating which clusters the SyncSet applies to in the SyncSet's namespace.
    #[serde(rename = "clusterDeploymentRefs")]
    pub cluster_deployment_refs: Vec<SyncIdentityProviderClusterDeploymentRefs>,
    /// IdentityProviders is an ordered list of ways for a user to identify themselves
    #[serde(rename = "identityProviders")]
    pub identity_providers: Vec<SyncIdentityProviderIdentityProviders>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderClusterDeploymentRefs {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// IdentityProvider provides identities for users authenticating using credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProviders {
    /// basicAuth contains configuration options for the BasicAuth IdP
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<SyncIdentityProviderIdentityProvidersBasicAuth>,
    /// github enables user authentication using GitHub credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<SyncIdentityProviderIdentityProvidersGithub>,
    /// gitlab enables user authentication using GitLab credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitlab: Option<SyncIdentityProviderIdentityProvidersGitlab>,
    /// google enables user authentication using Google credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google: Option<SyncIdentityProviderIdentityProvidersGoogle>,
    /// htpasswd enables user authentication using an HTPasswd file to validate credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htpasswd: Option<SyncIdentityProviderIdentityProvidersHtpasswd>,
    /// keystone enables user authentication using keystone password credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keystone: Option<SyncIdentityProviderIdentityProvidersKeystone>,
    /// ldap enables user authentication using LDAP credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap: Option<SyncIdentityProviderIdentityProvidersLdap>,
    /// mappingMethod determines how identities from this provider are mapped to users Defaults to "claim"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mappingMethod")]
    pub mapping_method: Option<String>,
    /// name is used to qualify the identities returned by this provider. - It MUST be unique and not shared by any other identity provider used - It MUST be a valid path segment: name cannot equal "." or ".." or contain "/" or "%" or ":" Ref: https://godoc.org/github.com/openshift/origin/pkg/user/apis/user/validation#ValidateIdentityProviderName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// openID enables user authentication using OpenID credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openID")]
    pub open_id: Option<SyncIdentityProviderIdentityProvidersOpenId>,
    /// requestHeader enables user authentication using request header credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeader")]
    pub request_header: Option<SyncIdentityProviderIdentityProvidersRequestHeader>,
    /// type identifies the identity provider type for this entry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// basicAuth contains configuration options for the BasicAuth IdP
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersBasicAuth {
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersBasicAuthCa>,
    /// tlsClientCert is an optional reference to a secret by name that contains the PEM-encoded TLS client certificate to present when connecting to the server. The key "tls.crt" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsClientCert")]
    pub tls_client_cert: Option<SyncIdentityProviderIdentityProvidersBasicAuthTlsClientCert>,
    /// tlsClientKey is an optional reference to a secret by name that contains the PEM-encoded TLS private key for the client certificate referenced in tlsClientCert. The key "tls.key" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsClientKey")]
    pub tls_client_key: Option<SyncIdentityProviderIdentityProvidersBasicAuthTlsClientKey>,
    /// url is the remote URL to connect to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersBasicAuthCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// tlsClientCert is an optional reference to a secret by name that contains the PEM-encoded TLS client certificate to present when connecting to the server. The key "tls.crt" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersBasicAuthTlsClientCert {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// tlsClientKey is an optional reference to a secret by name that contains the PEM-encoded TLS private key for the client certificate referenced in tlsClientCert. The key "tls.key" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersBasicAuthTlsClientKey {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// github enables user authentication using GitHub credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGithub {
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. This can only be configured when hostname is set to a non-empty value. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersGithubCa>,
    /// clientID is the oauth client ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,
    /// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<SyncIdentityProviderIdentityProvidersGithubClientSecret>,
    /// hostname is the optional domain (e.g. "mycompany.com") for use with a hosted instance of GitHub Enterprise. It must match the GitHub Enterprise settings value configured at /setup/settings#hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// organizations optionally restricts which organizations are allowed to log in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    /// teams optionally restricts which teams are allowed to log in. Format is <org>/<team>.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. This can only be configured when hostname is set to a non-empty value. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGithubCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGithubClientSecret {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// gitlab enables user authentication using GitLab credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGitlab {
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersGitlabCa>,
    /// clientID is the oauth client ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,
    /// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<SyncIdentityProviderIdentityProvidersGitlabClientSecret>,
    /// url is the oauth server base URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGitlabCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGitlabClientSecret {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// google enables user authentication using Google credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGoogle {
    /// clientID is the oauth client ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,
    /// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<SyncIdentityProviderIdentityProvidersGoogleClientSecret>,
    /// hostedDomain is the optional Google App domain (e.g. "mycompany.com") to restrict logins to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostedDomain")]
    pub hosted_domain: Option<String>,
}

/// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersGoogleClientSecret {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// htpasswd enables user authentication using an HTPasswd file to validate credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersHtpasswd {
    /// fileData is a required reference to a secret by name containing the data to use as the htpasswd file. The key "htpasswd" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. If the specified htpasswd data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileData")]
    pub file_data: Option<SyncIdentityProviderIdentityProvidersHtpasswdFileData>,
}

/// fileData is a required reference to a secret by name containing the data to use as the htpasswd file. The key "htpasswd" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. If the specified htpasswd data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersHtpasswdFileData {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// keystone enables user authentication using keystone password credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersKeystone {
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersKeystoneCa>,
    /// domainName is required for keystone v3
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainName")]
    pub domain_name: Option<String>,
    /// tlsClientCert is an optional reference to a secret by name that contains the PEM-encoded TLS client certificate to present when connecting to the server. The key "tls.crt" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsClientCert")]
    pub tls_client_cert: Option<SyncIdentityProviderIdentityProvidersKeystoneTlsClientCert>,
    /// tlsClientKey is an optional reference to a secret by name that contains the PEM-encoded TLS private key for the client certificate referenced in tlsClientCert. The key "tls.key" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsClientKey")]
    pub tls_client_key: Option<SyncIdentityProviderIdentityProvidersKeystoneTlsClientKey>,
    /// url is the remote URL to connect to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersKeystoneCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// tlsClientCert is an optional reference to a secret by name that contains the PEM-encoded TLS client certificate to present when connecting to the server. The key "tls.crt" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersKeystoneTlsClientCert {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// tlsClientKey is an optional reference to a secret by name that contains the PEM-encoded TLS private key for the client certificate referenced in tlsClientCert. The key "tls.key" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. If the specified certificate data is not valid, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersKeystoneTlsClientKey {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// ldap enables user authentication using LDAP credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersLdap {
    /// attributes maps LDAP attributes to identities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SyncIdentityProviderIdentityProvidersLdapAttributes>,
    /// bindDN is an optional DN to bind with during the search phase.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindDN")]
    pub bind_dn: Option<String>,
    /// bindPassword is an optional reference to a secret by name containing a password to bind with during the search phase. The key "bindPassword" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindPassword")]
    pub bind_password: Option<SyncIdentityProviderIdentityProvidersLdapBindPassword>,
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersLdapCa>,
    /// insecure, if true, indicates the connection should not use TLS WARNING: Should not be set to `true` with the URL scheme "ldaps://" as "ldaps://" URLs always attempt to connect using TLS, even when `insecure` is set to `true` When `true`, "ldap://" URLS connect insecurely. When `false`, "ldap://" URLs are upgraded to a TLS connection using StartTLS as specified in https://tools.ietf.org/html/rfc2830.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// url is an RFC 2255 URL which specifies the LDAP search parameters to use. The syntax of the URL is: ldap://host:port/basedn?attribute?scope?filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// attributes maps LDAP attributes to identities
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersLdapAttributes {
    /// email is the list of attributes whose values should be used as the email address. Optional. If unspecified, no email is set for the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    /// id is the list of attributes whose values should be used as the user ID. Required. First non-empty attribute is used. At least one attribute is required. If none of the listed attribute have a value, authentication fails. LDAP standard identity attribute is "dn"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<String>>,
    /// name is the list of attributes whose values should be used as the display name. Optional. If unspecified, no display name is set for the identity LDAP standard display name attribute is "cn"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    /// preferredUsername is the list of attributes whose values should be used as the preferred username. LDAP standard login attribute is "uid"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredUsername")]
    pub preferred_username: Option<Vec<String>>,
}

/// bindPassword is an optional reference to a secret by name containing a password to bind with during the search phase. The key "bindPassword" is used to locate the data. If specified and the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersLdapBindPassword {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersLdapCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// openID enables user authentication using OpenID credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersOpenId {
    /// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersOpenIdCa>,
    /// claims mappings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<SyncIdentityProviderIdentityProvidersOpenIdClaims>,
    /// clientID is the oauth client ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,
    /// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<SyncIdentityProviderIdentityProvidersOpenIdClientSecret>,
    /// extraAuthorizeParameters are any custom parameters to add to the authorize request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraAuthorizeParameters")]
    pub extra_authorize_parameters: Option<BTreeMap<String, String>>,
    /// extraScopes are any scopes to request in addition to the standard "openid" scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraScopes")]
    pub extra_scopes: Option<Vec<String>>,
    /// issuer is the URL that the OpenID Provider asserts as its Issuer Identifier. It must use the https scheme with no query or fragment component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

/// ca is an optional reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. The key "ca.crt" is used to locate the data. If specified and the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. If empty, the default system roots are used. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersOpenIdCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// claims mappings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersOpenIdClaims {
    /// email is the list of claims whose values should be used as the email address. Optional. If unspecified, no email is set for the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    /// groups is the list of claims value of which should be used to synchronize groups from the OIDC provider to OpenShift for the user. If multiple claims are specified, the first one with a non-empty value is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// name is the list of claims whose values should be used as the display name. Optional. If unspecified, no display name is set for the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    /// preferredUsername is the list of claims whose values should be used as the preferred username. If unspecified, the preferred username is determined from the value of the sub claim
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredUsername")]
    pub preferred_username: Option<Vec<String>>,
}

/// clientSecret is a required reference to the secret by name containing the oauth client secret. The key "clientSecret" is used to locate the data. If the secret or expected key is not found, the identity provider is not honored. The namespace for this secret is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersOpenIdClientSecret {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// requestHeader enables user authentication using request header credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersRequestHeader {
    /// ca is a required reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. Specifically, it allows verification of incoming requests to prevent header spoofing. The key "ca.crt" is used to locate the data. If the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<SyncIdentityProviderIdentityProvidersRequestHeaderCa>,
    /// challengeURL is a URL to redirect unauthenticated /authorize requests to Unauthenticated requests from OAuth clients which expect WWW-Authenticate challenges will be redirected here. ${url} is replaced with the current URL, escaped to be safe in a query parameter https://www.example.com/sso-login?then=${url} ${query} is replaced with the current query string https://www.example.com/auth-proxy/oauth/authorize?${query} Required when challenge is set to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "challengeURL")]
    pub challenge_url: Option<String>,
    /// clientCommonNames is an optional list of common names to require a match from. If empty, any client certificate validated against the clientCA bundle is considered authoritative.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCommonNames")]
    pub client_common_names: Option<Vec<String>>,
    /// emailHeaders is the set of headers to check for the email address
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailHeaders")]
    pub email_headers: Option<Vec<String>>,
    /// headers is the set of headers to check for identity information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// loginURL is a URL to redirect unauthenticated /authorize requests to Unauthenticated requests from OAuth clients which expect interactive logins will be redirected here ${url} is replaced with the current URL, escaped to be safe in a query parameter https://www.example.com/sso-login?then=${url} ${query} is replaced with the current query string https://www.example.com/auth-proxy/oauth/authorize?${query} Required when login is set to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loginURL")]
    pub login_url: Option<String>,
    /// nameHeaders is the set of headers to check for the display name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameHeaders")]
    pub name_headers: Option<Vec<String>>,
    /// preferredUsernameHeaders is the set of headers to check for the preferred username
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredUsernameHeaders")]
    pub preferred_username_headers: Option<Vec<String>>,
}

/// ca is a required reference to a config map by name containing the PEM-encoded CA bundle. It is used as a trust anchor to validate the TLS certificate presented by the remote server. Specifically, it allows verification of incoming requests to prevent header spoofing. The key "ca.crt" is used to locate the data. If the config map or expected key is not found, the identity provider is not honored. If the specified ca data is not valid, the identity provider is not honored. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderIdentityProvidersRequestHeaderCa {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// IdentityProviderStatus defines the observed state of SyncSet
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyncIdentityProviderStatus {
}

