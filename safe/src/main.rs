use std::io::Write;
fn output<T>(c : T) where T : std::fmt::Debug
{
    println!("{:?}",c);
}

fn output2(c : String,len: usize)
{
    println!("{},len {}",c.as_str(),len);
}

#[derive(Debug)]
struct Block
{
    data : [u64;10]
}
fn block_move()
{
    let block1 = Block{ data: [1,2,3,4,5,6,7,8,9,0] } ;
    let block2 = block1 ;

    let block3 = Box::new(Block{ data: [1,2,3,4,5,6,7,8,9,0] }) ;
    let block4 = block3 ;

    output(block2) ;
    output(block4) ;
}

type Rc  = std::rc::Rc<Block> ;
type Map = std::collections::HashMap<u32,Rc> ;
fn block_rc()
{
    let mut map = Map::new();
    map.insert(1, Rc::new(Block{ data: [1,2,3,4,5,6,7,8,9,0] })) ;
    map.insert(2, Rc::new(Block{ data: [1,2,3,4,5,6,7,8,9,1] })) ;
    map.get(&2).and_then(|v| Some(output(v)) ) ;
}



enum Country
{
    China,
    Usa,
}
struct Person {
    pub fst_name : String,
    pub sec_name : String,
}

fn lifetime_example() {
    let person = Person { fst_name : String::from("google"), sec_name : String::from("") } ;
    let sname  = surname(&Country::Usa,&person) ;
}
fn surname<'a>(c : &Country , person : &'a Person) -> &'a String {
    match c  {
        Country::Usa   => &person.sec_name ,
        Country::China => &person.fst_name ,
    }
}

fn move_example()
{
    let s1 = String::from("hello world") ;
    let s2 = s1 ;
    //output(s1) ;
    output(s2) ;
}
fn clone_example()
{
    let s1 = String::from("hello world") ;
    let s2 = s1.clone() ;
    output(s1) ;
    output(s2) ;
}
fn ref_example()
{
    let s1 = String::from("hello world") ;
    let len = length(&s1);
    output2(s1,len) ;

}
fn length(s : &String) ->usize
{
    s.len()
}

trait Curl { 
    fn get(&self  , url :&str) -> String ;
    fn post(&self , url :&str) -> String ;
}
struct Worker {
   logfile: std::fs::File,
   curl   : Box<Curl>,
}
impl Worker {
    fn get_order(&mut self) {
        let resp = self.curl.get("http://api.taobao.com/orders/list");
        self.logfile.write(resp.as_bytes()) ;
    }
    fn put_order(&mut self) {
        let resp = self.curl.post("http://api.taobao.com/orders/new");
        self.logfile.write(resp.as_bytes()) ;
    }
}

type FileCell = std::cell::RefCell<std::fs::File>;
struct Worker2 {
   logfile: FileCell,
   curl   : Box<Curl>,
}
impl Worker2 {
    fn get_order(&self) {
        let resp = self.curl.get("http://api.taobao.com/orders/list");
        self.logfile.borrow_mut().write(resp.as_bytes()) ;
    }
    fn put_order(&self) {
        let resp = self.curl.post("http://api.taobao.com/orders/new");
        self.logfile.borrow_mut().write(resp.as_bytes()) ;
    }
}



fn main() {

    move_example() ;
    clone_example() ;
    ref_example() ;
    lifetime_example() ;
    block_move();
    block_rc();
}
