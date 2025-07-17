use crate::types::{Price, Order, OrderType, Side};

use std::collections::{BTreeMap, VecDeque};
use std::cmp::Reverse;

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

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Bid => self.add_buy_order(order),
            Side::Ask => self.add_sell_order(order),
        }
    }

    fn add_buy_order(&mut self, order: Order) {
        match order.kind {
            OrderType::Market => todo!(),
            OrderType::Limit { price } => todo!(),
            OrderType::Stop { stop_price } => todo!(),
        }
    }

    fn add_sell_order(&mut self, order: Order) {
        match order.kind {
            OrderType::Market => todo!(),
            OrderType::Limit { price } => todo!(),
            OrderType::Stop { stop_price } => todo!(),
        }
    }
}