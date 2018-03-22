
use model::* ;

pub trait Conv 
{
    fn to_orders(&self,str: &String) ->OrderVecR ;
}

pub trait Fetch
{
    fn local_fetch_data(&self, path: &String) -> String ;
    fn remot_fetch_data(&self, path: &String) -> String ;
}
pub trait Conf 
{
    fn local_path(&self) -> String ;
    fn remot_path(&self) -> String ;
    fn source(&self) -> &Source ;
}



pub enum Source {
    Local,
    Remot,
}
pub trait Plat{}


impl <T> Fetch for T  where T: Plat 
{
    fn local_fetch_data(&self, path: &String) -> String 
    {
        String::from("")

    }
    fn remot_fetch_data(&self, path: &String) -> String 
    {
        String::from("")

    }

}
impl <T> Conv for T 
    where T: Plat
{
    fn to_orders(&self,str : &String) -> OrderVecR 
    {
        Err(Error::Unimpl)

    }

}

impl <T> ExchangeAPI for  T
    where T: Conf + Fetch + Conv 
{
    fn fetch_orders(&self) -> OrderVecR
    {
        let data = match *self.source(){
            Source::Local => self.local_fetch_data(&self.local_path()) ,
            Source::Remot => self.remot_fetch_data(&self.remot_path()) , 
        } ;
        self.to_orders(&data)
    }

}
