// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/operator.openshift.io/v1/consoles.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// ConsoleSpec is the specification of the desired behavior of the Console.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.openshift.io", version = "v1", kind = "Console", plural = "consoles")]
#[kube(status = "ConsoleStatus")]
#[kube(schema = "disabled")]
pub struct ConsoleSpec {
    /// customization is used to optionally provide a small set of customization options to the web console.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<ConsoleCustomization>,
    /// logLevel is an intent based logging for an overall component.  It does not give fine grained control, but it is a simple way to manage coarse grained logging choices that operators have to interpret for their operands. 
    ///  Valid values are: "Normal", "Debug", "Trace", "TraceAll". Defaults to "Normal".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<ConsoleLogLevel>,
    /// managementState indicates whether and how the operator should manage the component
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementState")]
    pub management_state: Option<String>,
    /// observedConfig holds a sparse config that controller has observed from the cluster state.  It exists in spec because it is an input to the level for the operator
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedConfig")]
    pub observed_config: Option<BTreeMap<String, serde_json::Value>>,
    /// operatorLogLevel is an intent based logging for the operator itself.  It does not give fine grained control, but it is a simple way to manage coarse grained logging choices that operators have to interpret for themselves. 
    ///  Valid values are: "Normal", "Debug", "Trace", "TraceAll". Defaults to "Normal".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operatorLogLevel")]
    pub operator_log_level: Option<ConsoleOperatorLogLevel>,
    /// plugins defines a list of enabled console plugin names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<String>>,
    /// providers contains configuration for using specific service providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<ConsoleProviders>,
    /// route contains hostname and secret reference that contains the serving certificate. If a custom route is specified, a new route will be created with the provided hostname, under which console will be available. In case of custom hostname uses the default routing suffix of the cluster, the Secret specification for a serving certificate will not be needed. In case of custom hostname points to an arbitrary domain, manual DNS configurations steps are necessary. The default console route will be maintained to reserve the default hostname for console if the custom route is removed. If not specified, default route will be used. DEPRECATED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<ConsoleRoute>,
    /// unsupportedConfigOverrides overrides the final configuration that was computed by the operator. Red Hat does not support the use of this field. Misuse of this field could lead to unexpected behavior or conflict with other configuration options. Seek guidance from the Red Hat support before using this field. Use of this property blocks cluster upgrades, it must be removed before upgrading your cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unsupportedConfigOverrides")]
    pub unsupported_config_overrides: Option<BTreeMap<String, serde_json::Value>>,
}

/// customization is used to optionally provide a small set of customization options to the web console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomization {
    /// addPage allows customizing actions on the Add page in developer perspective.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addPage")]
    pub add_page: Option<ConsoleCustomizationAddPage>,
    /// brand is the default branding of the web console which can be overridden by providing the brand field.  There is a limited set of specific brand options. This field controls elements of the console such as the logo. Invalid value will prevent a console rollout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<ConsoleCustomizationBrand>,
    /// customLogoFile replaces the default OpenShift logo in the masthead and about dialog. It is a reference to a ConfigMap in the openshift-config namespace. This can be created with a command like 'oc create configmap custom-logo --from-file=/path/to/file -n openshift-config'. Image size must be less than 1 MB due to constraints on the ConfigMap size. The ConfigMap key should include a file extension so that the console serves the file with the correct MIME type. Recommended logo specifications: Dimensions: Max height of 68px and max width of 200px SVG format preferred
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customLogoFile")]
    pub custom_logo_file: Option<ConsoleCustomizationCustomLogoFile>,
    /// customProductName is the name that will be displayed in page titles, logo alt text, and the about dialog instead of the normal OpenShift product name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customProductName")]
    pub custom_product_name: Option<String>,
    /// developerCatalog allows to configure the shown developer catalog categories (filters) and types (sub-catalogs).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "developerCatalog")]
    pub developer_catalog: Option<ConsoleCustomizationDeveloperCatalog>,
    /// documentationBaseURL links to external documentation are shown in various sections of the web console.  Providing documentationBaseURL will override the default documentation URL. Invalid value will prevent a console rollout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationBaseURL")]
    pub documentation_base_url: Option<String>,
    /// perspectives allows enabling/disabling of perspective(s) that user can see in the Perspective switcher dropdown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perspectives: Option<Vec<ConsoleCustomizationPerspectives>>,
    /// projectAccess allows customizing the available list of ClusterRoles in the Developer perspective Project access page which can be used by a project admin to specify roles to other users and restrict access within the project. If set, the list will replace the default ClusterRole options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectAccess")]
    pub project_access: Option<ConsoleCustomizationProjectAccess>,
    /// quickStarts allows customization of available ConsoleQuickStart resources in console.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "quickStarts")]
    pub quick_starts: Option<ConsoleCustomizationQuickStarts>,
}

