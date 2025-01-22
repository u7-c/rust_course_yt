// the functions are same that are in cpp
fn main(){
    let item1:u8=4;
    let item2:u8=5;
    let result:u8;
    result = add(item1,item2);
    println!("the sum of two numbers is {}",result);

}
//it is iportanttomention the return type of function
fn add(item1:u8,item2:u8)->u8{
    return item1+item2;
}