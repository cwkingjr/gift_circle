mod gift_path;

use gift_path::{Person, get_gift_path};

fn main() {

    let person1= Person::new("Father".to_string(),"father@example.com".to_string(),1,"reading".to_string());
    let person2= Person::new("Mother".to_string(),"mother@example.com".to_string(),1,"coloring".to_string());
    let person3= Person::new("Son 2".to_string(),"son@example.com".to_string(),2,"programming".to_string());
    let person4= Person::new("Daughter 2".to_string(),"duaghter2@example.com".to_string(),2,"camping".to_string());
    let person5= Person::new("Daughter".to_string(),"daughter@example.com".to_string(),3,"writing".to_string());
    let person6= Person::new("Son 2".to_string(),"son2@example.com".to_string(),3,"doctoring".to_string());

    let participants = vec!(person1, person2, person3, person4, person5, person6);
    println!("Submitted participants{:#?}", &participants);

    let mypath = get_gift_path(participants);

    println!("Gift Circle count: {:#?}", mypath.len());
    println!("Gift Circle order: {:#?}", mypath);
}