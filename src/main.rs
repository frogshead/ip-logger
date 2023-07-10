
use sqlite::Connection;
use std::env;

#[derive(Debug)]
struct Host{
    name: String,
    ip: Option<String>
}

impl Host {
    fn new(name: String) -> Host{
        Host { name, ip: None }
    }
    fn store(&mut self){
        if self.ip == None {
            self.get_ip();
        }
        match &self.ip {
            Some(ip) => {
                Connection::open("./ips.db")
                .expect("Cannot open database")
                .execute(
                    format!(
                    "CREATE TABLE IF NOT EXISTS addresses(hostname TEXT NOT NULL, ip TEXT NOT NULL,Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);
                    INSERT INTO addresses VALUES('{}','{:?}',CURRENT_TIMESTAMP);
                    
                    ",self.name, ip)).expect("Writing to database failed.");

                
            },
            None =>{ println!("Missing Ip-address"); return ();}
        }
        
    }

    fn get_ip(&mut self){
        println!("Getting IP-addr for {:?}", self.name);
        match dns_lookup::lookup_host(&self.name){
            Ok(addr) => self.ip = Some(addr[0].to_string()),
            Err(_) => println!("Resolving IP address failed for {}", self.name)
        }
        
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return;
    }
    let hostname = &args[1];
    let mut host = Host::new(hostname.to_string());
    host.store();
}

fn usage() -> (){
    println!("Give a hostname as a first command line argument and resolved ip is stored to sqlite database");
    println!();
    println!("Example: ip-logger viitamäki.fi");
    ()
}