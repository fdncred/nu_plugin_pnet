use nu_plugin::EvaluatedCall;
use nu_protocol::Value;
use pnet::datalink::{MacAddr, NetworkInterface};
use pnet::ipnetwork::IpNetwork;

pub fn flags_to_nu(call: &EvaluatedCall, interface: &NetworkInterface) -> Value {
    Value::Record {
        cols: vec![
            "is_up".to_string(),
            "is_broadcast".to_string(),
            "is_loopback".to_string(),
            "is_point_to_point".to_string(),
            "is_multicast".to_string(),
        ],
        vals: vec![
            Value::Bool {
                val: interface.is_up(),
                span: call.head,
            },
            Value::Bool {
                val: interface.is_broadcast(),
                span: call.head,
            },
            Value::Bool {
                val: interface.is_loopback(),
                span: call.head,
            },
            Value::Bool {
                val: interface.is_point_to_point(),
                span: call.head,
            },
            Value::Bool {
                val: interface.is_multicast(),
                span: call.head,
            },
        ],
        span: call.head,
    }
}

pub fn mac_to_nu(call: &EvaluatedCall, mac: Option<MacAddr>) -> Value {
    if let Some(mac) = mac {
        Value::String {
            val: format!("{}", mac),
            span: call.head,
        }
    } else {
        Value::Nothing { span: call.head }
    }
}

pub fn ip_to_nu(call: &EvaluatedCall, ip: &IpNetwork) -> Value {
    let type_name = match ip {
        IpNetwork::V4(..) => "v4",
        IpNetwork::V6(..) => "v6",
    };
    Value::Record {
        cols: vec!["type".to_string(), "addr".to_string(), "prefix".to_string()],
        vals: vec![
            Value::String {
                val: type_name.to_string(),
                span: call.head,
            },
            Value::String {
                val: format!("{}", ip),
                span: call.head,
            },
            Value::Int {
                val: ip.prefix() as i64,
                span: call.head,
            },
        ],
        span: call.head,
    }
}

/// Convert a slice of ipnetworks to nushell values
pub fn ips_to_nu(call: &EvaluatedCall, ips: &[IpNetwork]) -> Value {
    Value::List {
        vals: ips.iter().map(|ip| ip_to_nu(call, ip)).collect(),
        span: call.head,
    }
}
