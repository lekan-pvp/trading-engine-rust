mod matching_engine;

use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};

fn main() {
    let buy_order_frim_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_frim_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order_from_alice = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order_from_alice);

    println!("{:?}", orderbook);
}
