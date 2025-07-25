// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/3scale/apicast-operator/apps.3scale.net/v1alpha1/apicasts.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// APIcastSpec defines the desired state of APIcast.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.3scale.net", version = "v1alpha1", kind = "APIcast", plural = "apicasts")]
#[kube(namespaced)]
#[kube(status = "APIcastStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APIcastSpec {
    /// Secret reference to a Kubernetes Secret containing the admin portal
    /// endpoint URL. The Secret must be located in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPortalCredentialsRef")]
    pub admin_portal_credentials_ref: Option<APIcastAdminPortalCredentialsRef>,
    /// AllProxy specifies a HTTP(S) proxy to be used for connecting to services if
    /// a protocol-specific proxy is not specified. Authentication is not supported.
    /// Format is <scheme>://<host>:<port>
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allProxy")]
    pub all_proxy: Option<String>,
    /// CACertificateSecretRef references secret containing the X.509 CA certificate in the PEM format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateSecretRef")]
    pub ca_certificate_secret_ref: Option<APIcastCaCertificateSecretRef>,
    /// The period (in seconds) that the APIcast configuration will be stored in
    /// APIcast's cache.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheConfigurationSeconds")]
    pub cache_configuration_seconds: Option<i64>,
    /// CacheMaxTime indicates the maximum time to be cached. If cache-control header is not set, the time to be cached will be the defined one.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheMaxTime")]
    pub cache_max_time: Option<String>,
    /// CacheStatusCodes defines the status codes for which the response content will be cached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheStatusCodes")]
    pub cache_status_codes: Option<String>,
    /// ConfigurationLoadMode can be used to set APIcast's configuration load mode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationLoadMode")]
    pub configuration_load_mode: Option<APIcastConfigurationLoadMode>,
    /// CustomEnvironments specifies an array of defined custome environments to be loaded
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customEnvironments")]
    pub custom_environments: Option<Vec<APIcastCustomEnvironments>>,
    /// CustomPolicies specifies an array of defined custome policies to be loaded
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customPolicies")]
    pub custom_policies: Option<Vec<APIcastCustomPolicies>>,
    /// DeploymentEnvironment is the environment for which the configuration will
    /// be downloaded from 3scale (Staging or Production), when using APIcast.
    /// The value will also be used in the header X-3scale-User-Agent in the
    /// authorize/report requests made to 3scale Service Management API. It is
    /// used by 3scale for statistics.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentEnvironment")]
    pub deployment_environment: Option<String>,
    /// DNSResolverAddress can be used to specify a custom DNS resolver address
    /// to be used by OpenResty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsResolverAddress")]
    pub dns_resolver_address: Option<String>,
    /// Secret reference to a Kubernetes secret containing the gateway
    /// configuration. The Secret must be located in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "embeddedConfigurationSecretRef")]
    pub embedded_configuration_secret_ref: Option<APIcastEmbeddedConfigurationSecretRef>,
    /// EnabledServices can be used to specify a list of service IDs used to
    /// filter the configured services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enabledServices")]
    pub enabled_services: Option<Vec<String>>,
    /// ExposedHost is the domain name used for external access. By default no
    /// external access is configured.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposedHost")]
    pub exposed_host: Option<APIcastExposedHost>,
    /// ExtendedMetrics enables additional information on Prometheus metrics; some labels will be used with specific information that will provide more in-depth details about APIcast.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extendedMetrics")]
    pub extended_metrics: Option<bool>,
    /// Enables/disables HPA
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hpa: Option<bool>,
    /// HTTPProxy specifies a HTTP(S) Proxy to be used for connecting to HTTP services.
    /// Authentication is not supported. Format is <scheme>://<host>:<port>
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpProxy")]
    pub http_proxy: Option<String>,
    /// HTTPSCertificateSecretRef references secret containing the X.509 certificate in the PEM format and the X.509 certificate secret key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsCertificateSecretRef")]
    pub https_certificate_secret_ref: Option<APIcastHttpsCertificateSecretRef>,
    /// HttpsPort controls on which port APIcast should start listening for HTTPS connections. If this clashes with HTTP port it will be used only for HTTPS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsPort")]
    pub https_port: Option<i32>,
    /// HTTPSProxy specifies a HTTP(S) Proxy to be used for connecting to HTTPS services.
    /// Authentication is not supported. Format is <scheme>://<host>:<port>
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsProxy")]
    pub https_proxy: Option<String>,
    /// HTTPSVerifyDepth defines the maximum length of the client certificate chain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsVerifyDepth")]
    pub https_verify_depth: Option<i64>,
    /// Image allows overriding the default APIcast gateway container image.
    /// This setting should only be used for dev/testing purposes. Setting
    /// this disables automated upgrades of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// LoadServicesWhenNeeded makes the configurations to be loaded lazily. APIcast will only load the ones configured for the host specified in the host header of the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadServicesWhenNeeded")]
    pub load_services_when_needed: Option<bool>,
    /// LogLevel controls the log level of APIcast's OpenResty logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<APIcastLogLevel>,
    /// ManagementAPIScope controls APIcast Management API scope. The Management
    /// API is powerful and can control the APIcast configuration. debug level
    /// should only be enabled for debugging purposes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementAPIScope")]
    pub management_api_scope: Option<APIcastManagementApiScope>,
    /// NoProxy specifies a comma-separated list of hostnames and domain
    /// names for which the requests should not be proxied. Setting to a single
    /// * character, which matches all hosts, effectively disables the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noProxy")]
    pub no_proxy: Option<String>,
    /// OidcLogLevel allows to set the log level for the logs related to OpenID Connect integration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oidcLogLevel")]
    pub oidc_log_level: Option<APIcastOidcLogLevel>,
    /// OpenSSLPeerVerificationEnabled controls OpenSSL peer verification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openSSLPeerVerificationEnabled")]
    pub open_ssl_peer_verification_enabled: Option<bool>,
    /// OpenTelemetry contains the gateway instrumentation configuration
    /// with APIcast.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<APIcastOpenTelemetry>,
    /// OpenTracingSpec contains the OpenTracing integration configuration
    /// with APIcast.
    /// Deprecated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTracing")]
    pub open_tracing: Option<APIcastOpenTracing>,
    /// PathRoutingEnabled can be used to enable APIcast's path-based routing
    /// in addition to to the default host-based routing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pathRoutingEnabled")]
    pub path_routing_enabled: Option<bool>,
    /// Number of replicas of the APIcast Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// Resources can be used to set custom compute Kubernetes Resource
    /// Requirements for the APIcast deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<APIcastResources>,
    /// ResponseCodesIncluded can be set to log the response codes of the responses
    /// in Apisonator, so they can then be visualized in the 3scale admin portal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseCodesIncluded")]
    pub response_codes_included: Option<bool>,
    /// Kubernetes Service Account name to be used for the APIcast Deployment. The
    /// Service Account must exist beforehand.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// ServiceCacheSize specifies the number of services that APICast can store in the internal cache
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceCacheSize")]
    pub service_cache_size: Option<i32>,
    /// ServiceConfigurationVersionOverride contains service configuration version map to prevent it from auto-updating.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceConfigurationVersionOverride")]
    pub service_configuration_version_override: Option<BTreeMap<String, String>>,
    /// ServicesFilterByURL is used to filter the service configured in the 3scale API Manager, the filter matches with the public base URL (Staging or production).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicesFilterByURL")]
    pub services_filter_by_url: Option<String>,
    /// Timezone specifies the local timezone of the APIcast deployment pods. A timezone value available in the TZ database must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UpstreamRetryCases Used only when the retry policy is configured. Specified in which cases a request to the upstream API should be retried.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamRetryCases")]
    pub upstream_retry_cases: Option<APIcastUpstreamRetryCases>,
    /// Workers defines the number of APIcast's worker processes per pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
}

