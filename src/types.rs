use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::{DateTime, Utc};

pub type Price = Decimal;

#[derive(Debug, Clone)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Market,
    Limit { price: Price },
    Stop { stop_price: Price },
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub quantity: f64,
    pub kind: OrderType,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub quantity: Decimal,
    pub price: Price,
    pub taker_side: Side,
    pub timestamp: DateTime<Utc>,
}