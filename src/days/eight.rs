use std::cmp;
pub fn parse(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap() as i32,
                    visible: false,
                    score: 1,
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy)]
pub struct Tree {
    height: i32,
    visible: bool,
    score: i32,
}

fn update_line<'a, I>(line: I)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut tallest_tree = -1;
    for tree in line {
        tree.visible = tree.height > tallest_tree || tree.visible;
        tallest_tree = cmp::max(tallest_tree, tree.height);
    }
}

pub fn solution_easy(input: &str) -> i32 {
    let mut data = parse(input);
    for line in &mut data {
        update_line(line.iter_mut());
        update_line(line.iter_mut().rev());
    }
    let mut data = matrix_transpose(data);
    for line in &mut data {
        update_line(line.iter_mut());
        update_line(line.iter_mut().rev());
    }
    data.concat().iter().filter(|t| t.visible).count() as i32
}

fn update_line_score<'a, I>(line: I)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut seen_heights = vec![];
    for tree in line {
        let mut score = 0;
        for &prev in seen_heights.iter().rev() {
            if prev < tree.height {
                score += 1;
            } else {
                score += 1;
                break;
            }
        }
        seen_heights.push(tree.height);
        tree.score *= score;
    }
}

pub fn solution_hard(input: &str) -> i32 {
    let mut data = parse(input);
    for line in &mut data {
        update_line_score(line.iter_mut());
        update_line_score(line.iter_mut().rev());
    }

    let mut data = matrix_transpose(data);
    for line in &mut data {
        update_line_score(line.iter_mut());
        update_line_score(line.iter_mut().rev());
    }

    data.concat().iter().map(|t| t.score).max().unwrap()
}

fn matrix_transpose(m: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let test_input = fs::read_to_string("input/test").expect("File not found.");
        assert_eq!(solution_easy(&test_input), 21);
        assert_eq!(solution_hard(&test_input), 8);
    }
}
