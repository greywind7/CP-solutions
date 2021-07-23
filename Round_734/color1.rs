use std::collections::HashSet;

fn inp() -> String{
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    inp
}

fn main(){
    let t = inp().trim().parse().unwrap();
    for _ in 0..t{
        let mut red = HashSet::new();
        let mut green = HashSet::new();
        let mut red_t = true;
        let mut no_r = 0;
        let mut st = inp();
        st = st.trim().to_string();
        for i in st.chars() {
            if red.contains(&i) || green.contains(&i){continue}
            else if red_t{
                red.insert(i);
                no_r += 1;
                red_t = !red_t;
            }
            else {
                green.insert(i);
                red_t = !red_t;
            }
        }
        println!("{}",no_r);
    }
}