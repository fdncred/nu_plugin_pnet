mod net;

use net::{flags_to_nu, ips_to_nu, mac_to_nu};
use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin,
    PluginCommand, SimplePluginCommand,
};
use nu_protocol::{record, Category, PluginExample, PluginSignature, Value};
use pnet::datalink::{self};

pub struct PNetPlugin;

impl Plugin for PNetPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(NetPlugin)]
    }
}

pub struct NetPlugin;

impl SimplePluginCommand for NetPlugin {
    type Plugin = PNetPlugin;

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("pnet")
            .usage("List network interfaces")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "List network interfaces".into(),
                example: "pnet".into(),
                result: None,
            }])
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
