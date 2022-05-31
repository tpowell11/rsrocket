use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Component {
    name : String,
    typ : i8,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Rocket {
    components: Vec<Component>,
}
impl Rocket {
    pub fn new() -> Self {
        Self{
            components : Vec::new()
        }
    }
    pub fn addComponent(mut self, c : Component){
        self.components.push(c)
    }
}

fn main() {
    println!("Hello, world!");
}
