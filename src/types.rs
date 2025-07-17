use rust_decimal::Decimal;
use rust_decimal_macros::dec;

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