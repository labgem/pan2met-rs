use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read lines from a file
/// The output is wrapped in a Result to allow matching on errors.
/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Read rows of a file and put them into an HashSet
/// Returns a HashSet with each row of the file
pub fn read_set<P>(filename: P) -> io::Result<HashSet<String>>
where
    P: AsRef<Path>,
{
    let mut set: HashSet<String> = HashSet::new();
    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        set.insert(line.clone());
    }
    Ok(set)
}

/// Read a TSV mapping file
/// the row is tab seperated values
/// the first column is the key
/// the second column is the values, comma separated
pub fn read_mapping<P>(filename: P) -> io::Result<HashMap<String, Vec<String>>>
where
    P: AsRef<Path>,
{
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    let lines = read_lines(filename)?;
    for row in lines.map_while(Result::ok) {
        let mut parts = row.split("\t");
        if let Some(key) = parts.next() {
            if let Some(values) = parts.next() {
                for value in values.split(",") {
                    if let Some(mapping_values) = mapping.get_mut(key) {
                        mapping_values.push(value.to_owned());
                    } else {
                        mapping.insert(key.to_owned(), vec![value.to_owned()]);
                    }
                }
            }
        }
    }
    Ok(mapping)
}

pub fn reverse_mapping(mapping: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut reversed: HashMap<String, Vec<String>> = HashMap::new();
    for (key, values) in mapping {
        for reverse_key in values {
            if let Some(reversed_values) = reversed.get_mut(reverse_key) {
                reversed_values.push(key.to_owned());
            } else {
                reversed.insert(reverse_key.to_owned(), vec![key.to_owned()]);
            }
        }
    }
    reversed
}
