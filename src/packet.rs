use bytes::Bytes;

use tonic::transport::Channel;

use etherparse::{PacketBuilder, PacketBuilderStep, UdpHeader};

use scion_grpc::daemon::DaemonServiceClient;

use scion_proto::address::{Asn, HostAddr, Isd, IsdAsn, ScionAddr};
use scion_proto::packet::ByEndpoint;
use scion_proto::path;
use scion_proto::scmp::{ScmpEchoRequest, ScmpMessage};
use scion_proto::wire_encoding::WireEncodeVec;

use crate::cli::Cli;
use crate::daemon::{connect_to_daemon, fetch_nth_path};

fn parse_isd_as(isd_asn: &String) -> IsdAsn {
    let isd_asn_vec: Vec<&str> = isd_asn.split('-').collect();

    let _isd = isd_asn_vec[0].parse::<u16>().unwrap();
    let _asn: Vec<&str> = isd_asn_vec[1].split(':').collect();

    let _asn_part1: u64 = u64::from_str_radix(_asn[0], 16).unwrap();
    let _asn_part2: u64 = u64::from_str_radix(_asn[1], 16).unwrap();
    let _asn_part3: u64 = u64::from_str_radix(_asn[2], 16).unwrap();
    let _asn: u64 = (_asn_part1 << 32) | (_asn_part2 << 16) | _asn_part3;

    let isd: Isd = Isd(_isd);
    let asn: Asn = Asn::new(_asn);

    let isd_asn: IsdAsn = IsdAsn::new(isd, asn);

    isd_asn
}

pub(crate) fn build_scmp_echo_request(cli: &Cli) -> ScmpMessage {
    let mut bytes: Bytes = Bytes::from("");
    if let Some(ref payload) = cli.payload {
        bytes = Bytes::from(payload.clone());
    }

    let _echo_request: ScmpEchoRequest = scion_proto::scmp::ScmpEchoRequest::new(1, 1, bytes);
    let echo_request: ScmpMessage = scion_proto::scmp::ScmpMessage::EchoRequest(_echo_request);

    echo_request
}

pub(crate) async fn build_scmp_message(cli: &Cli, echo_request: &ScmpMessage) -> Vec<u8> {
    let isd_asn_src: IsdAsn = parse_isd_as(&cli.src_isd_as);
    let isd_asn_dst: IsdAsn = parse_isd_as(&cli.dst_isd_as);

    let host_addr: HostAddr = HostAddr::V4(cli.ipv4_src_addr);
    let peer_addr: HostAddr = HostAddr::V4(cli.ipv4_dst_addr);

    let isd_asn_endpoints: ByEndpoint<IsdAsn> = ByEndpoint {
        source: (isd_asn_src),
        destination: (isd_asn_dst),
    };

    let scion_src_addr: ScionAddr = ScionAddr::new(isd_asn_src, host_addr);
    let scion_dst_addr: ScionAddr = ScionAddr::new(isd_asn_dst, peer_addr);

    let endhosts: ByEndpoint<ScionAddr> = ByEndpoint {
        source: (scion_src_addr),
        destination: (scion_dst_addr),
    };

    let path_request: scion_grpc::daemon::v1::PathsRequest = scion_grpc::daemon::v1::PathsRequest {
        source_isd_as: isd_asn_src.into(),
        destination_isd_as: isd_asn_dst.into(),
        refresh: true,
        hidden: false,
    };

    let mut daemon_client: DaemonServiceClient<Channel> = connect_to_daemon().await;
    let scion_proto_path: path::Path = fetch_nth_path(
        cli.path_id,
        &mut daemon_client,
        &path_request,
        &isd_asn_endpoints,
    )
    .await;

    let scmp_packet: scion_proto::packet::ScionPacketScmp =
        scion_proto::packet::ScionPacketScmp::new(
            &endhosts,
            &scion_proto_path,
            echo_request.clone(),
        )
        .unwrap();
    let scmp_packet_bytes: Vec<u8> = scmp_packet
        .encode_to_bytes_vec()
        .into_iter()
        .flat_map(|b: Bytes| b.as_ref().to_vec())
        .collect::<Vec<u8>>();

    let scmp_packet_bytes: Vec<u8> = scmp_packet_bytes.as_slice().to_vec();

    scmp_packet_bytes
}

pub(crate) fn build_udp_datagram(scmp_packet_bytes: &Vec<u8>, cli: &Cli) -> Vec<u8> {
    // create L3 and L4 headers using etherparse::PacketBuilder
    let builder: PacketBuilderStep<UdpHeader> =
        PacketBuilder::ipv4(cli.ipv4_src_addr.octets(), cli.ipv4_dst_addr.octets(), 20)
            .udp(cli.udp_src_port, cli.udp_dst_port);

    // payload of the udp packet
    let udp_payload: &[u8] = scmp_packet_bytes;
    let mut udp_datagram: Vec<u8> = Vec::<u8>::with_capacity(builder.size(udp_payload.len()));

    // serialize
    builder.write(&mut udp_datagram, &udp_payload).unwrap();

    udp_datagram
}
