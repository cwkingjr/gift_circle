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
pub struct Person {
    pub name: String,
    pub email_address: String,
    pub group: usize,
    pub theme: String,
}

impl Person {
    pub fn new(name: String, email_address: String, group: usize, theme: String) -> Self {
        Person { name, email_address, group, theme }
    }
}

fn largest_group_info(persons: &Vec<Person>) -> GroupInfo {        
    let group_counter = persons.iter().map(|p| p.group).collect::<Counter<_>>().most_common_ordered();
    GroupInfo::new(group_counter[0].0, group_counter[0].1)
}

fn largest_non_prev_group_info(persons: &Vec<Person>, previous_group: usize) -> GroupInfo {        
    let group_counter = persons.iter().filter(|p| p.group != previous_group).map(|p| p.group).collect::<Counter<_>>().most_common_ordered();
    GroupInfo::new(group_counter[0].0, group_counter[0].1)
}

fn has_possible_hamiltonian_path(persons: &Vec<Person>) -> bool {
    (largest_group_info(persons).size * 2) <= persons.len() 
}

// Last person gives gift to first person so can't be in the same group.
fn is_a_cycle(persons: &Vec<Person>) -> bool {
    if persons.len() < 2 {
        panic!("You must submit at least three people in order to form a gift circle.")
    }

    let first_group = persons.first().unwrap().group;
    let last_group = persons.last().unwrap().group;

    first_group != last_group
}

// Detemines that there are no adjoining entries with the same group
fn is_head_group_diff_from_tail_group(persons: &Vec<Person>) -> bool {
    let mut previous_group: usize = 0;
    for person in persons.iter() {
        if person.group == previous_group {
            return false;
        }
        previous_group = person.group;
    }
    true
}

fn is_gift_path_valid(persons: &Vec<Person>) -> bool {
    is_head_group_diff_from_tail_group(persons) && is_a_cycle(persons)
}

fn move_person(from_persons: &mut Vec<Person>, to_persons: &mut Vec<Person>, person: &Person) {
    from_persons.retain(|p| p != person);
    to_persons.push(person.clone());
}

fn generate_path(from_persons: &Vec<Person>) -> Vec<Person> {
    // Go through the list of available participants and generate a gift path
    // where noone gives a gift to anyone in their same group.

    // Preserve the original list of particiants in case we need it to start again when no path found.
    let mut available_persons: Vec<Person> = from_persons.clone();

    let mut persons_path:  Vec<Person> = vec!();

    let mut previous_group: usize = 0;
    
    while available_persons.len() > 0 {
        // If the largest group is half (or more) than the total remaining, we have
        // to pick someone from that group. Otherwise, pick randomly.

        let largest_np_group_info = largest_non_prev_group_info(&available_persons, previous_group);
        //println!("Large group info: {:?}", largest_np_group_info);
        //println!("Previous group: {:?}", previous_group);

        if (largest_np_group_info.size * 2) > available_persons.len() {
            // pick from largest, non-previous group 
            let candidates = available_persons.iter().filter(|&p| p.group == largest_np_group_info.group).cloned().collect::<Vec<Person>>();
            //println!("Largest remaining group candidates: {:#?}", candidates);
            let choice = candidates.choose(&mut rand::thread_rng()).unwrap();
            //println!("Largest remaining group choice: {:?}", choice);
            move_person(&mut available_persons, &mut persons_path, choice);
            previous_group = choice.group;
        } else {
            // pick from random, non-previous group
            let candidates = available_persons.iter().filter(|&p| p.group != previous_group).cloned().collect::<Vec<Person>>();
            //println!("Random group candidates: {:#?}", candidates);
            let choice = candidates.choose(&mut rand::thread_rng()).unwrap();
            //println!("Random group choice: {:?}", choice);
            move_person(&mut available_persons, &mut persons_path, choice);
            previous_group = choice.group;
        }
    }

    persons_path
}

pub fn get_gift_path(from_persons: Vec<Person>) -> Vec<Person> {

    //println!("Begining participants{:#?}", from_persons);
    //println!("Beginning largest group info: {:#?}", largest_group_info(&from_persons));
    //println!("Beginning particiant count: {:#?}", from_persons.len());

    let possible_path = has_possible_hamiltonian_path(&from_persons);
    //println!("Possible Hamlitonian path: {:#?}", possible_path);
    
    if !possible_path {
        panic!("Sorry, no possible hamiltonian path with this set of groups.")
    }

    // keep generating paths until one is valid
    let mut good_to_go = false;
    let mut mypath: Vec<Person> = vec!();
    while !good_to_go {
        mypath = generate_path(&from_persons);
        if is_gift_path_valid(&mypath) {
            good_to_go = true;
        }
    }

    mypath
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gift_path() {

        let person1= Person::new("Father".to_string(),"father@example.com".to_string(),1,"reading".to_string());
        let person2= Person::new("Mother".to_string(),"mother@example.com".to_string(),1,"coloring".to_string());
        let person3= Person::new("Son".to_string(),"son@example.com".to_string(),2,"programming".to_string());
        let person4= Person::new("Daughter".to_string(),"daughter@example.com".to_string(),2,"comedy".to_string());

        let participants = vec!(person1, person2, person3, person4);

        let mypath = get_gift_path(participants);
        assert_eq!(mypath.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_get_gift_path_panics_with_too_few_entries() {

        let person1= Person::new("Father".to_string(),"father@example.com".to_string(),1,"reading".to_string());
        let person2= Person::new("Mother".to_string(),"mother@example.com".to_string(),1,"coloring".to_string());
        let person3= Person::new("Son".to_string(),"son@example.com".to_string(),2,"programming".to_string());

        let participants = vec!(person1, person2, person3);

        get_gift_path(participants);
    }
    
}