/// addPage allows customizing actions on the Add page in developer perspective.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationAddPage {
    /// disabledActions is a list of actions that are not shown to users. Each action in the list is represented by its ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disabledActions")]
    pub disabled_actions: Option<Vec<String>>,
}

/// customization is used to optionally provide a small set of customization options to the web console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsoleCustomizationBrand {
    #[serde(rename = "openshift")]
    Openshift,
    #[serde(rename = "okd")]
    Okd,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "ocp")]
    Ocp,
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "azure")]
    Azure,
    OpenShift,
    #[serde(rename = "OKD")]
    OkdX,
    #[serde(rename = "Online")]
    OnlineX,
    #[serde(rename = "OCP")]
    OcpX,
    #[serde(rename = "Dedicated")]
    DedicatedX,
    #[serde(rename = "Azure")]
    AzureX,
    #[serde(rename = "ROSA")]
    Rosa,
}

/// customLogoFile replaces the default OpenShift logo in the masthead and about dialog. It is a reference to a ConfigMap in the openshift-config namespace. This can be created with a command like 'oc create configmap custom-logo --from-file=/path/to/file -n openshift-config'. Image size must be less than 1 MB due to constraints on the ConfigMap size. The ConfigMap key should include a file extension so that the console serves the file with the correct MIME type. Recommended logo specifications: Dimensions: Max height of 68px and max width of 200px SVG format preferred
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationCustomLogoFile {
    /// Key allows pointing to a specific key/value inside of the configmap.  This is useful for logical file references.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// developerCatalog allows to configure the shown developer catalog categories (filters) and types (sub-catalogs).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationDeveloperCatalog {
    /// categories which are shown in the developer catalog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<ConsoleCustomizationDeveloperCatalogCategories>>,
    /// types allows enabling or disabling of sub-catalog types that user can see in the Developer catalog. When omitted, all the sub-catalog types will be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<ConsoleCustomizationDeveloperCatalogTypes>,
}

/// DeveloperConsoleCatalogCategory for the developer console catalog.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationDeveloperCatalogCategories {
    /// ID is an identifier used in the URL to enable deep linking in console. ID is required and must have 1-32 URL safe (A-Z, a-z, 0-9, - and _) characters.
    pub id: String,
    /// label defines a category display label. It is required and must have 1-64 characters.
    pub label: String,
    /// subcategories defines a list of child categories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategories: Option<Vec<ConsoleCustomizationDeveloperCatalogCategoriesSubcategories>>,
    /// tags is a list of strings that will match the category. A selected category show all items which has at least one overlapping tag between category and item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// DeveloperConsoleCatalogCategoryMeta are the key identifiers of a developer catalog category.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationDeveloperCatalogCategoriesSubcategories {
    /// ID is an identifier used in the URL to enable deep linking in console. ID is required and must have 1-32 URL safe (A-Z, a-z, 0-9, - and _) characters.
    pub id: String,
    /// label defines a category display label. It is required and must have 1-64 characters.
    pub label: String,
    /// tags is a list of strings that will match the category. A selected category show all items which has at least one overlapping tag between category and item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// types allows enabling or disabling of sub-catalog types that user can see in the Developer catalog. When omitted, all the sub-catalog types will be shown.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationDeveloperCatalogTypes {
    /// disabled is a list of developer catalog types (sub-catalogs IDs) that are not shown to users. Types (sub-catalogs) are added via console plugins, the available types (sub-catalog IDs) are available in the console on the cluster configuration page, or when editing the YAML in the console. Example: "Devfile", "HelmChart", "BuilderImage" If the list is empty or all the available sub-catalog types are added, then the complete developer catalog should be hidden.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
    /// enabled is a list of developer catalog types (sub-catalogs IDs) that will be shown to users. Types (sub-catalogs) are added via console plugins, the available types (sub-catalog IDs) are available in the console on the cluster configuration page, or when editing the YAML in the console. Example: "Devfile", "HelmChart", "BuilderImage" If the list is non-empty, a new type will not be shown to the user until it is added to list. If the list is empty the complete developer catalog will be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
    /// state defines if a list of catalog types should be enabled or disabled.
    pub state: ConsoleCustomizationDeveloperCatalogTypesState,
}

