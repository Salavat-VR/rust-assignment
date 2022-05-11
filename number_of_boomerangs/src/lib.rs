use std::collections::HashMap;


pub type Distance = i32;

fn simple_distance(x: &[i32], y: &[i32]) -> i32 {
    // defining the simplified distance formula not to use square root
    (x[0] - y[0]) * (x[0] - y[0]) + (x[1] - y[1]) * (x[1] - y[1])
}

fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();

    // storing the distances between the points and number of points
    let mut buffer: HashMap<Distance, i32> = HashMap::new();
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            // not counting distance between itself
            if i == j {
                continue;
            }
            let x = &points[i];
            let y = &points[j];
            let simple_distance = simple_distance(x, y);

            // if distance hasn't already present we insert it and counting same distances
            let ea = buffer.entry(simple_distance).or_default();
            *ea += 1;
        }

        for &value in buffer.values() {
            if value > 1 {
                result += value * (value - 1);
            }
        }
        buffer.clear();
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::number_of_boomerangs;

    #[test]
    fn test_case_1() {
        let points: Vec<Vec<i32>> = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
        assert_eq!(number_of_boomerangs(points), 2);
    }

    #[test]
    fn test_case_2() {
        let points: Vec<Vec<i32>> = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(number_of_boomerangs(points), 2);
    }

    #[test]
    fn test_case_3() {
        let points: Vec<Vec<i32>> = vec![vec![1, 1]];
        assert_eq!(number_of_boomerangs(points), 0);
    }
}
