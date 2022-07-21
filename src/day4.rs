use md5;

fn main() {
    let secret = "ckczppom".to_string();
    for i in 1.. {
        let temp = md5::compute([secret.clone(), i.to_string()].concat().bytes().collect::<Vec<u8>>()).to_vec();
        if temp[0] == 0 && temp[1] == 0 && temp[2] & 0xf0 == 0 {
            println!("five {}", i);
        }
        if temp[0] == 0 && temp[1] == 0 && temp[2] == 0 {
            println!("six {}", i);
            break;
        }
    }
}