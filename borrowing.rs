fn main(){
    let s1:String=String::from("Hello");
    let len:usize=calculate_len(&s1);//passing the address
    println!("the string {} count is {}",s1,len);
}
fn calculate_len(s2:&String)->usize{//argument is in address type so &str
    return s2.len();
}