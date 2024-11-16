use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use regex::Regex;

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::transport::{transport_channel, TransportChannelType};

use scion_proto::scmp::ScmpMessage;

mod cli;
mod daemon;
mod packet;

use cli::Cli;
use packet::{build_scmp_echo_request, build_scmp_message, build_udp_datagram};

fn send_datagram(udp_datagram: &Vec<u8>, cli: &Cli) {
    let dur: Duration = Duration::from_millis(cli.time);

    loop {
        // wrap into Ipv4 Packet
        let packet: Ipv4Packet = Ipv4Packet::new(udp_datagram).unwrap();

        // create an L3 transport Channel for UDP datagrams
        let (mut tx, _rx) = transport_channel(
            1024,
            TransportChannelType::Layer3(IpNextHeaderProtocols::Udp),
        )
        .expect("Failed to create transport channel");

        let dst = packet.get_destination();

        // Send the packet
        tx.send_to(packet, std::net::IpAddr::V4(dst))
            .expect("Failed to send packet");

        println!("SCMP Echo Request sent.");
        sleep(dur);
    }
}

#[derive(Debug)]
struct ValidationError(String);

#[tokio::main]
async fn main() -> Result<(), ValidationError> {
    let cli = Cli::parse();

    println!("Source ISD-AS: {:?}", cli.src_isd_as);
    println!("Destination ISD-AS: {:?}", cli.dst_isd_as);
    println!("IPv4 source address: {:?}", cli.ipv4_src_addr);
    println!("IPv4 destination address: {:?}", cli.ipv4_dst_addr);
    println!("UDP source port: {:?}", cli.udp_src_port);
    println!("UDP destination port: {:?}", cli.udp_dst_port);
    println!("SCION daemon address: {:?}", cli.daemon_address);
    println!("Path ID: {:?}", cli.path_id);
    println!("Delay between echo requests: {:?}", cli.time);

    if let Some(ref payload) = cli.payload {
        println!("Custom SCMP payload: {}", payload);
    }

    // check if ISD-AS strings are valid
    let isd_as_pattern = r"^(0|[1-9][0-9]{0,4}|[1-5][0-9]{5}|6[0-4][0-9]{4}|65[0-4][0-9]{3}|6553[0-5])-([0-9a-fA-F]{1,4}):([0-9a-fA-F]{1,4}):([0-9a-fA-F]{1,4})$";
    let re = Regex::new(isd_as_pattern).unwrap();

    if !re.is_match(cli.src_isd_as.as_str()) {
        return Err(ValidationError(format!(
            "Invalid source ISD-AS format: '{}'",
            cli.src_isd_as
        )));
    }

    if !re.is_match(cli.dst_isd_as.as_str()) {
        return Err(ValidationError(format!(
            "Invalid destination ISD-AS format: '{}'",
            cli.dst_isd_as
        )));
    }

    let message: ScmpMessage = build_scmp_echo_request(&cli);
    let scmp_packet_bytes: Vec<u8> = build_scmp_message(&cli, &message).await;
    let udp_datagram: Vec<u8> = build_udp_datagram(&scmp_packet_bytes, &cli);
    send_datagram(&udp_datagram, &cli);

    Ok(())
}
