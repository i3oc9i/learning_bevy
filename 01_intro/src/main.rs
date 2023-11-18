use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)    
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_does_job)
            .add_system(people_with_job)
            .add_system(people_without_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (Person {
            name: "i3oc9i".to_string()
        },
        Employed {
            job: Job::Director
        })
    );
    commands.spawn(
        Person {
            name: "Lucas".to_string()
        }
    );
    commands.spawn(
        (Person {
            name: "Alessio".to_string()
        },
        Employed {
            job: Job::Engineer
        })
    );
}

pub fn print_names(query: Query<&Person>) {
    for person in query.iter() {
        println!("Name: {}.", person.name);
    }
}

pub fn people_with_job(query: Query<&Person, With<Employed>>) {
    for person in query.iter() {
        println!("Name: {} has a job.", person.name);
    }
}

pub fn people_without_job(query: Query<&Person, Without<Employed>>) {
    for person in query.iter() {
        println!("Name: {} does not have a job.", person.name);
    }
}

pub fn people_does_job(query: Query<(&Person, &Employed)>) {
    for (person, employed) in query.iter() {
        let job_name = match employed.job {
            Job::Engineer => "Engineer",
            Job::Manager => "Manager",
            Job::Director => "Director"
        };
        
        println!("{1} is a {0}.", job_name, person.name); // {1}, {0} for the fun of it!
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Engineer,
    Manager,
    Director
}
