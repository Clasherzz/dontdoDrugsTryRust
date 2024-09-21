pub mod hello{
    pub fn hello_world(){
        println!("hello");
        }
}

pub struct post{
    pub title: String,
    pub content: String,
}

pub struct user{
    pub name: String,
    pub age: u8,
}



pub trait description {
    fn descriptionfn(&s:&post){
        println!("write description");
    }

}
impl description for post{
    fn descriptionfn(&s:&post){
        println!("post description");
    }
}