/// types allows enabling or disabling of sub-catalog types that user can see in the Developer catalog. When omitted, all the sub-catalog types will be shown.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsoleCustomizationDeveloperCatalogTypesState {
    Enabled,
    Disabled,
}

/// Perspective defines a perspective that cluster admins want to show/hide in the perspective switcher dropdown
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectives {
    /// id defines the id of the perspective. Example: "dev", "admin". The available perspective ids can be found in the code snippet section next to the yaml editor. Incorrect or unknown ids will be ignored.
    pub id: String,
    /// pinnedResources defines the list of default pinned resources that users will see on the perspective navigation if they have not customized these pinned resources themselves. The list of available Kubernetes resources could be read via `kubectl api-resources`. The console will also provide a configuration UI and a YAML snippet that will list the available resources that can be pinned to the navigation. Incorrect or unknown resources will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pinnedResources")]
    pub pinned_resources: Option<Vec<ConsoleCustomizationPerspectivesPinnedResources>>,
    /// visibility defines the state of perspective along with access review checks if needed for that perspective.
    pub visibility: ConsoleCustomizationPerspectivesVisibility,
}

/// PinnedResourceReference includes the group, version and type of resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectivesPinnedResources {
    /// group is the API Group of the Resource. Enter empty string for the core group. This value should consist of only lowercase alphanumeric characters, hyphens and periods. Example: "", "apps", "build.openshift.io", etc.
    pub group: String,
    /// resource is the type that is being referenced. It is normally the plural form of the resource kind in lowercase. This value should consist of only lowercase alphanumeric characters and hyphens. Example: "deployments", "deploymentconfigs", "pods", etc.
    pub resource: String,
    /// version is the API Version of the Resource. This value should consist of only lowercase alphanumeric characters. Example: "v1", "v1beta1", etc.
    pub version: String,
}

/// visibility defines the state of perspective along with access review checks if needed for that perspective.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectivesVisibility {
    /// accessReview defines required and missing access review checks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessReview")]
    pub access_review: Option<ConsoleCustomizationPerspectivesVisibilityAccessReview>,
    /// state defines the perspective is enabled or disabled or access review check is required.
    pub state: ConsoleCustomizationPerspectivesVisibilityState,
}

/// accessReview defines required and missing access review checks.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectivesVisibilityAccessReview {
    /// missing defines a list of permission checks. The perspective will only be shown when at least one check fails. When omitted, the access review is skipped and the perspective will not be shown unless it is required to do so based on the configuration of the required access review list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<Vec<ConsoleCustomizationPerspectivesVisibilityAccessReviewMissing>>,
    /// required defines a list of permission checks. The perspective will only be shown when all checks are successful. When omitted, the access review is skipped and the perspective will not be shown unless it is required to do so based on the configuration of the missing access review list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<ConsoleCustomizationPerspectivesVisibilityAccessReviewRequired>>,
}

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectivesVisibilityAccessReviewMissing {
    /// Group is the API Group of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Resource is one of the existing resource types.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Subresource is one of the existing resource types.  "" means none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    /// Version is the API Version of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationPerspectivesVisibilityAccessReviewRequired {
    /// Group is the API Group of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Resource is one of the existing resource types.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Subresource is one of the existing resource types.  "" means none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    /// Version is the API Version of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// visibility defines the state of perspective along with access review checks if needed for that perspective.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsoleCustomizationPerspectivesVisibilityState {
    Enabled,
    Disabled,
    AccessReview,
}

