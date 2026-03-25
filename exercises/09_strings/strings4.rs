// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// here is a rule by claude:
// - string literals are always &str; 1
// - &Something or slicing gives us &str; 7
// - methods that trim/scan without transforming return a view; 8
// - anything that constructs or converts gives us String; the rest
// basically:
//   if the result can point back into existing memory without copying, it's &str.
//   if it needs its own heap allocation, it's String.
// question yourself:
//   does the expression produced an owned value (thus String) or a borrowed view (thus &str)?

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // 1; &'static str, baked into the binary

    string("red".to_string()); // 2; explicit conversion

    string(String::from("hi")); // 3; constructor

    string("rust is fun!".to_owned()); // 4; "give me an owned copy"

    string("nice weather".into()); // 5; type-inferred conversion

    string(format!("Interpolation {}", "Station")); // 6; builds a new String at runtime

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // 7; & + slice of a String

    string_slice("  hello there ".trim()); // 8; no new allocation, just adjusts the pointer/length

    string("Happy Monday!".replace("Mon", "Tues")); // 9; produces a new String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 10; produces a new String
}
