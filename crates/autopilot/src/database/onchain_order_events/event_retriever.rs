use {
    contracts::cowswap_onchain_orders,
    ethcontract::{H160, H256, contract::AllEventsBuilder, transport::DynTransport},
    hex_literal::hex,
    shared::{ethrpc::Web3, event_handling::EventRetrieving},
};

const ORDER_PLACEMENT_TOPIC: H256 = H256(hex!(
    "cf5f9de2984132265203b5c335b25727702ca77262ff622e136baa7362bf1da9"
));
const ORDER_INVALIDATION_TOPIC: H256 = H256(hex!(
    "b8bad102ac8bbacfef31ff1c906ec6d951c230b4dce750bb0376b812ad35852a"
));
static ALL_VALID_ONCHAIN_ORDER_TOPICS: [H256; 2] =
    [ORDER_PLACEMENT_TOPIC, ORDER_INVALIDATION_TOPIC];

// Note: we use a custom implementation of `EventRetrieving` rather than using
// the one that is automatically derivable from the onchain-order contract. This
// is because the Rust implementation of the onchain-order contract assumes that
// only events that appear in the ABI can be emitted. In this custom
// implementation, we ignore all events except for those specified by the above
// hardcoded topics (which should correspond to the topics of all avents in the
// onchain-order contract ABI).
pub struct CoWSwapOnchainOrdersContract {
    web3: Web3,
    addresses: Vec<H160>,
}

impl CoWSwapOnchainOrdersContract {
    pub fn new(web3: Web3, addresses: Vec<H160>) -> Self {
        assert!(
            !addresses.is_empty(),
            "CoWSwapOnchainOrdersContract must have at least one address to listen to."
        );
        Self { web3, addresses }
    }
}

impl EventRetrieving for CoWSwapOnchainOrdersContract {
    type Event = cowswap_onchain_orders::Event;

    fn get_events(&self) -> AllEventsBuilder<DynTransport, Self::Event> {
        let mut events = AllEventsBuilder::new(self.web3.clone(), H160::default(), None);
        // We want to observe multiple addresses for events.
        events.filter = events.filter.address(self.addresses.clone());
        // Filter out events that don't belong to the ABI of `OnchainOrdersContract`.
        // This is done because there could be other unrelated events fired by
        // the contract which should be ignored. Also, it makes the request more
        // efficient, since it needs to return less events.
        events.filter = events
            .filter
            .topic0(ALL_VALID_ONCHAIN_ORDER_TOPICS.to_vec().into());
        events
    }
}
