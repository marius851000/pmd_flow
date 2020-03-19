use pmd_flow::FlowDataOutput;
use pmd_flow::{FlowData, FlowDataValue};
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from("./script_flow_data_us.bin");
    let input_file = File::open(&input_path).unwrap();

    let flow = FlowData::new(input_file).unwrap();
    let output = FlowDataOutput::new(flow);

    let serialized = serde_json::to_string(&output).unwrap();
    //println!("serialized: {:?}", serialized);
    let output_path = PathBuf::from("./out.json");
    let mut output_file = File::create(&output_path).unwrap();
    use std::io::Write;
    output_file.write(serialized.as_bytes()).unwrap();
    //Some hacky hack...
    /* let dic4 = flow.get_dictionary_mut(4).unwrap();
    dic4.insert("to".into(), FlowDataValue::String("$0x000A".into()));
    let dic192 = flow.get_dictionary_mut(192).unwrap();
    dic192.insert("map".into(), FlowDataValue::String("FM_TW00_SCHOOL00".into()));
    dic192.insert("place".into(), FlowDataValue::String("EV_SCHOOL00_START_POINT02".into()));
    let output_path = PathBuf::from("./script_handmade.bin");
    {
        let mut output_file = File::create(&output_path).unwrap();
        flow.write(&mut output_file).unwrap();
        println!("wrote");
    }

    let input_file_custom = File::open(&output_path).unwrap();
    let _flow_bis = FlowData::new(input_file_custom).unwrap(); */
}
