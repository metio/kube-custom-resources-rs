#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD


### Generate code with kopium
for file in $(find ./crd-catalog -name '*.yaml' -type f | LC_ALL=C sort --general-numeric-sort); do
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  mkdir --parents "./kube-custom-resources-rs/src/${module}"

  if ! kopium --docs --filename "${file}" >"./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
    echo "error in ${file}"
  fi
done


### Remove buggy resources
BUGGY_RESOURCES=(
  acid_zalan_do_v1/operatorconfigurations
  acid_zalan_do_v1/postgresqls
  acid_zalan_do_v1/postgresteams
  addons_cluster_x_k8s_io_v1alpha4/clusterresourcesetbindings
  addons_cluster_x_k8s_io_v1beta1/clusterresourcesetbindings
  apiextensions_crossplane_io_v1/compositionrevisions
  apiextensions_crossplane_io_v1/compositions
  apiextensions_crossplane_io_v1beta1/compositionrevisions
  apigatewayv2_services_k8s_aws_v1alpha1/integrations
  appprotect_f5_com_v1beta1/appolicies
  apps_clusternet_io_v1alpha1/manifests
  apps_kubeedge_io_v1alpha1/edgeapplications
  apps_gitlab_com_v1beta2/runners
  apps_redhat_com_v1alpha1/clusterimpairments
  aquasecurity_github_io_v1alpha1/clusterconfigauditreports
  aquasecurity_github_io_v1alpha1/configauditreports
  argoproj_io_v1alpha1/applicationsets
  autoscaling_k8s_elastic_co_v1alpha1/elasticsearchautoscalers
  camel_apache_org_v1/integrations
  camel_apache_org_v1/integrationkits
  camel_apache_org_v1/integrationplatforms
  camel_apache_org_v1alpha1/kameletbindings
  ceph_rook_io_v1/cephclusters
  charts_flagsmith_com_v1alpha1/flagsmiths
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
  enterprise_gloo_solo_io_v1/authconfigs
  extensions_istio_io_v1alpha1/wasmplugins
  flagger_app_v1beta1/alertproviders
  flagger_app_v1beta1/metrictemplates
  fossul_io_v1/backupconfigs
  fossul_io_v1/backups
  fossul_io_v1/backupschedules
  fossul_io_v1/fossuls
  fossul_io_v1/restores
  gateway_solo_io_v1/gateways
  gateway_solo_io_v1/httpgateways
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
  getambassador_io_v3alpha1/mappings
  gloo_solo_io_v1/proxies
  gloo_solo_io_v1/settings
  gloo_solo_io_v1/upstreams
  gloo_solo_io_v1/upstreamgroups
  grafana_integreatly_org_v1beta1/grafanas
  graphql_gloo_solo_io_v1beta1/graphqlapis
  helm_sigstore_dev_v1alpha1/rekors
  hive_openshift_io_v1/selectorsyncsets
  hive_openshift_io_v1/syncsets
  infinispan_org_v1/infinispans
  install_istio_io_v1alpha1/istiooperators
  jobsmanager_raczylo_com_v1beta1/managedjobs
  kafka_strimzi_io_v1beta2/kafkamirrormaker2s
  kiali_io_v1alpha1/kialis
  kubean_io_v1alpha1/localartifactsets
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
  kyverno_io_v1/clusterpolicies
  kyverno_io_v1/policies
  kyverno_io_v2beta1/clusterpolicies
  kyverno_io_v2beta1/policies
  lambda_services_k8s_aws_v1alpha1/aliases
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
  monitoring_coreos_com_v1/prometheuses
  monitoring_coreos_com_v1/servicemonitors
  monitoring_coreos_com_v1alpha1/prometheusagents
  monitoring_coreos_com_v1alpha1/scrapeconfigs
  multicluster_x_k8s_io_v1alpha1/works
  networking_istio_io_v1alpha3/destinationrules
  networking_istio_io_v1alpha3/envoyfilters
  networking_istio_io_v1alpha3/gateways
  networking_istio_io_v1alpha3/serviceentries
  networking_istio_io_v1alpha3/sidecars
  networking_istio_io_v1alpha3/virtualservices
  networking_istio_io_v1alpha3/workloadentries
  networking_istio_io_v1alpha3/workloadgroups
  networking_istio_io_v1beta1/destinationrules
  networking_istio_io_v1beta1/gateways
  networking_istio_io_v1beta1/proxyconfigs
  networking_istio_io_v1beta1/serviceentries
  networking_istio_io_v1beta1/sidecars
  networking_istio_io_v1beta1/virtualservices
  networking_istio_io_v1beta1/workloadentries
  networking_istio_io_v1beta1/workloadgroups
  objectbucket_io_v1alpha1/objectbuckets
  objectbucket_io_v1alpha1/objectbucketclaims
  operator_knative_dev_v1beta1/knativeeventings
  operator_knative_dev_v1beta1/knativeservings
  operator_tekton_dev_v1alpha1/tektonchains
  operator_tekton_dev_v1alpha1/tektonconfigs
  operator_tekton_dev_v1alpha1/tektonhubs
  operator_tekton_dev_v1alpha1/tektoninstallersets
  operator_tekton_dev_v1alpha1/tektonpipelines
  operator_tekton_dev_v1alpha1/tektontriggers
  operator_tigera_io_v1/imagesets
  resources_teleport_dev_v2/teleportprovisiontokens
  resources_teleport_dev_v5/teleportroles
  resources_teleport_dev_v6/teleportroles
  ripsaw_cloudbulldozer_io_v1alpha1/benchmarks
  security_istio_io_v1/authorizationpolicies
  security_istio_io_v1/requestauthentications
  security_istio_io_v1beta1/authorizationpolicies
  security_istio_io_v1beta1/peerauthentications
  security_istio_io_v1beta1/requestauthentications
  security_profiles_operator_x_k8s_io_v1alpha2/selinuxprofiles
  sematext_com_v1/sematextagents
  source_toolkit_fluxcd_io_v1/gitrepositories
  telemetry_istio_io_v1alpha1/telemetries
  traefik_io_v1alpha1/middlewares
  work_karmada_io_v1alpha1/works
)
for resource in "${BUGGY_RESOURCES[@]}"; do
  echo "removing ${resource}"
  rm --force "./kube-custom-resources-rs/src/${resource}.rs"
done


### Generate mod.rs files
find ./kube-custom-resources-rs/src -name mod.rs -type f -delete
for file in $(find ./crd-catalog -name '*.yaml' -type f | LC_ALL=C sort --general-numeric-sort); do
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

for mld in $(find ./kube-custom-resources-rs/src -type d | LC_ALL=C sort --general-numeric-sort); do
  module=$(basename "${mld}")

  if [ -f "${mld}/mod.rs" ]; then
    echo "#[cfg(feature = \"${module}\")]" >> ./kube-custom-resources-rs/src/lib.rs
    echo "pub mod ${module};" >> ./kube-custom-resources-rs/src/lib.rs
    echo "${module} = []" >>./kube-custom-resources-rs/Cargo.toml
  fi
done

