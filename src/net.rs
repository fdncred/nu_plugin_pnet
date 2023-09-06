use nu_plugin::EvaluatedCall;
use nu_protocol::{record, Value};
use pnet::datalink::{MacAddr, NetworkInterface};
use pnet::ipnetwork::IpNetwork;

pub fn flags_to_nu(call: &EvaluatedCall, interface: &NetworkInterface) -> Value {
    let span = call.head;
    Value::record(
        record! {
            "is_up" => Value::bool(interface.is_up(), span),
            "is_broadcast" => Value::bool(interface.is_broadcast(), span),
            "is_loopback" => Value::bool(interface.is_loopback(), span),
            "is_point_to_point" => Value::bool(interface.is_point_to_point(), span),
            "is_multicast" => Value::bool(interface.is_multicast(), span),
        },
        span,
    )
}

pub fn mac_to_nu(call: &EvaluatedCall, mac: Option<MacAddr>) -> Value {
    if let Some(mac) = mac {
        Value::string(format!("{}", mac), call.head)
    } else {
        Value::nothing(call.head)
    }
}

pub fn ip_to_nu(call: &EvaluatedCall, ip: &IpNetwork) -> Value {
    let type_name = match ip {
        IpNetwork::V4(..) => "v4",
        IpNetwork::V6(..) => "v6",
    };
    Value::record(
        record! {
            "type" => Value::string(type_name, call.head),
            "addr" => Value::string(format!("{}", ip), call.head),
            "prefix" => Value::int(ip.prefix() as i64, call.head),
        },
        call.head,
    )
}

/// Convert a slice of ipnetworks to nushell values
pub fn ips_to_nu(call: &EvaluatedCall, ips: &[IpNetwork]) -> Value {
    Value::list(ips.iter().map(|ip| ip_to_nu(call, ip)).collect(), call.head)
}
