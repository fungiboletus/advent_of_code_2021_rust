use std::collections::HashSet;

/**
 * Making a Graph in Rust is actually not trivial like in many other programming languages.
 *
 * I did use at first Rc<RefCell<Whatever>>, and it worked but it was very ugly.
 * It also had memory leaks.
 * Then I read https://github.com/nrc/r4cppp/blob/master/graphs/README.md
 *
 * So we are going to use PetGraph.
 *
 * An alternative is to use an array and link the nodes of the graphs using index in the array,
 * but it's a bit ugly too.
 */
use petgraph::graphmap::UnGraphMap;

fn is_string_only_uppercase(data: &str) -> bool {
    data.chars().all(|c| c.is_ascii_uppercase())
}

type Caves<'a> = UnGraphMap<&'a str, ()>;

fn parse_data(data: &str) -> Caves {
    let caves: UnGraphMap<&str, ()> = UnGraphMap::from_edges(data.lines().map(|line| {
        let mut raw_connection = line.split('-');
        let link_start = raw_connection.next().expect("No start link");
        let link_end = raw_connection.next().expect("No end link");
        return (link_start, link_end);
    }));

    // Print petgraph in dot format
    // println!("{:?}", petgraph::dot::Dot::new(&caves));
    return caves;
}

fn count_paths_deep<'a>(caves: &Caves<'a>, current: &'a str, visited: HashSet<&'a str>) -> usize {
    if current == "end" {
        return 1;
    }

    let is_large_cave = is_string_only_uppercase(current);
    if !is_large_cave && visited.contains(current) {
        return 0;
    }
    let mut new_visited = visited.clone();
    new_visited.insert(current);

    let mut count = 0;
    for connection in caves.neighbors(current) {
        count += count_paths_deep(caves, connection, new_visited.clone());
    }
    return count;
}

// I looked at a solution and the code is ugly, but who cares.
fn count_paths_deep_twice<'a>(
    caves: &Caves<'a>,
    current: &'a str,
    visited: HashSet<&'a str>,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut new_visited = visited;

    let is_large_cave = is_string_only_uppercase(current);
    if !is_large_cave {
        if new_visited.contains(current) {
            if current != "start" {
                let mut count = 0;
                for connection in caves.neighbors(current) {
                    count += count_paths_deep(caves, connection, new_visited.clone());
                }
                return count;
            }
            return 0;
        }
        new_visited.insert(current);
    }

    let mut count = 0;
    for connection in caves.neighbors(current) {
        count += count_paths_deep_twice(caves, connection, new_visited.clone());
    }
    return count;
}

pub fn day_12_part_1(data: &str) -> i64 {
    let caves = parse_data(data);
    return count_paths_deep(&caves, "start", HashSet::new()) as i64;
}

pub fn day_12_part_2(data: &str) -> i64 {
    let caves = parse_data(data);
    return count_paths_deep_twice(&caves, "start", HashSet::new()) as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const LARGER_EXAMPLE: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EVEN_LARGER_EXAMPLE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_day_12_part_1() {
        assert_eq!(day_12_part_1(SMALL_EXAMPLE), 10);
        assert_eq!(day_12_part_1(LARGER_EXAMPLE), 19);
        assert_eq!(day_12_part_1(EVEN_LARGER_EXAMPLE), 226);
    }

    #[test]
    fn test_day_12_part_2() {
        assert_eq!(day_12_part_2(SMALL_EXAMPLE), 36);
        assert_eq!(day_12_part_2(LARGER_EXAMPLE), 103);
        assert_eq!(day_12_part_2(EVEN_LARGER_EXAMPLE), 3509);
    }
}
