use local_ipaddress;

fn main() {
    println!("{}", local_ipaddress::get().unwrap())
}
