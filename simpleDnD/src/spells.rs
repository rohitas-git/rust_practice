pub struct Light{}
pub struct Dark{}
pub struct Chaos{}

pub trait Cast {
    fn cast(&self);
}

impl Cast for Light{fn cast(&self){println!("Light!");}}
impl Cast for Dark{fn cast(&self){println!("Dark!");}}
impl Cast for Chaos{fn cast(&self){println!("Chaos!");}}

pub struct Spellbook{
    pub spells: Vec<Box<dyn Cast>>,
}

impl Spellbook{
    pub fn run(&self){
        println!("Start casting my awesome spells :)");
        for spell in self.spells.iter(){spell.cast()}
    }
}