use crate::bounding_box::BoundingBox;
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
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
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

            if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {
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

pub fn bounding_box_for_path(origin: &Point, path: Option<&[Point]>) -> BoundingBox {
    let mut left = origin.x;
    let mut right = origin.x + 1;
    let mut top = origin.y;
    let mut bottom = origin.y + 1;

    if let Some(points) = path {
        for point in points {
            if point.x < left {
                left = point.x;
            }
            if point.x + 1 > right {
                right = point.x + 1;
            }
            if point.y < top {
                top = point.y;
            }
            if point.y + 1 > bottom {
                bottom = point.y + 1;
            }
        }
    }

    BoundingBox {
        left,
        right,
        top,
        bottom,
    }
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

    #[test]
    fn test_bounding_box_for_path_with_none() {
        let origin = Point { x: 5, y: 5 };
        let bbox = bounding_box_for_path(&origin, None);

        // With no path provided, the bounding box should exactly cover the origin cell.
        assert_eq!(bbox.left, 5);
        assert_eq!(bbox.right, 6);
        assert_eq!(bbox.top, 5);
        assert_eq!(bbox.bottom, 6);
    }

    #[test]
    fn test_bounding_box_for_path_with_points() {
        let origin = Point { x: 2, y: 3 };
        let path = vec![
            Point { x: 1, y: 4 },
            Point { x: 5, y: 2 },
            Point { x: 3, y: 6 },
        ];

        let bbox = bounding_box_for_path(&origin, Some(&path));

        // Calculate expected boundaries:
        // left   = min(origin.x, min(x in path))       = min(2, 1)    = 1
        // right  = max(origin.x+1, max(x+1 in path))   = max(3, 6)    = 6
        // top    = min(origin.y, min(y in path))       = min(3, 2)    = 2
        // bottom = max(origin.y+1, max(y+1 in path))   = max(4, 7)    = 7

        assert_eq!(bbox.left, 1);
        assert_eq!(bbox.right, 6);
        assert_eq!(bbox.top, 2);
        assert_eq!(bbox.bottom, 7);
    }
}
