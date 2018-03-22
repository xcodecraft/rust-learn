
use std::result::Result ;

#[derive(Debug)]
pub struct Order
{}
#[derive(Debug)]
pub enum Error
{
    Unimpl,
}

pub type OrderVec = Vec<Order> ;
pub type OrderVecR = Result<OrderVec,Error> ;


pub trait ExchangeAPI
{
    fn fetch_orders(&self) -> OrderVecR ;
}
