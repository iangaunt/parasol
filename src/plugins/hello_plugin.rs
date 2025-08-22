use bevy::{prelude::*, time::TimerMode};

// Name component for implementation in other components.
#[derive(Component)]
#[allow(dead_code)]
struct Name(String);

// Person for component.
#[derive(Component)]
struct Person;

// Resources are pieces of globally unique data.
#[derive(Resource)]
pub struct GreetTimer(Timer);

#[derive(Resource)]
pub struct HelloTimer(Timer);

// Creation of our own plugin.
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // Creates the "World" for Bevy to operate on.

        // Initializes the GreetTimer & HelloTimer resources.
        app.insert_resource(GreetTimer(
            Timer::from_seconds(3.0, TimerMode::Repeating)
        ));
        
        // Initializes the HelloTimer resource.
        app.insert_resource(HelloTimer(
            Timer::from_seconds(5.0, TimerMode::Repeating)
        ));

        app.add_systems(Startup, people);
        app.add_systems(Update, (hello, (
            (update_people, greet).chain() // Makes sure update_people occurs before greet
        ))); // hello does not need to be chained - allows to run in parallel
    }
}

// Creates a series of People components with the Name attribute.
fn people(mut commands: Commands) {
    commands.spawn((Person, Name("Ian G".to_string())));
    commands.spawn((Person, Name("Kyle G".to_string())));
}

// Greets the people given in the Query.
// The Res<Time> allows for a fixed loop based on elapsed time.
fn greet(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // Update timer with time elapsed.
    // If that leads to a completed timer, then greet all.
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            // Fetches the zero-th (first) property of the Name component.
            println!("Hello, {}!", name.0);
        }
    }
}

// Function for basic system functionality.
fn hello(time: Res<Time>, mut timer: ResMut<HelloTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Hello, world!");
    }
}

// Mutable query example.
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Ian G" {
            name.0 = "Ian H".to_string();
            break;
        }
    }
}
