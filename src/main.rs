use pmd_flow::FlowData;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from("./script_flow_data_us.bin");
    let input_file = File::open(&input_path).unwrap();

    let flow = FlowData::new(input_file).unwrap();
    //Some hacky hack...
    //let dic4 = flow.get_dictionary_mut(4).unwrap();
    //dic4.insert("to".into(), FlowDataValue::String("$0x572F".into()));
    let output_path = PathBuf::from("./script_handmade.bin");
    {
        let mut output_file = File::create(&output_path).unwrap();
        flow.write(&mut output_file).unwrap();
        println!("wrote");
    }

    let input_file_custom = File::open(&output_path).unwrap();
    let _flow_bis = FlowData::new(input_file_custom).unwrap();
}
