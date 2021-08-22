use regex::Regex;

#[path = "../src/types/tileset.rs"]
mod tileset_json;

fn main() {
  let regex = Regex::new(r"range (\d+) to (\d+)").unwrap();
  let string = "range 1 to 63";
  // result will be a tuple containing the start and end indices for the first match in the string
  let result = regex.captures(string).unwrap();

  println!("{}", result.get(1).unwrap().as_str());
}
