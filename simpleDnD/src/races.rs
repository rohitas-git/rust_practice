pub struct Human{ pub name: String}
pub struct Elf{pub name: String}
pub struct Orc{pub name: String} 
pub struct Dwarf{pub name: String} 

pub trait Constitution{
    fn constitution_bonus(&self)-> u8{
        0 //default
    }
}
pub trait Strength{}
pub trait Agility{}
pub trait Wisdom{}

impl Constitution for Dwarf{
    fn constitution_bonus(&self)-> u8{
        1
    }
}

impl Constitution for Orc{
    fn constitution_bonus(&self)-> u8{
        2
    }
}

impl Constitution for Human{}
impl Constitution for Elf{}

pub trait Construct {
    fn new(namex:String)-> Self;
}

impl Construct for Human{
    fn new(namex:String)->Self{
        Human{name:namex}
    }
}

impl Construct for Elf{
    fn new(namex:String)->Self{
        Elf{name:namex}
    }
}

pub trait Hindi {
    
}

impl Hindi for Human{}
impl Hindi for Elf {}

pub fn speak_hindi<Speaks:Hindi>(_character:Speaks){println!("Namaste!!!");}