use ndarray::Array2;
use std::collections::BinaryHeap;

fn parse_data(data: &str) -> Array2<u8> {
    let data: Vec<Vec<u8>> = data
        .lines()
        .into_iter()
        .map(|x| {
            x.as_bytes()
                .iter()
                .map(|&x| u8::from(x - 48))
                .collect::<Vec<u8>>()
        })
        .collect();

    let shape = (data.len(), data[0].len());
    let flat_data = data.into_iter().flatten().collect::<Vec<u8>>();

    return Array2::from_shape_vec(shape, flat_data).expect("Could not create array from data");
}

pub fn day_9_part_1(data: &str) -> i64 {
    let data = parse_data(data);
    let dim = data.dim();

    return data
        .indexed_iter()
        .map(|((i, j), location)| {
            // If a neigbour location is higher
            if (i > 0 && data[(i - 1, j)] <= *location)
                || (i < dim.0 - 1 && data[(i + 1, j)] <= *location)
                || (j > 0 && data[(i, j - 1)] <= *location)
                || (j < dim.1 - 1 && data[(i, j + 1)] <= *location)
            {
                return 0;
            }
            // Return current location + 1 because reasons
            return *location as i64 + 1;
        })
        .sum();
}

pub fn day_9_part_2(data: &str) -> i64 {
    let data = parse_data(data);
    let dim = data.dim();
    let mut basins: Array2<usize> = Array2::zeros(dim);

    // I could use a recursive function but I want to play with heap data structures now
    // to avoid stack overflows during the next days.
    let mut heap: BinaryHeap<(usize, (usize, usize))> = BinaryHeap::new();

    // Make sure that we visit all the locations by adding them in the heap
    for ((i, j), location) in data.indexed_iter() {
        // The 9 are some kind of walls that we ignore
        if *location != 9 {
            heap.push((0, (i, j)));
        }
    }

    // Create a map with the size of the maximum number of basins
    // Basin identifiers start from 1, so we ignore the index 0
    let mut basins_counters: Vec<usize> = vec![0; dim.0 * dim.1 + 1];

    let mut nb_basins = 0;

    // Iterate until the heap is not empty
    while let Some((basin_id, (i, j))) = heap.pop() {
        // If the location is already part of a basin we ignore it
        if basins[(i, j)] != 0 {
            continue;
        }

        let mut current_basin_id = basin_id;
        if basin_id == 0 {
            // We found a new basin
            nb_basins += 1;
            current_basin_id = nb_basins;
        }

        // Debugging with println is sometimes nicer than the debugger
        //println!("{:?}", ((i, j), basin_id, new_basin_id));
        //println!("{:?}", basins);
        //println!("----");

        // Mark the location as part of a basin
        basins[(i, j)] = current_basin_id;

        // Increment the counter for the basin
        basins_counters[current_basin_id] += 1;

        // Check the neighbours
        if i > 0 && data[(i - 1, j)] != 9 && basins[(i - 1, j)] == 0 {
            heap.push((current_basin_id, (i - 1, j)));
        }
        if i < dim.0 - 1 && data[(i + 1, j)] != 9 && basins[(i + 1, j)] == 0 {
            heap.push((current_basin_id, (i + 1, j)));
        }
        if j > 0 && data[(i, j - 1)] != 9 && basins[(i, j - 1)] == 0 {
            heap.push((current_basin_id, (i, j - 1)));
        }
        if j < dim.1 - 1 && data[(i, j + 1)] != 9 && basins[(i, j + 1)] == 0 {
            heap.push((current_basin_id, (i, j + 1)));
        }
    }

    // Get the 3 highest basins counters
    let mut non_empty_basins_counters = basins_counters
        .iter()
        .filter(|x| **x > 0)
        .map(|x| *x as i64)
        .collect::<Vec<i64>>();
    non_empty_basins_counters.sort_unstable();
    return non_empty_basins_counters
        .iter()
        .rev()
        .take(3)
        .product::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_day_9_part_1() {
        assert_eq!(day_9_part_1(EXAMPLE), 15);
    }

    #[test]
    fn test_day_9_part_2() {
        assert_eq!(day_9_part_2(EXAMPLE), 1134);
    }
}
