/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.

Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.

It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.

Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.

Declare a `event_time` variable set to a string of
"06:00".

Use variable shadowing to redeclare `event_time` set
to a integer of 6.

Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice using direct interpolation {value}, sequential
arguments ( {} ), and numeric arguments ( {0} ).

Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.

Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/

const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "summer";
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!(
        "My favorite season is {0}. The team scored {1} points. The event started at {2}. A touchdown is worth {3} points.",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );

    #[allow(unused_variables)]
    let favorite_beverage = "coconut water";
}
