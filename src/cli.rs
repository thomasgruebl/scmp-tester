use std::net::Ipv4Addr;

use clap::Parser;

///
/// A simple program to test the behaviour of SCION Control Message Protocol (SCMP) echo requests with custom parameters.
///
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Source ISD-AS to use. Structure: <ISD>-<AS>. Example: 18-ffaa:1:117c
    #[arg(short = 'a', long)]
    pub(crate) src_isd_as: String,

    /// Destination ISD-AS to use. Structure: <ISD>-<AS>. Example: 17-ffaa:1:117b
    #[arg(short = 'b', long)]
    pub(crate) dst_isd_as: String,

    /// IPv4 source address.
    #[arg(short='s', long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    pub(crate) ipv4_src_addr: Ipv4Addr,

    /// IPv4 destination address.
    #[arg(short='d', long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    pub(crate) ipv4_dst_addr: Ipv4Addr,

    /// UDP source port to use.
    #[arg(short='p', long, default_value_t = 32766, value_parser = clap::value_parser!(u16).range(1..))]
    pub(crate) udp_src_port: u16,

    /// UDP destination port to use.
    #[arg(short='q', long, default_value_t = 30001, value_parser = clap::value_parser!(u16).range(1..))]
    pub(crate) udp_dst_port: u16,

    /// The SCION dameon address.
    #[arg(short='x', long, default_value_t = String::from("https://localhost:30255"))]
    pub(crate) daemon_address: String,

    /// Optional custom payload added to the "Data (variable Len)" field as per IETF draft-dekater-scion-dataplane-03.
    #[arg(short = 'c', long)]
    pub(crate) payload: Option<String>,

    /// The path ID.
    #[arg(short = 'n', long, default_value_t = 0)]
    pub(crate) path_id: usize,

    /// Time (in milliseconds) between echo requests.
    #[arg(short = 't', long, default_value_t = 1000)]
    pub(crate) time: u64,
}
