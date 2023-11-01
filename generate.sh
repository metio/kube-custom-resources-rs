#!/usr/bin/env bash

### Generate code with kopium
for file in $(find ./crd-catalog -name '*.yaml' -type f); do
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  mkdir --parents "./kube-custom-resources-rs/src/${module}"

  if ! kopium --auto --filename "${file}" >"./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
    echo "error in ${file}"
  else
    echo "pub mod ${rust_crd};" >>"./kube-custom-resources-rs/src/${module}/mod.rs"
  fi
done


### Remove buggy resources
BUGGY_RESOURCES=(
  acid_zalan_do_v1/postgresqls
  apiextensions_crossplane_io_v1/compositionrevisions
  apiextensions_crossplane_io_v1/compositions
  apiextensions_crossplane_io_v1beta1/compositionrevisions
  appprotect_f5_com_v1beta1/appolicies
  apps_clusternet_io_v1alpha1/manifests
  apps_gitlab_com_v1beta2/runners
  aquasecurity_github_io_v1alpha1/clusterconfigauditreports
  aquasecurity_github_io_v1alpha1/configauditreports
  argoproj_io_v1alpha1/applicationsets
  autoscaling_k8s_elastic_co_v1alpha1/elasticsearchautoscalers
  camel_apache_org_v1/integrations
  ceph_rook_io_v1/cephclusters
  charts_helm_k8s_io_v1alpha1/snykmonitors
  charts_opdev_io_v1alpha1/synapses
  charts_operatorhub_io_v1alpha1/cockroachdbs
  cilium_io_v2/ciliumclusterwideenvoyconfigs
  cilium_io_v2/ciliumenvoyconfigs
  cilium_io_v2/ciliumidentities
  cilium_io_v2/ciliumlocalredirectpolicies
  core_strimzi_io_v1beta2/strimzipodsets
  crd_projectcalico_org_v1/felixconfigurations
  crd_projectcalico_org_v1/globalnetworkpolicies
  crd_projectcalico_org_v1/ippools
  crd_projectcalico_org_v1/networkpolicies
  dex_coreos_com_v1/authcodes
  dex_coreos_com_v1/authrequests
  dex_coreos_com_v1/connectors
  dex_coreos_com_v1/devicerequests
  dex_coreos_com_v1/devicetokens
  dex_coreos_com_v1/oauth2clients
  dex_coreos_com_v1/offlinesessionses
  dex_coreos_com_v1/passwords
  dex_coreos_com_v1/refreshtokens
  dex_coreos_com_v1/signingkeies
  fossul_io_v1/backupconfigs
  fossul_io_v1/backups
  fossul_io_v1/backupschedules
  fossul_io_v1/fossuls
  fossul_io_v1/restores
  gateway_solo_io_v1/routeoptions
  gateway_solo_io_v1/routetables
  gateway_solo_io_v1/virtualhostoptions
  gateway_solo_io_v1/virtualservices
  getambassador_io_v1/authservices
  getambassador_io_v1/consulresolvers
  getambassador_io_v1/devportals
  getambassador_io_v1/kubernetesendpointresolvers
  getambassador_io_v1/kubernetesserviceresolvers
  getambassador_io_v1/logservices
  getambassador_io_v1/mappings
  getambassador_io_v1/modules
  getambassador_io_v1/ratelimitservices
  getambassador_io_v1/tcpmappings
  getambassador_io_v1/tlscontexts
  getambassador_io_v1/tracingservices
  getambassador_io_v2/authservices
  getambassador_io_v2/consulresolvers
  getambassador_io_v2/devportals
  getambassador_io_v2/hosts
  getambassador_io_v2/kubernetesendpointresolvers
  getambassador_io_v2/kubernetesserviceresolvers
  getambassador_io_v2/logservices
  getambassador_io_v2/mappings
  getambassador_io_v2/modules
  getambassador_io_v2/ratelimitservices
  getambassador_io_v2/tcpmappings
  getambassador_io_v2/tlscontexts
  getambassador_io_v2/tracingservices
  grafana_integreatly_org_v1beta1/grafanas
  helm_sigstore_dev_v1alpha1/rekors
  hive_openshift_io_v1/selectorsyncsets
  hive_openshift_io_v1/syncsets
  infinispan_org_v1/infinispans
  install_istio_io_v1alpha1/istiooperators
  kiali_io_v1alpha1/kialis
  kuma_io_v1alpha1/circuitbreakers
  kuma_io_v1alpha1/dataplaneinsights
  kuma_io_v1alpha1/dataplanes
  kuma_io_v1alpha1/externalservices
  kuma_io_v1alpha1/faultinjections
  kuma_io_v1alpha1/healthchecks
  kuma_io_v1alpha1/meshes
  kuma_io_v1alpha1/meshgatewayroutes
  kuma_io_v1alpha1/meshgateways
  kuma_io_v1alpha1/meshinsights
  kuma_io_v1alpha1/proxytemplates
  kuma_io_v1alpha1/ratelimits
  kuma_io_v1alpha1/retries
  kuma_io_v1alpha1/serviceinsights
  kuma_io_v1alpha1/timeouts
  kuma_io_v1alpha1/trafficlogs
  kuma_io_v1alpha1/trafficpermissions
  kuma_io_v1alpha1/trafficroutes
  kuma_io_v1alpha1/traffictraces
  kuma_io_v1alpha1/virtualoutbounds
  kuma_io_v1alpha1/zoneegresses
  kuma_io_v1alpha1/zoneegressinsights
  kuma_io_v1alpha1/zoneingresses
  kuma_io_v1alpha1/zoneingressinsights
  kuma_io_v1alpha1/zoneinsights
  kuma_io_v1alpha1/zones
  lb_lbconfig_carlosedp_com_v1/externalloadbalancers
  litmuschaos_io_v1alpha1/chaosresults
  logging_banzaicloud_io_v1beta1/fluentbitagents
  logging_banzaicloud_io_v1beta1/loggings
  logging_banzaicloud_io_v1beta1/nodeagents
  logging_extensions_banzaicloud_io_v1alpha1/eventtailers
  longhorn_io_v1beta1/backingimagedatasources
  longhorn_io_v1beta1/backingimagemanagers
  longhorn_io_v1beta1/backingimages
  longhorn_io_v1beta1/backups
  longhorn_io_v1beta1/backuptargets
  longhorn_io_v1beta1/backupvolumes
  longhorn_io_v1beta1/engineimages
  longhorn_io_v1beta1/engines
  longhorn_io_v1beta1/instancemanagers
  longhorn_io_v1beta1/nodes
  longhorn_io_v1beta1/recurringjobs
  longhorn_io_v1beta1/replicas
  longhorn_io_v1beta1/settings
  longhorn_io_v1beta1/sharemanagers
  longhorn_io_v1beta1/volumes
  longhorn_io_v1beta2/settings
  metal3_io_v1alpha1/baremetalhosts
  microcks_github_io_v1alpha1/microcksinstalls
  monitoring_coreos_com_v1/podmonitors
  monitoring_coreos_com_v1/probes
  monitoring_coreos_com_v1/servicemonitors
  multicluster_x_k8s_io_v1alpha1/works
  networking_istio_io_v1alpha3/virtualservices
  networking_istio_io_v1beta1/virtualservices
  operator_knative_dev_v1beta1/knativeeventings
  operator_knative_dev_v1beta1/knativeservings
  operator_tekton_dev_v1alpha1/tektonchains
  operator_tekton_dev_v1alpha1/tektonconfigs
  operator_tekton_dev_v1alpha1/tektonhubs
  operator_tekton_dev_v1alpha1/tektoninstallersets
  operator_tekton_dev_v1alpha1/tektonpipelines
  operator_tekton_dev_v1alpha1/tektontriggers
  resources_teleport_dev_v2/teleportprovisiontokens
  resources_teleport_dev_v5/teleportroles
  resources_teleport_dev_v6/teleportroles
  security_istio_io_v1/requestauthentications
  security_istio_io_v1beta1/requestauthentications
  security_profiles_operator_x_k8s_io_v1alpha2/selinuxprofiles
  source_toolkit_fluxcd_io_v1/gitrepositories
  traefik_io_v1alpha1/middlewares
  work_karmada_io_v1alpha1/works
)
for resource in "${BUGGY_RESOURCES[@]}"; do
  echo "removing ${resource}"
  rm --force "./kube-custom-resources-rs/src/${resource}.rs"