/// Secret reference to a Kubernetes Secret containing the admin portal
/// endpoint URL. The Secret must be located in the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastAdminPortalCredentialsRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CACertificateSecretRef references secret containing the X.509 CA certificate in the PEM format.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastCaCertificateSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// APIcastSpec defines the desired state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APIcastConfigurationLoadMode {
    #[serde(rename = "boot")]
    Boot,
    #[serde(rename = "lazy")]
    Lazy,
}

/// CustomEnvironmentSpec contains or has reference to an APIcast custom environment
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastCustomEnvironments {
    /// LocalObjectReference contains enough information to let you locate the
    /// referenced object inside the same namespace.
    #[serde(rename = "secretRef")]
    pub secret_ref: APIcastCustomEnvironmentsSecretRef,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastCustomEnvironmentsSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CustomPolicySpec contains or has reference to an APIcast custom policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastCustomPolicies {
    /// Name specifies the name of the custom policy
    pub name: String,
    /// SecretRef specifies the secret holding the custom policy metadata and lua code
    #[serde(rename = "secretRef")]
    pub secret_ref: APIcastCustomPoliciesSecretRef,
    /// Version specifies the name of the custom policy
    pub version: String,
}

/// SecretRef specifies the secret holding the custom policy metadata and lua code
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastCustomPoliciesSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Secret reference to a Kubernetes secret containing the gateway
/// configuration. The Secret must be located in the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastEmbeddedConfigurationSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ExposedHost is the domain name used for external access. By default no
/// external access is configured.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastExposedHost {
    pub host: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<APIcastExposedHostTls>>,
}

