use std::env;
use std::io;
use std::io::Write;

struct Net {
    field1: (i32, i32, i32),
    field2: (i32, i32, i32),
    field3: (i32, i32, i32),
    field4: (i32, i32, i32),
    field5: (i32, i32, i32),
    field6: (i32, i32, i32),
    field7: (i32, i32, i32),
    field8: (i32, i32, i32),
}

fn parse(field: (i32, i32, i32), strings: &[String])  {
    let mut nums: Vec<_> = strings.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    let mut previous = 0;
    let mut current = 0;
    let mut target = nums[3];
    let mut increment = field.0;
    
    while current <= target {
        previous = current;
        current += increment;
    }
    
    let mut networkid = &mut nums[0..4].to_vec();
    let mut nextid = &mut nums[0..4].to_vec();
    let mut broadcastid = &mut nums[0..4].to_vec();
    let mut firsthost = &mut nums[0..4].to_vec();
    let mut lasthost = &mut nums[0..4].to_vec();
    let mut subnet = &mut nums[0..4].to_vec();
    networkid[3] = previous;
    nextid[3] = current;
    broadcastid[3] = current - 1;
    firsthost[3] = previous + 1;
    lasthost[3] = current - 2;
    subnet.iter_mut().for_each(|elem| *elem = 255);
    subnet[3] = field.1;
    let networkid_string: String = networkid.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    let nextid_string: String = nextid.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    let broadcastid_string: String = broadcastid.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    let firsthost_string: String = firsthost.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    let lasthost_string: String = lasthost.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    let subnet_string: String = subnet.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    println!("Network ID: {:?}", networkid_string);
    println!("Next ID: {:?}", nextid_string);
    println!("Broadcast ID: {:?}", broadcastid_string);
    println!("First Host: {:?}", firsthost_string);
    println!("Last Host: {:?}", lasthost_string);
    println!("{:?} IP's, {:?} usable", increment, increment-2);
    println!("CIDR: {:?}", subnet_string);
    

    //String::from("Parsed result")
}

fn main() {

    let my_net = Net {
        field1: (1, 255, 32),
        field2: (2, 254, 31),
        field3: (4, 252, 30),
        field4: (8, 248, 29),
        field5: (16, 240, 28),
        field6: (32, 224, 27),
        field7: (64, 192, 26),
        field8: (128, 128, 25),
    };
    
    let args: Vec<String> = env::args().collect();

    println!("Please enter IP: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    let mut array: [String; 5] = Default::default();
    let mut i = 0;
    for c in input.chars() {
        if c == '.' || c == '/'  {
            i += 1;
        }
        else {
            array[i].push(c);
        }
    }

    if let Some(last) = array.last() {
        match last.as_str().parse::<i32>() {
            Ok(32) => {parse(my_net.field1, &array); } 
            Ok(31) => {parse(my_net.field2, &array); }
            Ok(30) => {parse(my_net.field3, &array); } 
            Ok(29) => {parse(my_net.field4, &array); }
            Ok(28) => {parse(my_net.field5, &array); } 
            Ok(27) => {parse(my_net.field6, &array); }
            Ok(26) => {parse(my_net.field7, &array); } 
            Ok(25) => {parse(my_net.field8, &array); }
           _ => {}
        }
    }
    //println!("Parsed {:?}", array);
    //println!("Network ID:{:?}", NetID)
}
