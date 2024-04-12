// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/prometheus-operator/prometheus-operator/monitoring.coreos.com/v1/probes.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Specification of desired Ingress selection for target discovery by Prometheus.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "monitoring.coreos.com", version = "v1", kind = "Probe", plural = "probes")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ProbeSpec {
    /// Authorization section for this endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ProbeAuthorization>,
    /// BasicAuth allow an endpoint to authenticate over basic authentication.
    /// More info: https://prometheus.io/docs/operating/configuration/#endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<ProbeBasicAuth>,
    /// Secret to mount to read bearer token for scraping targets. The secret
    /// needs to be in the same namespace as the probe and accessible by
    /// the Prometheus Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bearerTokenSecret")]
    pub bearer_token_secret: Option<ProbeBearerTokenSecret>,
    /// Interval at which targets are probed using the configured prober.
    /// If not specified Prometheus' global scrape interval is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// The job name assigned to scraped metrics by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobName")]
    pub job_name: Option<String>,
    /// Per-scrape limit on the number of targets dropped by relabeling
    /// that will be kept in memory. 0 means no limit.
    /// 
    /// 
    /// It requires Prometheus >= v2.47.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepDroppedTargets")]
    pub keep_dropped_targets: Option<i64>,
    /// Per-scrape limit on number of labels that will be accepted for a sample.
    /// Only valid in Prometheus versions 2.27.0 and newer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelLimit")]
    pub label_limit: Option<i64>,
    /// Per-scrape limit on length of labels name that will be accepted for a sample.
    /// Only valid in Prometheus versions 2.27.0 and newer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelNameLengthLimit")]
    pub label_name_length_limit: Option<i64>,
    /// Per-scrape limit on length of labels value that will be accepted for a sample.
    /// Only valid in Prometheus versions 2.27.0 and newer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelValueLengthLimit")]
    pub label_value_length_limit: Option<i64>,
    /// MetricRelabelConfigs to apply to samples before ingestion.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricRelabelings")]
    pub metric_relabelings: Option<Vec<ProbeMetricRelabelings>>,
    /// The module to use for probing specifying how to probe the target.
    /// Example module configuring in the blackbox exporter:
    /// https://github.com/prometheus/blackbox_exporter/blob/master/example.yml
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<ProbeOauth2>,
    /// Specification for the prober to use for probing targets.
    /// The prober.URL parameter is required. Targets cannot be probed if left empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prober: Option<ProbeProber>,
    /// SampleLimit defines per-scrape limit on number of scraped samples that will be accepted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sampleLimit")]
    pub sample_limit: Option<i64>,
    /// The scrape class to apply.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeClass")]
    pub scrape_class: Option<String>,
    /// `scrapeProtocols` defines the protocols to negotiate during a scrape. It tells clients the
    /// protocols supported by Prometheus in order of preference (from most to least preferred).
    /// 
    /// 
    /// If unset, Prometheus uses its default value.
    /// 
    /// 
    /// It requires Prometheus >= v2.49.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeProtocols")]
    pub scrape_protocols: Option<Vec<String>>,
    /// Timeout for scraping metrics from the Prometheus exporter.
    /// If not specified, the Prometheus global scrape timeout is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeTimeout")]
    pub scrape_timeout: Option<String>,
    /// TargetLimit defines a limit on the number of scraped targets that will be accepted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLimit")]
    pub target_limit: Option<i64>,
    /// Targets defines a set of static or dynamically discovered targets to probe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<ProbeTargets>,
    /// TLS configuration to use when scraping the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsConfig")]
    pub tls_config: Option<ProbeTlsConfig>,
}

/// Authorization section for this endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeAuthorization {
    /// Selects a key of a Secret in the namespace that contains the credentials for authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ProbeAuthorizationCredentials>,
    /// Defines the authentication type. The value is case-insensitive.
    /// 
    /// 
    /// "Basic" is not a supported value.
    /// 
    /// 
    /// Default: "Bearer"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Selects a key of a Secret in the namespace that contains the credentials for authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeAuthorizationCredentials {
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

/// BasicAuth allow an endpoint to authenticate over basic authentication.
/// More info: https://prometheus.io/docs/operating/configuration/#endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeBasicAuth {
    /// `password` specifies a key of a Secret containing the password for
    /// authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<ProbeBasicAuthPassword>,
    /// `username` specifies a key of a Secret containing the username for
    /// authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<ProbeBasicAuthUsername>,
}

/// `password` specifies a key of a Secret containing the password for
/// authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeBasicAuthPassword {
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

/// `username` specifies a key of a Secret containing the username for
/// authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeBasicAuthUsername {
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

