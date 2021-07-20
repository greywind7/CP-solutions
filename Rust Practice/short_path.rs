// https://codeforces.com/contest/1547/problem/A
fn inp(n:i32) -> (i32,i32) {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    if n == 0{return (0,0)};
    let mut it = inp.split_whitespace();
    let a = it.next().unwrap().parse::<i32>().unwrap();
    if n == 1{
        return (a,0);
    }
    let b = it.next().unwrap().parse::<i32>().unwrap();
    return (a,b);
}

fn main(){
    let t = inp(1).0;
    for _ in 0..t {
        inp(0);
        let a = inp(2);
        let b = inp(2);
        let f = inp(2);
        if a.0 != b.0 && a.1 != b.1 {
            println!("{}", ((a.0-b.0).abs() + (a.1 - b.1).abs()));
        }
        else if a.0 == b.0 {
            if f.0 == a.0 && !((f.1 > a.1 && f.1 > b.1) || (f.1 < a.1 && f.1 < b.1)){
                println!("{}",(a.1 - b.1).abs() + 2);
            }
            else{
                println!("{}",(a.1 - b.1).abs());
            }
        }
        else if a.1 == b.1 {
            if f.1 == a.1 && !((f.0 > a.0 && f.0 > b.0) || (f.0 < a.0 && f.0 < b.0)){
                println!("{}",(a.0 - b.0).abs() + 2);
            }
            else{
                println!("{}",(a.0 - b.0).abs());
            }
        }
    }
}