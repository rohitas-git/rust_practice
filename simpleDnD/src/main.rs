
use std::vec;

use simple_dnd::{races::*, spells::*};

fn main(){
    let my_dwarf = Dwarf {name: String::from("HellDwarf")};
    println!("{}'s constitution bonus: {}",my_dwarf.name, my_dwarf.constitution_bonus());

    let my_human= Human::new(String::from("Anuj Sir"));
    println!("{}'s constitution bonus: {}",my_human.name, my_human.constitution_bonus());

    speak_hindi(my_human);
    
    // Time for Spells
    let spell_book = Spellbook{
        spells:vec![
            Box::new(Light{}),
            Box::new(Dark{}),
            Box::new(Chaos{}),
            ],
    };

    spell_book.run();
}