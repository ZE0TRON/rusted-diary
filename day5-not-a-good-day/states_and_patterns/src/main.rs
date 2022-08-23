struct Socket{
    socket_type: SocketType,
    port: u16
}
enum SocketType{
    UDP,
    TCP 
}

enum SinglePort{

}
enum Port{
    UDP(u16),
    TCP(u16),
    Dual(u16, u16)
} 

impl Port {
    fn text(&self) -> String{
        match self{
            Port::UDP(port) => port.to_string(),
            Port::TCP(port) => port.to_string(),
            Port::Dual(port1, port2 ) => port1.to_string() + " => " + &port2.to_string()  
        }
    }
}

enum Packet {
    Empty,
    TCP{data: String, port: Port},
    UDP{data: String, socket:Socket},
    Simple(String)
}

impl Packet {
    fn print(&self) {
        match self {
            Packet::Empty => {
                println!("Empty packet");
            },
            Packet::Simple(data) => {
                println!("Simple packet with data : {}", data);
            },
            Packet::TCP { data, port } => {
                println!("TCP packet with data : {} with port : {}",data , port.text());
            }
            Packet::UDP { .. } => {
                println!("Upps this is lost");
            }
        }
    }
}

fn find_packet(socket_type: &String) -> Option<Packet> {
    if socket_type == "TCP"  {
        Some(Packet::TCP { data: String::from("Test") , port: Port::TCP(55) })
    } else {
        None
    }
}

fn main() {
    let dns_socket = Socket{
        socket_type: SocketType::UDP,
        port: 53 
    };

    let http_port = Port::TCP(80);
    let dns_port = Port::UDP(53);
    let empty_packet = Packet::Empty;
    let tcp_packet = Packet::TCP { data: String::from("data1"), port: http_port };
    let udp_packet = Packet::UDP { data: String::from("data2"), socket: dns_socket};
    let simple_packet = Packet::Simple(String::from("data3"));
    let none1 : Option<i32> = None;
    let none2 : Option<String> = None;
    simple_packet.print();
    tcp_packet.print();
    udp_packet.print();
}
