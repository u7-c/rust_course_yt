fn main(){
    let mut string1="hi my name is aryan";
    println!("{}",string1);
    // this stores the string in &s type
    //there are two types string and &str 
    // but by default its &strjust as u see above
    let mut string2:String=String::from("hi my name is aryan");
    println!("{}",string2);
    //its in string type now
    string2.push_str(" yo nigga");
    println!("{}",string2);
    //this is how u add a string to a existing one which is alreay existing
    // by strigname.push_str( "")

}
//what is the difference?
// string-dynamiclength strings - heap allocated
//&str fixed length strings - fixed length strings - memory u cant change he size
 