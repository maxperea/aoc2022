use crate::days::tools::*;

pub fn solution_easy(input: &str) -> i32 {
    let mut forest = parse(input);
    for treeline in &mut forest {
        update_visibility(treeline.iter_mut());
        update_visibility(treeline.iter_mut().rev());
    }
    let mut forest = matrix_transpose(forest);
    for treeline in &mut forest {
        update_visibility(treeline.iter_mut());
        update_visibility(treeline.iter_mut().rev());
    }
    forest.concat().iter().filter(|t| t.visible).count() as i32
}

pub fn solution_hard(input: &str) -> i32 {
    let mut forest = parse(input);
    for treeline in &mut forest {
        update_score(treeline.iter_mut());
        update_score(treeline.iter_mut().rev());
    }

    let mut forest = matrix_transpose(forest);
    for treeline in &mut forest {
        update_score(treeline.iter_mut());
        update_score(treeline.iter_mut().rev());
    }

    forest.concat().iter().map(|t| t.score).max().unwrap()
}

#[derive(Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool,
    score: i32,
}

fn parse(input: &str) -> Vec<Vec<Tree>> {
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

fn update_visibility<'a, I>(line: I)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut tallest_tree = -1;
    for tree in line {
        tree.visible = tree.height > tallest_tree || tree.visible;
        tallest_tree = std::cmp::max(tallest_tree, tree.height);
    }
}

fn update_score<'a, I>(line: I)
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
