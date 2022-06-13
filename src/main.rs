use counter::Counter;
use rand::seq::SliceRandom;


#[derive(Debug)]
struct GroupInfo {
    group: usize,
    size: usize,
}

impl GroupInfo {
    fn new(group: usize, size: usize) -> Self {
        GroupInfo { group, size }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Person {
    name: String,
    email_address: String,
    group: usize,
    theme: String,
}

impl Person {
    fn new(name: String, email_address: String, group: usize, theme: String) -> Self {
        Person { name, email_address, group, theme }
    }
}

#[derive(Debug)]
struct Participants {
    available: Vec<Person>,
    gift_path: Vec<Person>
}

impl Participants {
    fn new() -> Self {
        Participants {
            available: vec![],
            gift_path: vec![]
        }
    }

    fn add_participant(&mut self, person: Person) {
        self.available.push(person);
    }

    fn largest_group_info(&self) -> GroupInfo {        
        let group_counter = self.available.iter().map(|p| p.group).collect::<Counter<_>>().most_common_ordered();
        GroupInfo::new(group_counter[0].0, group_counter[0].1)
    }

    fn largest_non_prev_group_info(&self, previous_group: usize) -> GroupInfo {        
        let group_counter = self.available.iter().filter(|p| p.group != previous_group).map(|p| p.group).collect::<Counter<_>>().most_common_ordered();
        GroupInfo::new(group_counter[0].0, group_counter[0].1)
    }

    fn available_count(&self) -> usize {
        self.available.len()
    }
    fn gift_path_count(&self) -> usize {
        self.gift_path.len()
    }

    fn has_possible_hamiltonian_path(&self) -> bool {
        (self.largest_group_info().size * 2) <= self.available_count() 
    }

    // Last person gives gift to first person so can't be in the same group.
    fn is_gift_path_a_cycle(&self) -> bool {
        if self.gift_path_count() < 2 {
            return false;
        }

        let first_group = self.gift_path.first().unwrap().group;
        let last_group = self.gift_path.last().unwrap().group;

        first_group != last_group
    }

    // Detemines that there are no adjoining entries with the same group
    fn is_gift_path_head_group_diff_from_tail_group(&self) -> bool {
        let mut previous_group: usize = 0;
        for person in self.gift_path.iter() {
            if person.group == previous_group {
                return false;
            }
            previous_group = person.group;
        }
        true
    }

    fn is_gift_path_valid(&self) -> bool {
        self.is_gift_path_head_group_diff_from_tail_group() & self.is_gift_path_a_cycle()
    }

    fn move_person_from_available_to_path(&mut self, person: &Person) {
        self.available.retain(|p| p != person);
        self.gift_path.push(person.clone());
    }

    fn generate_random_gift_path(&mut self) {
        // Go through the list of available participants and generate a gift path
        // where noone gives a gift to anyone in their same group.

        let mut previous_group: usize = 0;
        
        while self.available_count() > 0 {
            // If the largest group is half (or more) than the total remaining, we have
            // to pick someone from that group. Otherwise, pick randomly.

            let largest_np_group_info = self.largest_non_prev_group_info(previous_group);
            println!("Large group info: {:?}", largest_np_group_info);
            println!("Previous group: {:?}", previous_group);
            let available_clone = self.available.clone();

            if (largest_np_group_info.size * 2) > self.available_count() {
                // pick from largest, non-previous group
                let candidates = available_clone.iter().filter(|&p| p.group == largest_np_group_info.group).collect::<Vec<_>>();
                println!("Largest remaining group candidates: {:#?}", candidates);
                let choice = candidates.choose(&mut rand::thread_rng()).unwrap();
                println!("Largest remaining group choice: {:?}", choice);
                self.move_person_from_available_to_path(*choice);
                previous_group = choice.group;
            } else {
                // pick from random, non-previous group
                let candidates = available_clone.iter().filter(|&p| p.group != previous_group).collect::<Vec<_>>();
                println!("Random group candidates: {:#?}", candidates);
                let choice = candidates.choose(&mut rand::thread_rng()).unwrap();
                println!("Random group choice: {:?}", choice);
                self.move_person_from_available_to_path(*choice);
                previous_group = choice.group;
            }
        }
    }
}

fn main() {
    let person1= Person::new("Father".to_string(),"father@example.com".to_string(),1,"reading".to_string());
    let person2= Person::new("Mother".to_string(),"mother@example.com".to_string(),1,"coloring".to_string());
    let person3= Person::new("Son 2".to_string(),"son@example.com".to_string(),1,"programming".to_string());
    let person4= Person::new("Daughter 2".to_string(),"duaghter2@example.com".to_string(),1,"camping".to_string());
    let person5= Person::new("Daughter".to_string(),"daughter@example.com".to_string(),3,"writing".to_string());
    let person6= Person::new("Son 2".to_string(),"son2@example.com".to_string(),3,"doctoring".to_string());

    let mut participants = Participants::new();
    participants.add_participant(person1);
    participants.add_participant(person2);
    participants.add_participant(person3);
    participants.add_participant(person4);
    participants.add_participant(person5);
    participants.add_participant(person6);

    //println!("Begining participants{:#?}", participants);
    println!("Beginning largest group info: {:#?}", participants.largest_group_info());
    println!("Beginning available count: {:#?}", participants.available_count());

    println!("Possible Hamlitonian path: {:#?}", participants.has_possible_hamiltonian_path());
    if participants.has_possible_hamiltonian_path() {
        participants.generate_random_gift_path();

        println!("Is cycle: {:#?}", participants.is_gift_path_a_cycle());
        println!("Is Gift Path valid: {:#?}", participants.is_gift_path_valid());
    }

    println!("Gift Path count: {:#?}", participants.gift_path_count());
    println!("Ending participants{:#?}", participants);

}
