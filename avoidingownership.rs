fn main(){
    let s1:String=String::from("hello");//s1 owner
    let (s2,len)=calculate_len(s1);//ownership transfer,new owner
    println!("the string {} count is {}",s2,len);


}
fn calculate_len(s:String)->(String,usize){//s will be the new owner
    let length:usize=s.len();
    return (s,length);//return ownership transfer,5

}