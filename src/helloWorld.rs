use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

struct Person;

struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

struct GreetTimer(Timer);

// Res for read access, ResMut for write access
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    let timer_has_finished = timer.0.tick(time.delta()).just_finished();

    if timer_has_finished {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("John Forster".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Alice Stewart".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Tom Page".to_string()));
}