/// IngressTLS describes the transport layer security associated with an ingress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastExposedHostTls {
    /// hosts is a list of hosts included in the TLS certificate. The values in
    /// this list must match the name/s used in the tlsSecret. Defaults to the
    /// wildcard host setting for the loadbalancer controller fulfilling this
    /// Ingress, if left unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// secretName is the name of the secret used to terminate TLS traffic on
    /// port 443. Field is left optional to allow TLS routing based on SNI
    /// hostname alone. If the SNI host in a listener conflicts with the "Host"
    /// header field used by an IngressRule, the SNI host is used for termination
    /// and value of the "Host" header is used for routing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// HTTPSCertificateSecretRef references secret containing the X.509 certificate in the PEM format and the X.509 certificate secret key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastHttpsCertificateSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// APIcastSpec defines the desired state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APIcastLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emerg")]
    Emerg,
}

/// APIcastSpec defines the desired state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APIcastManagementApiScope {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "policies")]
    Policies,
    #[serde(rename = "debug")]
    Debug,
}

/// APIcastSpec defines the desired state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APIcastOidcLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emerg")]
    Emerg,
}

/// OpenTelemetry contains the gateway instrumentation configuration
/// with APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastOpenTelemetry {
    /// Enabled controls whether OpenTelemetry integration with APIcast is enabled.
    /// By default it is not enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// TracingConfigSecretKey contains the key of the secret to select the configuration from.
    /// if unspecified, the first secret key in lexicographical order will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfigSecretKey")]
    pub tracing_config_secret_key: Option<String>,
    /// TracingConfigSecretRef contains a Secret reference the Opentelemetry configuration.
    /// The configuration file specification is defined in the Nginx instrumentation library repo
    /// https://github.com/open-telemetry/opentelemetry-cpp-contrib/tree/main/instrumentation/nginx
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfigSecretRef")]
    pub tracing_config_secret_ref: Option<APIcastOpenTelemetryTracingConfigSecretRef>,
}

/// TracingConfigSecretRef contains a Secret reference the Opentelemetry configuration.
/// The configuration file specification is defined in the Nginx instrumentation library repo
/// https://github.com/open-telemetry/opentelemetry-cpp-contrib/tree/main/instrumentation/nginx
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastOpenTelemetryTracingConfigSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// OpenTracingSpec contains the OpenTracing integration configuration
/// with APIcast.
/// Deprecated
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastOpenTracing {
    /// Enabled controls whether OpenTracing integration with APIcast is enabled.
    /// By default it is not enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// TracingConfigSecretRef contains a Secret reference the OpenTracing configuration.
    /// Each supported tracing library provides a default configuration file
    /// that is used if TracingConfig is not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfigSecretRef")]
    pub tracing_config_secret_ref: Option<APIcastOpenTracingTracingConfigSecretRef>,
    /// TracingLibrary controls which OpenTracing library is loaded. At the moment
    /// the only supported tracer is `jaeger`. If not set, `jaeger` will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingLibrary")]
    pub tracing_library: Option<String>,
}

/// TracingConfigSecretRef contains a Secret reference the OpenTracing configuration.
/// Each supported tracing library provides a default configuration file
/// that is used if TracingConfig is not specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastOpenTracingTracingConfigSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Resources can be used to set custom compute Kubernetes Resource
/// Requirements for the APIcast deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<APIcastResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// APIcastSpec defines the desired state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APIcastUpstreamRetryCases {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "invalid_header")]
    InvalidHeader,
    #[serde(rename = "http_500")]
    Http500,
    #[serde(rename = "http_502")]
    Http502,
    #[serde(rename = "http_503")]
    Http503,
    #[serde(rename = "http_504")]
    Http504,
    #[serde(rename = "http_403")]
    Http403,
    #[serde(rename = "http_404")]
    Http404,
    #[serde(rename = "http_429")]
    Http429,
    #[serde(rename = "non_idempotent")]
    NonIdempotent,
    #[serde(rename = "off")]
    Off,
}

/// APIcastStatus defines the observed state of APIcast.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIcastStatus {
    /// Represents the observations of a foo's current state.
    /// Known .status.conditions.type are: "Available"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The image being used in the APIcast deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ObservedGeneration reflects the generation of the most recently observed spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