/// Secret to mount to read bearer token for scraping targets. The secret
/// needs to be in the same namespace as the probe and accessible by
/// the Prometheus Operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeBearerTokenSecret {
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

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeMetricRelabelings {
    /// Action to perform based on the regex matching.
    /// 
    /// 
    /// `Uppercase` and `Lowercase` actions require Prometheus >= v2.36.0.
    /// `DropEqual` and `KeepEqual` actions require Prometheus >= v2.41.0.
    /// 
    /// 
    /// Default: "Replace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProbeMetricRelabelingsAction>,
    /// Modulus to take of the hash of the source label values.
    /// 
    /// 
    /// Only applicable when the action is `HashMod`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a Replace action is performed if the
    /// regular expression matches.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator is the string between concatenated SourceLabels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is
    /// concatenated using the configured Separator and matched against the
    /// configured regular expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<Vec<String>>,
    /// Label to which the resulting string is written in a replacement.
    /// 
    /// 
    /// It is mandatory for `Replace`, `HashMod`, `Lowercase`, `Uppercase`,
    /// `KeepEqual` and `DropEqual` actions.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProbeMetricRelabelingsAction {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "Replace")]
    ReplaceX,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "Keep")]
    KeepX,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "Drop")]
    DropX,
    #[serde(rename = "hashmod")]
    Hashmod,
    HashMod,
    #[serde(rename = "labelmap")]
    Labelmap,
    LabelMap,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    LabelDrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    LabelKeep,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "Lowercase")]
    LowercaseX,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "Uppercase")]
    UppercaseX,
    #[serde(rename = "keepequal")]
    Keepequal,
    KeepEqual,
    #[serde(rename = "dropequal")]
    Dropequal,
    DropEqual,
}

/// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeOauth2 {
    /// `clientId` specifies a key of a Secret or ConfigMap containing the
    /// OAuth2 client's ID.
    #[serde(rename = "clientId")]
    pub client_id: ProbeOauth2ClientId,
    /// `clientSecret` specifies a key of a Secret containing the OAuth2
    /// client's secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: ProbeOauth2ClientSecret,
    /// `endpointParams` configures the HTTP parameters to append to the token
    /// URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointParams")]
    pub endpoint_params: Option<BTreeMap<String, String>>,
    /// `scopes` defines the OAuth2 scopes used for the token request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// `tokenURL` configures the URL to fetch the token from.
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
}

/// `clientId` specifies a key of a Secret or ConfigMap containing the
/// OAuth2 client's ID.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeOauth2ClientId {
    /// ConfigMap containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ProbeOauth2ClientIdConfigMap>,
    /// Secret containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ProbeOauth2ClientIdSecret>,
}

/// ConfigMap containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeOauth2ClientIdConfigMap {
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

/// Secret containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeOauth2ClientIdSecret {
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

/// `clientSecret` specifies a key of a Secret containing the OAuth2
/// client's secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeOauth2ClientSecret {
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

/// Specification for the prober to use for probing targets.
/// The prober.URL parameter is required. Targets cannot be probed if left empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeProber {
    /// Path to collect metrics from.
    /// Defaults to `/probe`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional ProxyURL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyUrl")]
    pub proxy_url: Option<String>,
    /// HTTP scheme to use for scraping.
    /// `http` and `https` are the expected values unless you rewrite the `__scheme__` label via relabeling.
    /// If empty, Prometheus uses the default value `http`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<ProbeProberScheme>,
    /// Mandatory URL of the prober.
    pub url: String,
}

/// Specification for the prober to use for probing targets.
/// The prober.URL parameter is required. Targets cannot be probed if left empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProbeProberScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

/// Targets defines a set of static or dynamically discovered targets to probe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargets {
    /// ingress defines the Ingress objects to probe and the relabeling
    /// configuration.
    /// If `staticConfig` is also defined, `staticConfig` takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<ProbeTargetsIngress>,
    /// staticConfig defines the static list of targets to probe and the
    /// relabeling configuration.
    /// If `ingress` is also defined, `staticConfig` takes precedence.
    /// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#static_config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticConfig")]
    pub static_config: Option<ProbeTargetsStaticConfig>,
}

/// ingress defines the Ingress objects to probe and the relabeling
/// configuration.
/// If `staticConfig` is also defined, `staticConfig` takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsIngress {
    /// From which namespaces to select Ingress objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ProbeTargetsIngressNamespaceSelector>,
    /// RelabelConfigs to apply to the label set of the target before it gets
    /// scraped.
    /// The original ingress address is available via the
    /// `__tmp_prometheus_ingress_address` label. It can be used to customize the
    /// probed URL.
    /// The original scrape job's name is available via the `__tmp_prometheus_job_name` label.
    /// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relabelingConfigs")]
    pub relabeling_configs: Option<Vec<ProbeTargetsIngressRelabelingConfigs>>,
    /// Selector to select the Ingress objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ProbeTargetsIngressSelector>,
}

