fn main() {

    let mut state = "1113222113".to_owned();

    for _ in 0..50 {
        let mut temp = Vec::new();

        let mut it = state.chars();
        let mut previous = it.next().unwrap();
        let mut occurrences = 1;
        for current in it {
            if current == previous {
                occurrences += 1;
            }
            else {
                temp.push(occurrences.to_string());
                temp.push(previous.to_string());
                occurrences = 1;
                previous = current;
            }
        }
        temp.push(occurrences.to_string());
        temp.push(previous.to_string());
        state = temp.concat();
    }
    println!("{}", state.len());
}