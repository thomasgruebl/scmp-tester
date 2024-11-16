use tonic::transport::Channel;

use scion::daemon::get_daemon_address;
use scion_grpc::daemon::DaemonServiceClient;

use scion_proto::address::IsdAsn;
use scion_proto::packet::ByEndpoint;

pub(crate) async fn connect_to_daemon() -> DaemonServiceClient<Channel> {
    let daemon_address_string = get_daemon_address();
    let daemon_address: &'static str = Box::leak(daemon_address_string.into_boxed_str());
    let daemon = tonic::transport::channel::Endpoint::from_static(daemon_address);
    let daemon_client =
        scion_grpc::daemon::v1::daemon_service_client::DaemonServiceClient::connect(daemon)
            .await
            .unwrap();
    daemon_client
}

pub(crate) async fn fetch_nth_path(
    n: usize,
    daemon_client: &mut DaemonServiceClient<Channel>,
    path_request: &scion_grpc::daemon::v1::PathsRequest,
    isd_asn_endpoints: &ByEndpoint<IsdAsn>,
) -> scion_proto::path::Path {
    // fetch available paths between the two endpoints
    let available_paths: tonic::Response<scion_grpc::daemon::v1::PathsResponse> =
        daemon_client.paths(*path_request).await.unwrap();

    // select the nth path
    let selected_path: scion_grpc::daemon::v1::Path =
        available_paths.into_inner().paths.get(n).unwrap().clone();

    // convert to scion_prot::path::Path
    let scion_proto_path =
        scion_proto::path::Path::try_from_grpc(selected_path, *isd_asn_endpoints).unwrap();

    println!("Selected path: {:?}", scion_proto_path);

    scion_proto_path
}