/// From which namespaces to select Ingress objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsIngressNamespaceSelector {
    /// Boolean describing whether all namespaces are selected in contrast to a
    /// list restricting them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any: Option<bool>,
    /// List of namespace names to select from.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchNames")]
    pub match_names: Option<Vec<String>>,
}

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsIngressRelabelingConfigs {
    /// Action to perform based on the regex matching.
    /// 
    /// 
    /// `Uppercase` and `Lowercase` actions require Prometheus >= v2.36.0.
    /// `DropEqual` and `KeepEqual` actions require Prometheus >= v2.41.0.
    /// 
    /// 
    /// Default: "Replace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProbeTargetsIngressRelabelingConfigsAction>,
    /// Modulus to take of the hash of the source label values.
    /// 
    /// 
    /// Only applicable when the action is `HashMod`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a Replace action is performed if the
    /// regular expression matches.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator is the string between concatenated SourceLabels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is
    /// concatenated using the configured Separator and matched against the
    /// configured regular expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<Vec<String>>,
    /// Label to which the resulting string is written in a replacement.
    /// 
    /// 
    /// It is mandatory for `Replace`, `HashMod`, `Lowercase`, `Uppercase`,
    /// `KeepEqual` and `DropEqual` actions.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProbeTargetsIngressRelabelingConfigsAction {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "Replace")]
    ReplaceX,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "Keep")]
    KeepX,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "Drop")]
    DropX,
    #[serde(rename = "hashmod")]
    Hashmod,
    HashMod,
    #[serde(rename = "labelmap")]
    Labelmap,
    LabelMap,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    LabelDrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    LabelKeep,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "Lowercase")]
    LowercaseX,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "Uppercase")]
    UppercaseX,
    #[serde(rename = "keepequal")]
    Keepequal,
    KeepEqual,
    #[serde(rename = "dropequal")]
    Dropequal,
    DropEqual,
}

/// Selector to select the Ingress objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsIngressSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ProbeTargetsIngressSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsIngressSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// staticConfig defines the static list of targets to probe and the
/// relabeling configuration.
/// If `ingress` is also defined, `staticConfig` takes precedence.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#static_config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsStaticConfig {
    /// Labels assigned to all metrics scraped from the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// RelabelConfigs to apply to the label set of the targets before it gets
    /// scraped.
    /// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relabelingConfigs")]
    pub relabeling_configs: Option<Vec<ProbeTargetsStaticConfigRelabelingConfigs>>,
    /// The list of hosts to probe.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "static")]
    pub r#static: Option<Vec<String>>,
}

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTargetsStaticConfigRelabelingConfigs {
    /// Action to perform based on the regex matching.
    /// 
    /// 
    /// `Uppercase` and `Lowercase` actions require Prometheus >= v2.36.0.
    /// `DropEqual` and `KeepEqual` actions require Prometheus >= v2.41.0.
    /// 
    /// 
    /// Default: "Replace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProbeTargetsStaticConfigRelabelingConfigsAction>,
    /// Modulus to take of the hash of the source label values.
    /// 
    /// 
    /// Only applicable when the action is `HashMod`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a Replace action is performed if the
    /// regular expression matches.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator is the string between concatenated SourceLabels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is
    /// concatenated using the configured Separator and matched against the
    /// configured regular expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<Vec<String>>,
    /// Label to which the resulting string is written in a replacement.
    /// 
    /// 
    /// It is mandatory for `Replace`, `HashMod`, `Lowercase`, `Uppercase`,
    /// `KeepEqual` and `DropEqual` actions.
    /// 
    /// 
    /// Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set for targets, alerts,
/// scraped samples and remote write samples.
/// 
/// 
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProbeTargetsStaticConfigRelabelingConfigsAction {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "Replace")]
    ReplaceX,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "Keep")]
    KeepX,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "Drop")]
    DropX,
    #[serde(rename = "hashmod")]
    Hashmod,
    HashMod,
    #[serde(rename = "labelmap")]
    Labelmap,
    LabelMap,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    LabelDrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    LabelKeep,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "Lowercase")]
    LowercaseX,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "Uppercase")]
    UppercaseX,
    #[serde(rename = "keepequal")]
    Keepequal,
    KeepEqual,
    #[serde(rename = "dropequal")]
    Dropequal,
    DropEqual,
}

/// TLS configuration to use when scraping the endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfig {
    /// Certificate authority used when verifying server certificates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<ProbeTlsConfigCa>,
    /// Client certificate to present when doing client-authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<ProbeTlsConfigCert>,
    /// Disable target certificate validation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipVerify")]
    pub insecure_skip_verify: Option<bool>,
    /// Secret containing the client key file for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySecret")]
    pub key_secret: Option<ProbeTlsConfigKeySecret>,
    /// Used to verify the hostname for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
}

/// Certificate authority used when verifying server certificates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCa {
    /// ConfigMap containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ProbeTlsConfigCaConfigMap>,
    /// Secret containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ProbeTlsConfigCaSecret>,
}

/// ConfigMap containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCaConfigMap {
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

/// Secret containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCaSecret {
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

/// Client certificate to present when doing client-authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCert {
    /// ConfigMap containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ProbeTlsConfigCertConfigMap>,
    /// Secret containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ProbeTlsConfigCertSecret>,
}

/// ConfigMap containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCertConfigMap {
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

/// Secret containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigCertSecret {
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

/// Secret containing the client key file for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProbeTlsConfigKeySecret {
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

