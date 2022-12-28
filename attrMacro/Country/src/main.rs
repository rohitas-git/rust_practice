use hello_attr::hello;

#[hello]
pub fn myCountry(){
    println!("Say hello to India!");
}

fn main() {
    myCountry();
}
