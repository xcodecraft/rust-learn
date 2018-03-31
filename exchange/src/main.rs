#[macro_use]
extern crate log;
extern crate pretty_env_logger;
mod model ;
mod plat_impl ;
mod plat_a ;

use model::* ;
use plat_a::* ;





fn fetch_orders(apis : Vec<Box<ExchangeAPI>>)
{
    for api in apis 
    {
        let order_r= api.fetch_orders();
        debug!("order: {:?}",order_r) ;
    }
}

fn main() {

    let mut apis = Vec::new() ;
    apis.push(PlatA::new()) ;
    apis.push(PlatB::new()) ;
    pretty_env_logger::init();
    fetch_orders(apis) ;

}
