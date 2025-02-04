use frame_support::weights::Weight;
pub trait WeightInfo {
    fn on_initialize(num_messages: u32, avg_payload_bytes: u32) -> Weight;
    fn on_initialize_non_interval() -> Weight;
    fn on_initialize_no_messages() -> Weight;
    fn register_channel() -> Weight;
    fn set_fee() -> Weight;
}

impl WeightInfo for () {
    fn on_initialize(_: u32, _: u32) -> Weight {
        0
    }
    fn on_initialize_non_interval() -> Weight {
        0
    }
    fn on_initialize_no_messages() -> Weight {
        0
    }
    fn set_fee() -> Weight {
        0
    }
    fn register_channel() -> Weight {
        0
    }
}
