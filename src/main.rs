mod net;

use net::{flags_to_nu, ips_to_nu, mac_to_nu};
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Value};
use pnet::datalink::{self};

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("pnet")
            .usage("List network interfaces")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "List network interfaces".into(),
                example: "pnet".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
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
    }
}

fn main() {
    serve_plugin(&mut NetPlugin {}, MsgPackSerializer {})
}
