use std::fmt;
struct MyServer {
    name: String,
    v_cpu: usize,
    ram: usize,
}

impl fmt::Display for MyServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Name: {}, vCPU: {}, RAM: {} GB", self.name, self.v_cpu, self.ram)
    }
}

impl fmt::Debug for MyServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyServer").field("name",&self.name).field("v_cpu",&self.v_cpu).field("ram",&self.ram).finish()
    }
}

impl Default for MyServer {
    fn default() -> MyServer {
        MyServer {
            name: "Default Name".to_string(),
            v_cpu: 2,
            ram: 4,
        }
    }
}

fn main() {
    let dns = MyServer { name: "DNS".to_string(), v_cpu: 4, ram: 16};
    println!("{}",dns);
    println!("{:?}",dns);

    let default_server = MyServer::default();
    println!("{}",default_server);
    println!("{:?}",default_server);
}
