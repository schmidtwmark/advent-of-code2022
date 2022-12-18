use std::{collections::VecDeque, default, str::FromStr};

use aoc::Solver;
use hashbrown::HashSet;
use itertools::Itertools;
use log::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y, z })
    }
}
impl Cube {
    // Return the 6 neighbors of a cube
    fn get_neighbors(&self) -> impl Iterator<Item = Cube> + Clone + '_ {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(move |(dx, dy, dz)| Cube {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }
}

fn count_surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes.iter().fold(0, |mut acc, cube| {
        let present_neighbors = cube.get_neighbors().filter(|c| cubes.contains(c));

        acc += 6 - present_neighbors.count();
        acc
    })
}
fn count_surface_area_outer_only(cubes: &HashSet<Cube>, surrounding_air: &HashSet<Cube>) -> usize {
    cubes.iter().fold(0, |mut acc, cube| {
        let air = cube.get_neighbors().filter(|c| surrounding_air.contains(c));

        acc += air.count();
        acc
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DimensionLimit<T> {
    min: T,
    max: T,
}

impl<T> DimensionLimit<T> {
    fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

fn get_dimension_limits(cubes: &HashSet<Cube>) -> [DimensionLimit<i32>; 3] {
    let mut x = DimensionLimit::new(i32::MAX, i32::MIN);
    let mut y = DimensionLimit::new(i32::MAX, i32::MIN);
    let mut z = DimensionLimit::new(i32::MAX, i32::MIN);

    for cube in cubes {
        x.min = x.min.min(cube.x);
        x.max = x.max.max(cube.x);
        y.min = y.min.min(cube.y);
        y.max = y.max.max(cube.y);
        z.min = z.min.min(cube.z);
        z.max = z.max.max(cube.z);
    }
    x.min -= 1;
    x.max += 1;
    y.min -= 1;
    y.max += 1;
    z.min -= 1;
    z.max += 1;

    [x, y, z]
}

fn get_connected_components(cubes: &HashSet<Cube>) -> Vec<HashSet<Cube>> {
    let mut components = Vec::new();
    let mut remaining = cubes.clone();

    while !remaining.is_empty() {
        let mut component = HashSet::new();
        let mut to_visit = vec![*remaining.iter().next().unwrap()];
        while let Some(cube) = to_visit.pop() {
            if !remaining.contains(&cube) {
                continue;
            }
            component.insert(cube);
            remaining.remove(&cube);
            to_visit.extend(cube.get_neighbors().filter(|c| remaining.contains(c)));
        }
        components.push(component);
    }

    components
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CubeType {
    Air,
    TrappedAir,
    Lava,
}

fn in_limit(cube: Cube, limits: &[DimensionLimit<i32>; 3]) -> bool {
    cube.x >= limits[0].min
        && cube.x <= limits[0].max
        && cube.y >= limits[1].min
        && cube.y <= limits[1].max
        && cube.z >= limits[2].min
        && cube.z <= limits[2].max
}

fn get_neighbor_towards_direction(
    direction: (i32, i32, i32),
    cube: Cube,
    cubes: &HashSet<Cube>,
    air_pockets: &HashSet<Cube>,
    limits: &[DimensionLimit<i32>; 3],
) -> (CubeType, Option<Cube>) {
    let mut current = cube;
    loop {
        current.x += direction.0;
        current.y += direction.1;
        current.z += direction.2;
        if !in_limit(current, limits) {
            return (CubeType::Air, None);
        }

        if cubes.contains(&current) {
            return (CubeType::Lava, Some(current));
        }

        if air_pockets.contains(&current) {
            return (CubeType::TrappedAir, Some(current));
        }
    }
}
fn get_surrounding_air(
    lava_cubes: &HashSet<Cube>,
    limits: &[DimensionLimit<i32>; 3],
) -> HashSet<Cube> {
    let [x_dims, y_dims, z_dims] = limits;
    let start = Cube {
        x: x_dims.min,
        y: y_dims.min,
        z: z_dims.min,
    };

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(cube) = queue.pop_front() {
        let neighbors = cube.get_neighbors().filter(|c| in_limit(*c, limits));
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }

            if lava_cubes.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    visited
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let cubes: HashSet<Cube> = lines
            .iter()
            .map(|line| line.parse::<Cube>().unwrap())
            .collect();
        debug!("{:?}", cubes);
        for (idx, component) in get_connected_components(&cubes).iter().enumerate() {
            debug!("Found component {idx} {:?}", component);
        }

        count_surface_area(&cubes)
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let cubes: HashSet<Cube> = lines
            .iter()
            .map(|line| line.parse::<Cube>().unwrap())
            .collect();

        let [x_dims, y_dims, z_dims] = get_dimension_limits(&cubes);
        debug!("{:?} {:?} {:?}", x_dims, y_dims, z_dims);

        // let air_pockets = get_air_pockets(&cubes, &[x_dims, y_dims, z_dims]);
        // info!("Found {} air pockets", air_pockets.len());

        let surrounding_air = get_surrounding_air(&cubes, &[x_dims, y_dims, z_dims]);

        count_surface_area_outer_only(&cubes, &surrounding_air)

        // let mut air_pockets = HashSet::<Cube>::new();

        // // For each position, if it is not present in the cubes, check all six directions
        // // If all 6 touch a cube, or it touches an air pocket, it is an air pocket
        // // If it exceeds the dimension limits, it is NOT an air pocket

        // for x in x_dims.min..=x_dims.max {
        //     for y in y_dims.min..=y_dims.max {
        //         for z in z_dims.min..=z_dims.max {
        //             let directions = [
        //                 (1, 0, 0),
        //                 (-1, 0, 0),
        //                 (0, 1, 0),
        //                 (0, -1, 0),
        //                 (0, 0, 1),
        //                 (0, 0, -1),
        //             ];

        //             let current = Cube { x, y, z };
        //             if cubes.contains(&current) {
        //                 continue;
        //             }

        //             let neighbors = directions
        //                 .iter()
        //                 .map(|direction| {
        //                     get_neighbor_towards_direction(
        //                         *direction,
        //                         current,
        //                         &cubes,
        //                         &air_pockets,
        //                         &[x_dims, y_dims, z_dims],
        //                     )
        //                 })
        //                 .collect_vec();
        //             if neighbors.iter().any(|(t, _)| *t == CubeType::Air) {
        //                 continue;
        //             }

        //             air_pockets.insert(current);
        //         }
        //     }
        // }

        // let combined: HashSet<Cube> = cubes.union(&air_pockets).copied().collect();
        // assert_eq!(combined.len(), cubes.len() + air_pockets.len());

        // count_surface_area(&combined)
    }
}

fn main() {
    let sample = include_str!("../../samples/18.txt");
    let sample_2 = include_str!("../../samples/18_2.txt");
    let input = include_str!("../../inputs/18.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 64),
        aoc::Input::new_sample(sample_2, 10),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 58),
        aoc::Input::new_final(input),
        // 2002 is too low
        // 2004 is too low
        // 2006 is too low
        // Too low means overcounting air pockets?
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
