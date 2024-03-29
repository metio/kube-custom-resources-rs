// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/config.openshift.io/v1/imagecontentpolicies.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec holds user settable values for configuration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "config.openshift.io", version = "v1", kind = "ImageContentPolicy", plural = "imagecontentpolicies")]
#[kube(schema = "disabled")]
pub struct ImageContentPolicySpec {
    /// repositoryDigestMirrors allows images referenced by image digests in pods to be pulled from alternative mirrored repository locations. The image pull specification provided to the pod will be compared to the source locations described in RepositoryDigestMirrors and the image may be pulled down from any of the mirrors in the list instead of the specified repository allowing administrators to choose a potentially faster mirror. To pull image from mirrors by tags, should set the "allowMirrorByTags". 
    ///  Each “source” repository is treated independently; configurations for different “source” repositories don’t interact. 
    ///  If the "mirrors" is not specified, the image will continue to be pulled from the specified repository in the pull spec. 
    ///  When multiple policies are defined for the same “source” repository, the sets of defined mirrors will be merged together, preserving the relative order of the mirrors, if possible. For example, if policy A has mirrors `a, b, c` and policy B has mirrors `c, d, e`, the mirrors will be used in the order `a, b, c, d, e`.  If the orders of mirror entries conflict (e.g. `a, b` vs. `b, a`) the configuration is not rejected but the resulting order is unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryDigestMirrors")]
    pub repository_digest_mirrors: Option<Vec<ImageContentPolicyRepositoryDigestMirrors>>,
}

/// RepositoryDigestMirrors holds cluster-wide information about how to handle mirrors in the registries config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImageContentPolicyRepositoryDigestMirrors {
    /// allowMirrorByTags if true, the mirrors can be used to pull the images that are referenced by their tags. Default is false, the mirrors only work when pulling the images that are referenced by their digests. Pulling images by tag can potentially yield different images, depending on which endpoint we pull from. Forcing digest-pulls for mirrors avoids that issue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMirrorByTags")]
    pub allow_mirror_by_tags: Option<bool>,
    /// mirrors is zero or more repositories that may also contain the same images. If the "mirrors" is not specified, the image will continue to be pulled from the specified repository in the pull spec. No mirror will be configured. The order of mirrors in this list is treated as the user's desired priority, while source is by default considered lower priority than all mirrors. Other cluster configuration, including (but not limited to) other repositoryDigestMirrors objects, may impact the exact order mirrors are contacted in, or some mirrors may be contacted in parallel, so this should be considered a preference rather than a guarantee of ordering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
    /// source is the repository that users refer to, e.g. in image pull specifications.
    pub source: String,
}

