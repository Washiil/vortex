use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use std::collections::{BTreeMap, VecDeque};
use std::cmp::Reverse;

pub type Price = Decimal;

pub enum Side {
    Bid,
    Ask,
}

pub enum OrderType {
    Market,
    Limit { price: Price },
    Stop { stop_price: Price },
}

pub struct Order {
    pub id: u64,
    pub side: Side,
    pub quantity: f64,
    pub kind: OrderType,
}

struct OrderBook {
    exchange_name: String,
    pub bids: BTreeMap<Reverse<Price>, VecDeque<Order>>,
    pub asks: BTreeMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    pub fn new(name: String) -> Self {
        OrderBook {
            exchange_name: name,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_limit_order(&mut self, order: Order) {
        let price = match order.kind {
            OrderType::Limit { price } => price,
            _ => return,
        };

        match order.side {
            Side::Bid => {
                self.bids
                    .entry(Reverse(price))
                    .or_default()
                    .push_back(order);
            }
            Side::Ask => {
                self.asks
                    .entry(price)
                    .or_default()
                    .push_back(order);
            }
        }
    }
}