// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/fluxcd/image-reflector-controller/image.toolkit.fluxcd.io/v1beta1/imagepolicies.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ImagePolicySpec defines the parameters for calculating the
/// ImagePolicy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "image.toolkit.fluxcd.io", version = "v1beta1", kind = "ImagePolicy", plural = "imagepolicies")]
#[kube(namespaced)]
#[kube(status = "ImagePolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ImagePolicySpec {
    /// FilterTags enables filtering for only a subset of tags based on a set of
    /// rules. If no rules are provided, all the tags from the repository will be
    /// ordered and compared.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterTags")]
    pub filter_tags: Option<ImagePolicyFilterTags>,
    /// ImageRepositoryRef points at the object specifying the image
    /// being scanned
    #[serde(rename = "imageRepositoryRef")]
    pub image_repository_ref: ImagePolicyImageRepositoryRef,
    /// Policy gives the particulars of the policy to be followed in
    /// selecting the most recent image
    pub policy: ImagePolicyPolicy,
}

/// FilterTags enables filtering for only a subset of tags based on a set of
/// rules. If no rules are provided, all the tags from the repository will be
/// ordered and compared.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyFilterTags {
    /// Extract allows a capture group to be extracted from the specified regular
    /// expression pattern, useful before tag evaluation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract: Option<String>,
    /// Pattern specifies a regular expression pattern used to filter for image
    /// tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// ImageRepositoryRef points at the object specifying the image
/// being scanned
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyImageRepositoryRef {
    /// Name of the referent.
    pub name: String,
    /// Namespace of the referent, when not specified it acts as LocalObjectReference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Policy gives the particulars of the policy to be followed in
/// selecting the most recent image
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyPolicy {
    /// Alphabetical set of rules to use for alphabetical ordering of the tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alphabetical: Option<ImagePolicyPolicyAlphabetical>,
    /// Numerical set of rules to use for numerical ordering of the tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numerical: Option<ImagePolicyPolicyNumerical>,
    /// SemVer gives a semantic version range to check against the tags
    /// available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<ImagePolicyPolicySemver>,
}

/// Alphabetical set of rules to use for alphabetical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyPolicyAlphabetical {
    /// Order specifies the sorting order of the tags. Given the letters of the
    /// alphabet as tags, ascending order would select Z, and descending order
    /// would select A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<ImagePolicyPolicyAlphabeticalOrder>,
}

/// Alphabetical set of rules to use for alphabetical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImagePolicyPolicyAlphabeticalOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// Numerical set of rules to use for numerical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyPolicyNumerical {
    /// Order specifies the sorting order of the tags. Given the integer values
    /// from 0 to 9 as tags, ascending order would select 9, and descending order
    /// would select 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<ImagePolicyPolicyNumericalOrder>,
}

/// Numerical set of rules to use for numerical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImagePolicyPolicyNumericalOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// SemVer gives a semantic version range to check against the tags
/// available.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyPolicySemver {
    /// Range gives a semver range for the image tag; the highest
    /// version within the range that's a tag yields the latest image.
    pub range: String,
}

/// ImagePolicyStatus defines the observed state of ImagePolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImagePolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LatestImage gives the first in the list of images scanned by
    /// the image repository, when filtered and ordered according to
    /// the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestImage")]
    pub latest_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