done


### Generate mod.rs files
find ./kube-custom-resources-rs/src -name mod.rs -type f -delete
for file in $(find ./crd-catalog -name '*.yaml' -type f); do
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  if [ -f "./kube-custom-resources-rs/src/${module}/${rust_crd}.rs" ]; then
    echo "pub mod ${rust_crd};" >>"./kube-custom-resources-rs/src/${module}/mod.rs"
  fi
done


### Adjust Cargo.toml and src/lib.rs
sed -i '/\[features\]/,$d' ./kube-custom-resources-rs/Cargo.toml
echo '[features]' >>./kube-custom-resources-rs/Cargo.toml
rm --force ./kube-custom-resources-rs/src/lib.rs

for mld in ./kube-custom-resources-rs/src/*; do
  module=$(basename "${mld}")

  if [ -f "${mld}/mod.rs" ]; then
    echo "#[cfg(feature = \"${module}\")]" >> ./kube-custom-resources-rs/src/lib.rs
    echo "pub mod ${module};" >> ./kube-custom-resources-rs/src/lib.rs
    echo "${module} = []" >>./kube-custom-resources-rs/Cargo.toml
  fi
done


### Add 'all' feature
echo "all = [" >>./kube-custom-resources-rs/Cargo.toml
for mld in ./kube-custom-resources-rs/src/*; do
  module=$(basename "${mld}")

  if [ -f "${mld}/mod.rs" ]; then
    echo "  \"${module}\"," >>./kube-custom-resources-rs/Cargo.toml
  fi
done
echo "]" >>./kube-custom-resources-rs/Cargo.toml
