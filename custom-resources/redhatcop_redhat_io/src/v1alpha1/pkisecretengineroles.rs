// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/pkisecretengineroles.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// PKISecretEngineRoleSpec defines the desired state of PKISecretEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "PKISecretEngineRole", plural = "pkisecretengineroles")]
#[kube(namespaced)]
#[kube(status = "PKISecretEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PKISecretEngineRoleSpec {
    /// Specifies the Time To Live value provided as a string duration with time suffix. Hour is the largest suffix. If not set, uses the system default value or the value of max_ttl, whichever is shorter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "TTL")]
    pub ttl: Option<String>,
    /// Specifies if clients can request any CN. Useful in some circumstances, but make sure you understand whether it is appropriate for your installation before enabling it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowAnyName")]
    pub allow_any_name: Option<bool>,
    /// Specifies if clients can request certificates matching the value of the actual domains themselves; e.g. if a configured domain set with allowed_domains is example.com, this allows clients to actually request a certificate containing the name example.com as one of the DNS values on the final certificate. In some scenarios, this can be considered a security risk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowBareDomains")]
    pub allow_bare_domains: Option<bool>,
    /// Allows names specified in allowed_domains to contain glob patterns (e.g. ftp*.example.com). Clients will be allowed to request certificates with names matching the glob patterns.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowGlobDomains")]
    pub allow_glob_domains: Option<bool>,
    /// Specifies if clients can request IP Subject Alternative Names. No authorization checking is performed except to verify that the given values are valid IP addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowIPSans")]
    pub allow_ip_sans: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowLocalhost")]
    pub allow_localhost: Option<bool>,
    /// Specifies if clients can request certificates with CNs that are subdomains of the CNs allowed by the other role options. This includes wildcard subdomains. For example, an allowed_domains value of example.com with this option set to true will allow foo.example.com and bar.example.com as well as *.example.com. This is redundant when using the allow_any_name option.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowSubdomains")]
    pub allow_subdomains: Option<bool>,
    /// Specifies the domains of the role. This is used with the allow_bare_domains and allow_subdomains options. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedDomains")]
    pub allowed_domains: Option<Vec<String>>,
    /// When set, allowed_domains may contain templates, as with ACL Path Templating.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedDomainsTemplate")]
    pub allowed_domains_template: Option<bool>,
    /// Defines allowed custom OID/UTF8-string SANs. This can be a comma-delimited list or a JSON string slice, where each element has the same format as OpenSSL: <oid>;<type>:<value>, but the only valid type is UTF8 or UTF-8. The value part of an element may be a * to allow any value with that OID. Alternatively, specifying a single * will allow any other_sans input.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedOtherSans")]
    pub allowed_other_sans: Option<String>,
    /// Defines allowed URI Subject Alternative Names. No authorization checking is performed except to verify that the given values are valid URIs. This can be a comma-delimited list or a JSON string slice. Values can contain glob patterns (e.g. spiffe://hostname/*). kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedURISans")]
    pub allowed_uri_sans: Option<Vec<String>>,
    /// Authentication is the kube auth configuration to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<PKISecretEngineRoleAuthentication>,
    /// Mark Basic Constraints valid when issuing non-CA certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicConstraintsValidForNonCa")]
    pub basic_constraints_valid_for_non_ca: Option<bool>,
    /// Specifies if certificates are flagged for client use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientFlag")]
    pub client_flag: Option<bool>,
    /// Specifies if certificates are flagged for code signing use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSigningFlag")]
    pub code_signing_flag: Option<bool>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<PKISecretEngineRoleConnection>,
    /// Specifies the C (Country) values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Specifies if certificates are flagged for email protection use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailProtectionFlag")]
    pub email_protection_flag: Option<bool>,
    /// Specifies if only valid host names are allowed for CNs, DNS SANs, and the host part of email addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enforceHostnames")]
    pub enforce_hostnames: Option<bool>,
    /// Specifies the allowed extended key usage constraint on issued certificates. Valid values can be found at https://golang.org/pkg/crypto/x509/#ExtKeyUsage - simply drop the ExtKeyUsage part of the value. Values are not case-sensitive. To specify no key usage constraints, set this to an empty list. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extKeyUsage")]
    pub ext_key_usage: Option<Vec<String>>,
    /// A comma-separated string or list of extended key usage oids. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extKeyUsageOids")]
    pub ext_key_usage_oids: Option<Vec<String>>,
    /// Specifies if certificates issued/signed against this role will have Vault leases attached to them. Certificates can be added to the CRL by vault revoke <lease_id> when certificates are associated with leases. It can also be done using the pki/revoke endpoint. However, when lease generation is disabled, invoking pki/revoke would be the only way to add the certificates to the CRL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateLease")]
    pub generate_lease: Option<bool>,
    /// Specifies the number of bits to use for the generated keys. This will need to be changed for ec keys, e.g., 224, 256, 384 or 521.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyBits")]
    pub key_bits: Option<i64>,
    /// Specifies the type of key to generate for generated private keys and the type of key expected for submitted CSRs. Currently, rsa and ec are supported, or when signing CSRs any can be specified to allow keys of either type and with any bit size (subject to > 1024 bits for RSA keys).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyType")]
    pub key_type: Option<PKISecretEngineRoleKeyType>,
    /// Specifies the allowed key usage constraint on issued certificates. Valid values can be found at https://golang.org/pkg/crypto/x509/#KeyUsage - simply drop the KeyUsage part of the value. Values are not case-sensitive. To specify no key usage constraints, set this to an empty list. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyUsage")]
    pub key_usage: Option<Vec<String>>,
    /// Specifies the L (Locality) values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Specifies the maximum Time To Live provided as a string duration with time suffix. Hour is the largest suffix. If not set, defaults to the system maximum lease TTL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<String>,
    /// The name of the obejct created in Vault. If this is specified it takes precedence over {metatada.name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If set, certificates issued/signed against this role will not be stored in the storage backend. This can improve performance when issuing large numbers of certificates. However, certificates issued in this way cannot be enumerated or revoked, so this option is recommended only for certificates that are non-sensitive, or extremely short-lived. This option implies a value of false for generate_lease.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noStore")]
    pub no_store: Option<bool>,
    /// Specifies the duration by which to backdate the NotBefore property.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBeforeDuration")]
    pub not_before_duration: Option<String>,
    /// Specifies the O (Organization) values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Specifies the OU (OrganizationalUnit) values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ou: Option<String>,
    /// Path at which to create the role. The final path in Vault will be {[spec.authentication.namespace]}/{spec.path}/roles/{metadata.name}. The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// A comma-separated string or list of policy OIDs. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyIdentifiers")]
    pub policy_identifiers: Option<Vec<String>>,
    /// Specifies the Postal Code values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postalCode")]
    pub postal_code: Option<String>,
    /// Specifies the ST (Province) values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// If set to false, makes the common_name field optional while generating a certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireCn")]
    pub require_cn: Option<bool>,
    /// Specifies the Serial Number, if any. Otherwise Vault will generate a random serial for you. If you want more than one, specify alternative names in the alt_names map using OID 2.5.4.5.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    /// Specifies if certificates are flagged for server use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverFlag")]
    pub server_flag: Option<bool>,
    /// Specifies the Street Address values in the subject field of issued certificates. This is a comma-separated string or JSON array.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streetAddress")]
    pub street_address: Option<String>,
    /// When used with the CSR signing endpoint, the common name in the CSR will be used instead of taken from the JSON data. This does not include any requested SANs in the CSR; use use_csr_sans for that.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useCSRCommonName")]
    pub use_csr_common_name: Option<bool>,
    /// When used with the CSR signing endpoint, the subject alternate names in the CSR will be used instead of taken from the JSON data. This does not include the common name in the CSR; use use_csr_common_name for that.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useCSRSans")]
    pub use_csr_sans: Option<bool>,
}

/// Authentication is the kube auth configuration to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleAuthentication {
    /// Namespace is the Vault namespace to be used in all the operations withing this connection/authentication. Only available in Vault Enterprise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Path is the path of the role used for this kube auth authentication. The operator will try to authenticate at {[namespace/]}auth/{spec.path}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Role the role to be used during authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// ServiceAccount is the service account used for the kube auth authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<PKISecretEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleAuthenticationServiceAccount {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<PKISecretEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<PKISecretEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PKISecretEngineRoleSpec defines the desired state of PKISecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PKISecretEngineRoleKeyType {
    #[serde(rename = "rsa")]
    Rsa,
    #[serde(rename = "ec")]
    Ec,
}

/// PKISecretEngineRoleStatus defines the observed state of PKISecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PKISecretEngineRoleStatus {
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

