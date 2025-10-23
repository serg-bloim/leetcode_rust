use std::collections::HashSet;
use std::hash::Hash;

trait SetExtrasCopy<T> {
    fn pop_any(&mut self) -> Option<T>;
}
impl<T> SetExtrasCopy<T> for HashSet<T>
where
    T: Eq + Hash + Copy,
{
    fn pop_any(&mut self) -> Option<T> {
        if let Some(&v) = self.iter().next() {
            return self.take(&v);
        }
        None
    }
}

#[derive(Debug)]
struct Bomb {
    x: i32,
    y: i32,
    radius: i32,
}
impl Bomb {
    fn reaches(&self, other: &Bomb) -> bool {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        // println!("node: {:?} can reach to {:?}: {}", self, other, res);
        (self.radius as i64).pow(2) >= dx.pow(2) + dy.pow(2)
    }
}
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let bombs: Vec<_> = bombs
            .into_iter()
            .map(|v| match *v.as_slice() {
                [x, y, radius] => Bomb {
                    x,
                    y,
                    radius,
                },
                _ => panic!("wrong input {:?}", v),
            })
            .collect();
        let mut starting_candidates: HashSet<_> = (0..bombs.len()).collect();
        let mut max_size = 0;
        loop {
            if let Some(starting_node_i) = starting_candidates.pop_any() {
                let mut not_yet_visited: HashSet<_> = (0..bombs.len()).collect();
                let size_before = not_yet_visited.len();
                not_yet_visited.remove(&starting_node_i);
                let mut frontier = vec![starting_node_i];
                while let Some(next_node_i) = frontier.pop() {
                    let next_node = bombs.get(next_node_i).unwrap();
                    not_yet_visited
                        .extract_if(|&node_i| {
                            let node = bombs.get(node_i).unwrap();
                            next_node.reaches(node)
                        })
                        .for_each(|x| {
                            frontier.push(x);
                            starting_candidates.remove(&x);
                        });
                }
                let size = size_before - not_yet_visited.len();
                if size > max_size {
                    max_size = size;
                }
            } else {
                return max_size as i32;
            }
        }
    }
}

struct Solution;
fn main() {}

macro_rules! vec2 {
    // Match a list of lists
    [ $( [ $( $x:expr ),* ] ),* $(,)? ] => {
        vec![
            $(
                vec![$($x),*]
            ),*
        ]
    };
}

#[test]
fn test1() {
    let inp = vec2![[2, 1, 3], [6, 1, 4]];
    let actual = Solution::maximum_detonation(inp);
    assert_eq!(actual, 2);
}

#[test]
fn test2() {
    let inp = vec2![[1,1,5],[10,10,5]];
    let actual = Solution::maximum_detonation(inp);
    assert_eq!(actual, 1);
}
#[test]
fn test3() {
    let inp = vec2![[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]];
    let actual = Solution::maximum_detonation(inp);
    assert_eq!(actual, 5);
}

#[test]
fn test4() {
    let inp = vec2![[1,1,100000],[100000,100000,1]];
    let actual = Solution::maximum_detonation(inp);
    assert_eq!(actual, 1);
}

#[test]
fn test5() {
        let inp = vec2![[855,82,158],[17,719,430],[90,756,164],[376,17,340],[691,636,152],[565,776,5],[464,154,271],[53,361,162],[278,609,82],[202,927,219],[542,865,377],[330,402,270],[720,199,10],[986,697,443],[471,296,69],[393,81,404],[127,405,177]];
    let actual = Solution::maximum_detonation(inp);
    assert_eq!(actual, 9);
}
