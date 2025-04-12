// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tigera/operator/operator.tigera.io/v1/imagesets.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ImageSetSpec defines the desired state of ImageSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "ImageSet", plural = "imagesets")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ImageSetSpec {
    /// Images is the list of images to use digests. All images that the operator will deploy
    /// must be specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageSetImages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImageSetImages {
    /// Digest is the image identifier that will be used for the Image.
    /// The field should not include a leading `@` and must be prefixed with `sha256:`.
    pub digest: String,
    /// Image is an image that the operator deploys and instead of using the built in tag
    /// the operator will use the Digest for the image identifier.
    /// The value should be the *original* image name without registry or tag or digest.
    /// For the image `docker.io/calico/node:v3.17.1` it should be represented as `calico/node`
    /// The "Installation" spec allows defining custom image registries, paths or prefixes.
    /// Even for custom images such as example.com/custompath/customprefix-calico-node:v3.17.1,
    /// this value should still be `calico/node`.
    pub image: String,
}

