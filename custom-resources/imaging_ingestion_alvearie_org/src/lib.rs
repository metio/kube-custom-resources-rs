/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `imaging-ingestion.alvearie.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## imaging-ingestion.alvearie.org/v1alpha1
- `DicomEventBridge`
- `DicomEventDrivenIngestion`
- `DicomInstanceBinding`
- `DicomStudyBinding`
- `DicomwebIngestionService`
- `DimseIngestionService`
- `DimseProxy`
*/
pub mod v1alpha1;
