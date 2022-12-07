mod net;

use net::{flags_to_nu, ips_to_nu, mac_to_nu};
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, Signature, Value};
use pnet::datalink::{self};

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("pnet")
            .usage("List network interfaces")
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        // eprintln!("hello from netplugin");
        if name != "pnet" {
            return Ok(Value::Nothing { span: call.head });
        }
        let cols = vec![
            "name".to_string(),
            "description".to_string(),
            "if_index".to_string(),
            "mac".to_string(),
            "ips".to_string(),
            "flags".to_string(),
        ];

        Ok(Value::List {
            vals: datalink::interfaces()
                .iter_mut()
                .map(|interface| Value::Record {
                    cols: cols.clone(),
                    vals: vec![
                        Value::String {
                            val: interface.name.clone(),
                            span: call.head,
                        },
                        Value::String {
                            val: interface.description.clone(),
                            span: call.head,
                        },
                        Value::Int {
                            val: interface.index as i64,
                            span: call.head,
                        },
                        mac_to_nu(call, interface.mac),
                        ips_to_nu(call, &interface.ips),
                        flags_to_nu(call, interface),
                    ],
                    span: call.head,
                })
                .collect(),
            span: call.head,
        })

        // Ok(Value::String {
        //     val: "hello from netplugin".to_string(),
        //     span: call.head,
        // })
    }
}

fn main() {
    serve_plugin(&mut NetPlugin {}, MsgPackSerializer {})
}
