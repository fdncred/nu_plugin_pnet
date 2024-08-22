mod net;

use net::{flags_to_nu, ips_to_nu, mac_to_nu};
use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand,
    SimplePluginCommand,
};
use nu_protocol::{record, Category, Example, LabeledError, Signature, Value};
use pnet::datalink::{self};

pub struct PNetPlugin;

impl Plugin for PNetPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(NetPlugin)]
    }
}

pub struct NetPlugin;

impl SimplePluginCommand for NetPlugin {
    type Plugin = PNetPlugin;

    fn name(&self) -> &str {
        "pnet"
    }

    fn description(&self) -> &str {
        "List network interfaces"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self)).category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "List network interfaces".into(),
            example: "pnet".into(),
            result: None,
        }]
    }

    fn run(
        &self,
        _config: &PNetPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(Value::list(
            datalink::interfaces()
                .iter_mut()
                .map(|interface|
                Value::record(record! {
                    "name" => Value::string(interface.name.clone(), call.head),
                    "description" => Value::string(interface.description.clone(), call.head),
                    "if_index" => Value::int(interface.index as i64, call.head),
                    "mac" => mac_to_nu(call, interface.mac),
                    "ips" => ips_to_nu(call, &interface.ips),
                    "flags" => flags_to_nu(call, interface),
                }, call.head))
                .collect(),
            call.head,
        ))
    }
}

fn main() {
    serve_plugin(&PNetPlugin, MsgPackSerializer {})
}
