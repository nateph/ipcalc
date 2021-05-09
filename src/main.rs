use clap::Clap;
use regex::Regex;

/// Perform simple operations, and get information on IP addresses and networks
#[derive(Clap, Debug)]
#[clap(version)]
struct Options {
    /// IP address to operate on.
    ip_address: String,
}

fn to_ipv4(octects: &[u32]) -> String {
    format!(
        "{}.{}.{}.{}",
        octects[0], octects[1], octects[2], octects[3]
    )
}

fn to_binary(octects: &[u32]) -> String {
    format!(
        "{:08b}.{:08b}.{:08b}.{:08b}",
        octects[0], octects[1], octects[2], octects[3]
    )
}

fn to_broadcast(ip_octects: &[u32], prefix: &u32) -> Vec<u32> {
    let address = ip_octects[0] << 24 | ip_octects[1] << 16 | ip_octects[2] << 8 | ip_octects[3];
    let netmask = 0xffffffff << (32 - prefix);
    let broadcast_addr = address | !netmask;
    to_octets(broadcast_addr)
}

fn to_wildcard(prefix: &u32) -> Vec<u32> {
    let netmask = 0xffffffff << (32 - prefix);
    to_octets(!netmask)
}

fn to_network_id(ip_octects: &[u32], netmask_octects: &[u32]) -> Vec<u32> {
    vec![
        ip_octects[0] & netmask_octects[0],
        ip_octects[1] & netmask_octects[1],
        ip_octects[2] & netmask_octects[2],
        ip_octects[3] & netmask_octects[3],
    ]
}

fn max_hosts(broadcast_octects: &[u32]) -> Vec<u32> {
    vec![
        broadcast_octects[0],
        broadcast_octects[1],
        broadcast_octects[2],
        broadcast_octects[3] - 1,
    ]
}

fn min_hosts(network_id_octects: &[u32]) -> Vec<u32> {
    vec![
        network_id_octects[0],
        network_id_octects[1],
        network_id_octects[2],
        network_id_octects[3] + 1,
    ]
}

fn netmask_octects_from_prefix(prefix: u32) -> Vec<u32> {
    let netmask = 0xffffffff << (32 - prefix);
    to_octets(netmask)
}

fn to_octets(ip: u32) -> Vec<u32> {
    vec![ip >> 24 & 0xff, ip >> 16 & 0xff, ip >> 8 & 0xff, ip & 0xff]
}

/// Validate the cidr formatted ip and return it as a Vector
fn get_ip_from_cidr(line: &str) -> Result<Vec<u32>, &'static str> {
    let re = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d{1,2})$").unwrap();
    let capture = re.captures(line).ok_or("invalid cidr address")?;
    let limits: [u32; 5] = [255, 255, 255, 255, 32];

    let mut ip_vec: Vec<u32> = Vec::with_capacity(5);
    // We have to .get(i + 1) here because the capture group is group 1
    for (i, m) in limits.iter().enumerate() {
        let digit = capture.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
        if digit > *m {
            return Err("invalid cidr address");
        }
        ip_vec.push(digit);
    }

    Ok(ip_vec)
}

fn print_all(ip: &[u32], prefix: &u32) {
    let netmask = netmask_octects_from_prefix(*prefix);
    let network_id = to_network_id(ip, &netmask);
    let wildcard = to_wildcard(prefix);
    let broadcast = to_broadcast(ip, &prefix);
    let host_max = max_hosts(&broadcast);
    let host_min = min_hosts(&network_id);
    println!("Address:   {:16} {}", to_ipv4(ip), to_binary(ip));
    println!(
        "Netmask:   {:16} {}",
        to_ipv4(&netmask),
        to_binary(&netmask)
    );
    println!(
        "Wildcard:  {:16} {}",
        to_ipv4(&wildcard),
        to_binary(&wildcard)
    );
    println!("=>");
    println!(
        "Network:   {:16} {}",
        to_ipv4(&network_id),
        to_binary(&network_id)
    );
    println!(
        "HostMin:   {:16} {}",
        to_ipv4(&host_min),
        to_binary(&host_min)
    );
    println!(
        "HostMax:   {:16} {}",
        to_ipv4(&host_max),
        to_binary(&host_max)
    );
    println!(
        "Broadcast: {:16} {}",
        to_ipv4(&broadcast),
        to_binary(&broadcast)
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    let mut ip = get_ip_from_cidr(&options.ip_address)?;
    let prefix = ip.pop().unwrap();

    print_all(&ip, &prefix);

    Ok(())
}
