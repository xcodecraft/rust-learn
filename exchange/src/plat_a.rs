use model::* ;
use plat_impl::* ;
pub struct PlatA 
{
    src : Source ,
}

impl PlatA
{
    pub fn new() -> Box<ExchangeAPI>
    {
        Box::new(PlatA{src: Source::Local})
    }
}
impl Plat for PlatA{}
impl Conf for PlatA
{
    fn local_path(&self) -> String 
    {
        String::from("")
    }
    fn remot_path(&self) -> String 
    {
        String::from("")
    }
    fn source(&self) -> &Source
    {
        &self.src
    }

}

pub struct PlatB 
{
    src : Source ,
}

impl PlatB
{
    pub fn new() -> Box<ExchangeAPI>
    {
        Box::new(PlatB{src: Source::Local})
    }
}
impl Plat for PlatB{}
impl Conf for PlatB
{
    fn local_path(&self) -> String 
    {
        String::from("")
    }
    fn remot_path(&self) -> String 
    {
        String::from("")
    }
    fn source(&self) -> &Source
    {
        &self.src
    }

}

