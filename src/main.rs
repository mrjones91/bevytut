use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup) // run 1x @ start
    .add_system(hello_world) //runs 1x / frame
    .add_system(print_names)
    .add_system(power_couple).run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((Person {
        name: "DJ".to_string(),
    }, 
    Employed {
        job: Job:SrCodeCrewInstructor("Sr Code School Instructor")
    }));

    commands.spawn((
        Person {
            name: "Constance".to_string()
        }, Employed {
            job: Job:SAHMpreneur("SAHMoMpreneur")
        }
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn power_couple(person_query: Query<&Person, With<Employed>>) {
    for person in per.iter() {
        println!("{} has a job at {}.", person.name, job)
    }
}

pub fn hello_world() {
    println!("Hello, world!");
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

pub enum Job {
    SrCodeCrewInstructor(String),
    SAHMpreneur(String)
}
