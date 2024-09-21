/*pub mod example;
pub use crate::example::hello::hello_world;
pub use crate::example::{post,user,description};
use std::io;
use rand::Rng;
struct Student {
    name: String,
    roll_no: i32,
    parents: bool
}

enum IpAdd{
    V4(String),
    V6(String)
}
// struct ip{
//     type:IpAdd,

// }

struct Triangle{
    base: i32,
    height: i32
}
#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32
}

impl Triangle{
    fn area(&self)->i32{
        (self.base * self.height)/2
    }
}


fn add() -> i64 {
    let n = 2;
    let mut n1 = 3;
    n1 = n + n1;
    n1
}
fn input(){
    println!("enter input");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("error reading line");
    println!("{}",s);
}
fn factorial(x: i64)->i64{
    if x==1{
        1
    }else{
        x*factorial(x-1)

    }
}
pub mod my_module{
    pub mod sub_mod{
    pub fn hai(){
        println!("hai");
    }
    }
}

//use my_module::sub_mod::hai;
 
// fn main() {

//     // println!("Hello, world!");
//     // let m = factorial(5);
//     // println!("{}",m);
//     // //println!("{}",even(2));
//     // hai();
//     // for i in 1..10{
//     //     println!("{}",i);
//     // }
//         // let a : u16 = 16;
//         // let a: String = String::from("kk");
//         // println!("{}",a);
//         // {
//         //     let a: u32 =17;
//         //     print!("{}",a);

//         // }
//         // print!("{}",a);
//         //we can redifine as much as we want a variable
//         //ownership of value can belong to a variable and at a time there can be only one  owner in rust 
//         // let a : &str = "hello";
//         // let b: &str = takeOwnership(a);
//         // println!("{}",b);
//         // println!("{}",a);
//         //&stores string as binary and String::from() stores the string value in the heap

//         let a : String = String::from("HELLO");
//         let b: String = takeOwnership(a);
//         print!("{}",b);
//         let mut  student1 = Student{
//             name: String::from("Govind"),
//             roll_no: 12,
//             parents: true

//         };
//         print!("{}",student1.name);
//         student1.name=String::from("Gayathri");
//         print!("{}",student1.name);
//         let rect1: Rectangle = Rectangle { width: 10, height: 10 };
//         let a: i32 = rectangle_function(&rect1);
//         println!("{}",a);
//         let t: Triangle = Triangle { base:2, height: 3 };
//         println!("{}",t.area());
//         println!("{:?}",rect1); // we use define trait for this to be printed as normal format wont work on derived ones
//         }
       


fn rectangle_function(rect: &Rectangle )-> i32{   //using reference here since we dont want to change anythin
    rect.height*rect.width
}




fn takeOwnership(a: String)->String{
    a
}

// fn main(){
//     //random game
    
//     let a: i32 = rand::thread_rng().gen_range(1..5);
//     let mut tryy: i32 = a.clone();
//     let mut count: i32 = a.clone();
//     loop{
//     println!("Enter your guessing number");
//     let mut s = String::new();
//     io::stdin().read_line(&mut s).expect("coudnt read the number");
//     let s: i32 = s.trim().parse().expect("error converting to integer");
//     println!("The guessed number is {}",s);
   
//     println!("{}",a);
    
//     if a==s {
        
//         println!("Got it correct");
//         break;
        
//     }else{
//         tryy=tryy-1;
//         println!("Nope");
//     }
//     count=count-1;

//     if count ==0{
//         println!("Match over");
//         break;
//     }
// }
// println!("{}  {}",tryy,a);
// let score: i32 = ((tryy)*5/(a));
// println!("{}",score);
// }
/*
fn main(){
    hello_world();//importing function from another module
    //using traits
    let a =post{
        title: String::from("modi"),
        content: String::from("dictator")
    };
    post::descriptionfn(&a);

} */
*/

use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "http://localhost:3000/api/evaluate-student-answers1".parse::<Uri>().unwrap();
    let req = Request::builder()
        .method("POST")
        .uri(url)
        
            .body(Body::from(r#"{
   "className":"Class Y",
   "questionPaperName": "Computer Science",
   "qnNumber":1
}"#))?;
    let form_data = vec![
        ("file", "C:\Users\HP\Downloads\mod 2 notes.pdf")
    ];
    let body = serde_urlencoded::to_string(&form_data)?;
    let req = req.body(Body::from(body))?;
    let res = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", res.status());
    println!("Response: {:?}", body);

    Ok(())
}
