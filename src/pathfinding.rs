use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x, y: self.y + 1 },
        ]
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    point: Point,
    f_score: i32,
    g_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: we use reverse ordering because BinaryHeap is a max-heap
        // and we want to prioritize lower f_scores
        other.f_score.cmp(&self.f_score)
    }
}


impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_path(
    start: Point,
    goal: Point,
    is_walkable: impl Fn(Point) -> bool,
) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_scores: HashMap<Point, i32> = HashMap::new();

    g_scores.insert(start, 0);
    open_set.push(Node {
        point: start,
        f_score: start.manhattan_distance(&goal),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.point == goal {
            return Some(reconstruct_path(&came_from, current.point));
        }

        for neighbor in current.point.neighbors() {
            if !is_walkable(neighbor) {
                continue;
            }

            let tentative_g_score = g_scores[&current.point] + 1;

            if !g_scores.contains_key(&neighbor) ||
                tentative_g_score < g_scores[&neighbor] {
                came_from.insert(neighbor, current.point);
                g_scores.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    point: neighbor,
                    f_score: tentative_g_score,
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
    let mut path = vec![current];

    while let Some(&next) = came_from.get(&current) {
        path.push(next);
        current = next;
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let start = Point { x: 0, y: 0 };
        let goal = Point { x: 2, y: 2 };

        let is_walkable = |_| true;

        let path = find_path(start, goal, is_walkable).unwrap();

        assert_eq!(path.len(), 5);
    }
}