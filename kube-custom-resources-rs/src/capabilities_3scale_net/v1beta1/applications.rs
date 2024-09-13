// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/applications.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ApplicationSpec defines the desired state of Application
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "Application", plural = "applications")]
#[kube(namespaced)]
#[kube(status = "ApplicationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ApplicationSpec {
    /// AccountCRName name of account custom resource under which the application will be created
    #[serde(rename = "accountCR")]
    pub account_cr: ApplicationAccountCr,
    /// ApplicationPlanName name of application plan that the application will use
    #[serde(rename = "applicationPlanName")]
    pub application_plan_name: String,
    /// Description human-readable text of the application
    pub description: String,
    /// Name identifies the application uniquely within the account
    pub name: String,
    /// ProductCRName of product custom resource from which the application plan will be used
    #[serde(rename = "productCR")]
    pub product_cr: ApplicationProductCr,
    /// Suspend application if true suspends application, if false resumes application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

/// AccountCRName name of account custom resource under which the application will be created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationAccountCr {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ProductCRName of product custom resource from which the application plan will be used
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationProductCr {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ApplicationStatus defines the observed state of Application
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationID")]
    pub application_id: Option<i64>,
    /// Current state of the 3scale application. Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration reflects the generation of the most recently observed Application Spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// 3scale control plane host
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountHost")]
    pub provider_account_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

