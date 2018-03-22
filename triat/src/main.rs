#[macro_use]
extern crate log;
extern crate pretty_env_logger;
mod model ;
mod plat_impl ;
mod plat_a ;

use model::* ;
use plat_a::* ;





fn fetch_orders(api : Box<ExchangeAPI>)
{
    let order_r= api.fetch_orders();
    debug!("order: {:?}",order_r) ;
}

fn main() {

    pretty_env_logger::init();
    let a =  PlatA::new();
    let b =  PlatB::new();
    fetch_orders(a) ;
    fetch_orders(b) ;

}
