// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/devportals.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DevPortalSpec defines the desired state of DevPortal
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "DevPortal", plural = "devportals")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DevPortalSpec {
    /// AmbassadorID declares which Ambassador instances should pay attention to this resource. If no value is provided, the default is: 
    ///  ambassador_id: - "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    /// Content specifies where the content shown in the DevPortal come from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<DevPortalContent>,
    /// Default must be true when this is the default DevPortal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Docs is a static docs definition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<DevPortalDocs>>,
    /// Describes how to display "services" in the DevPortal. Default namespace.name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub naming_scheme: Option<DevPortalNamingScheme>,
    /// Configures this DevPortal to use server definitions from the openAPI doc instead of rewriting them based on the url used for the connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_servers: Option<bool>,
    /// DevPortalSearchSpec allows configuration over search functionality for the DevPortal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<DevPortalSearch>,
    /// Selector is used for choosing what is shown in the DevPortal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<DevPortalSelector>,
}

/// Content specifies where the content shown in the DevPortal come from
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevPortalContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// DevPortalDocsSpec is a static documentation definition: instead of using a Selector for finding documentation for services, users can provide a static list of <service>:<URL> tuples. These services will be shown in the Dev Portal with the documentation obtained from this URL.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevPortalDocs {
    /// Service is the service being documented
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Timeout specifies the amount of time devportal will wait for the downstream service to report an openapi spec back
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    /// URL is the URL used for obtaining docs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// DevPortalSpec defines the desired state of DevPortal
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DevPortalNamingScheme {
    #[serde(rename = "namespace.name")]
    NamespaceName,
    #[serde(rename = "name.prefix")]
    NamePrefix,
}

/// DevPortalSearchSpec allows configuration over search functionality for the DevPortal
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevPortalSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Type of search. "title-only" does a fuzzy search over openapi and page titles "all-content" will fuzzy search over all openapi and page content. "title-only" is the default. warning:  using all-content may incur a larger memory footprint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DevPortalSearchType>,
}

/// DevPortalSearchSpec allows configuration over search functionality for the DevPortal
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DevPortalSearchType {
    #[serde(rename = "title-only")]
    TitleOnly,
    #[serde(rename = "all-content")]
    AllContent,
}

/// Selector is used for choosing what is shown in the DevPortal
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevPortalSelector {
    /// MatchLabels specifies the list of labels that must be present in Mappings for being present in this DevPortal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// MatchNamespaces is a list of namespaces that will be included in this DevPortal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchNamespaces")]
    pub match_namespaces: Option<Vec<String>>,
}

