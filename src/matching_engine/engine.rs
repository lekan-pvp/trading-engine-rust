use super::orderbook::OrderBook;

pub struct TraidingPair {}

pub struct MatchingEngine {
    orderbooke: HashMap<TraidingPair, OrderBook>,
}