/// projectAccess allows customizing the available list of ClusterRoles in the Developer perspective Project access page which can be used by a project admin to specify roles to other users and restrict access within the project. If set, the list will replace the default ClusterRole options.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationProjectAccess {
    /// availableClusterRoles is the list of ClusterRole names that are assignable to users through the project access tab.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableClusterRoles")]
    pub available_cluster_roles: Option<Vec<String>>,
}

/// quickStarts allows customization of available ConsoleQuickStart resources in console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleCustomizationQuickStarts {
    /// disabled is a list of ConsoleQuickStart resource names that are not shown to users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
}

/// ConsoleSpec is the specification of the desired behavior of the Console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsoleLogLevel {
    #[serde(rename = "")]
    KopiumEmpty,
    Normal,
    Debug,
    Trace,
    TraceAll,
}

/// ConsoleSpec is the specification of the desired behavior of the Console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsoleOperatorLogLevel {
    #[serde(rename = "")]
    KopiumEmpty,
    Normal,
    Debug,
    Trace,
    TraceAll,
}

/// providers contains configuration for using specific service providers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleProviders {
    /// statuspage contains ID for statuspage.io page that provides status info about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuspage: Option<ConsoleProvidersStatuspage>,
}

/// statuspage contains ID for statuspage.io page that provides status info about.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleProvidersStatuspage {
    /// pageID is the unique ID assigned by Statuspage for your page. This must be a public page.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pageID")]
    pub page_id: Option<String>,
}

/// route contains hostname and secret reference that contains the serving certificate. If a custom route is specified, a new route will be created with the provided hostname, under which console will be available. In case of custom hostname uses the default routing suffix of the cluster, the Secret specification for a serving certificate will not be needed. In case of custom hostname points to an arbitrary domain, manual DNS configurations steps are necessary. The default console route will be maintained to reserve the default hostname for console if the custom route is removed. If not specified, default route will be used. DEPRECATED
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleRoute {
    /// hostname is the desired custom domain under which console will be available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// secret points to secret in the openshift-config namespace that contains custom certificate and key and needs to be created manually by the cluster admin. Referenced Secret is required to contain following key value pairs: - "tls.crt" - to specifies custom certificate - "tls.key" - to specifies private key of the custom certificate If the custom hostname uses the default routing suffix of the cluster, the Secret specification for a serving certificate will not be needed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ConsoleRouteSecret>,
}

/// secret points to secret in the openshift-config namespace that contains custom certificate and key and needs to be created manually by the cluster admin. Referenced Secret is required to contain following key value pairs: - "tls.crt" - to specifies custom certificate - "tls.key" - to specifies private key of the custom certificate If the custom hostname uses the default routing suffix of the cluster, the Secret specification for a serving certificate will not be needed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleRouteSecret {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// ConsoleStatus defines the observed status of the Console.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleStatus {
    /// conditions is a list of conditions and their status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// generations are used to determine when an item needs to be reconciled or has changed in a way that needs a reaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<ConsoleStatusGenerations>>,
    /// observedGeneration is the last generation change you've dealt with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas indicates how many replicas are ready and at the desired state
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// version is the level this availability applies to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// GenerationStatus keeps track of the generation for a given resource so that decisions about forced updates can be made.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConsoleStatusGenerations {
    /// group is the group of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// hash is an optional field set for resources without generation that are content sensitive like secrets and configmaps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// lastGeneration is the last generation of the workload controller involved
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastGeneration")]
    pub last_generation: Option<i64>,
    /// name is the name of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the thing you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

