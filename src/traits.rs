// Traits can be used to implement interface definitions, or polymorphism.
trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => { "Wing Chun".to_string() }
            Character::Archer => { "Kung Fu".to_string() }
            Character::Wizard => { "Thai Chi".to_string() }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        let my_character: Character = Character::Archer;
        let chosen_fighting_style = my_character.choose_style();
        
        dbg!(chosen_fighting_style);
    }

}