//! BridgeOutboundChannel pallet benchmarking
use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::traits::OnInitialize;

const BASE_NETWORK_ID: EVMChainId = EVMChainId::zero();

#[allow(unused_imports)]
use crate::Pallet as BridgeOutboundChannel;

benchmarks! {
    // Benchmark `on_initialize` under worst case conditions, i.e. messages
    // in queue are committed.
    on_initialize {
        let m in 1 .. T::MaxMessagesPerCommit::get() as u32;
        let p in 0 .. T::MaxMessagePayloadSize::get() as u32;

        for _ in 0 .. m {
            let payload: Vec<u8> = (0..).take(p as usize).collect();
            append_message_queue::<T>(BASE_NETWORK_ID, Message {
                network_id: BASE_NETWORK_ID,
                target: H160::zero(),
                nonce: 0u64,
                max_gas: 100000u64.into(),
                payload,
            });
        }

        let block_number = 0u32.into();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number) }
    verify {
        assert_eq!(<MessageQueues<T>>::get(BASE_NETWORK_ID).len(), 0);
    }

    // Benchmark 'on_initialize` for the best case, i.e. nothing is done
    // because it's not a commitment interval.
    on_initialize_non_interval {
        take_message_queue::<T>(BASE_NETWORK_ID);
        append_message_queue::<T>(BASE_NETWORK_ID, Message {
            network_id: BASE_NETWORK_ID,
            target: H160::zero(),
            nonce: 0u64,
            max_gas: 100000u64.into(),
            payload: vec![1u8; T::MaxMessagePayloadSize::get() as usize],
        });

        let interval: T::BlockNumber = 10u32.into();
        Interval::<T>::put(interval);
        let block_number: T::BlockNumber = 12u32.into();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number) }
    verify {
        assert_eq!(<MessageQueues<T>>::get(BASE_NETWORK_ID).len(), 1);
    }

    // Benchmark 'on_initialize` for the case where it is a commitment interval
    // but there are no messages in the queue.
    on_initialize_no_messages {
        take_message_queue::<T>(BASE_NETWORK_ID);

        let block_number = Interval::<T>::get();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number.into()) }
}

impl_benchmark_test_suite!(
    BridgeOutboundChannel,
    crate::test::new_tester(),
    crate::test::Test,
);
