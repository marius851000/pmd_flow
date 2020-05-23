//! This library can read flow file, as used in Pokemon Super Mystery Dungeon.
//!
//! While it can read and write the script_flow_data_us.bin file used in this game, this library
//! isn't finished. It doesn't support different flow file used in this game, and there is many
//! assertion that crash the program rather than returning Error.
mod flowdata;
pub use flowdata::{FlowData, FlowDataError, FlowDataValue};

mod tool;

mod output;
pub use output::FlowDataOutput;
