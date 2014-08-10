extern crate levenshtein;

use levenshtein::levenshtein_dist;

fn main() {
	let str1 = "soylent green is people";
	let str2 = "people soiled our green";
	let val = levenshtein_dist(str1, str2);
	println!("String #1: {}\nString #2: {}\nDistance is: {}", str1, str2, val);
}
