use webgl_matrix::Vector;
use std::f32;

// ----------------------------------------------------------------------------
// Based on http://www.flipcode.com/archives/Octree_Implementation.shtml
// ----------------------------------------------------------------------------

#[derive(PartialEq)]
enum Interaction {
    Disjoint,
    Overlap,
    Within,
    Contains,
}

struct Boundary {
    pub center: [f32; 3],
    pub h_side: f32,
}

impl Boundary {
    pub fn from_points(points: &[[f32; 3]]) -> Self {
        let min_v = [
            points
                .iter()
                .fold(std::f32::MAX, |acc, v| if acc < v[0] { acc } else { v[0] }),
            points
                .iter()
                .fold(std::f32::MAX, |acc, v| if acc < v[1] { acc } else { v[1] }),
            points
                .iter()
                .fold(std::f32::MAX, |acc, v| if acc < v[2] { acc } else { v[2] }),
        ];

        let max_v = [
            points
                .iter()
                .fold(std::f32::MIN, |acc, v| if acc > v[0] { acc } else { v[0] }),
            points
                .iter()
                .fold(std::f32::MIN, |acc, v| if acc > v[1] { acc } else { v[1] }),
            points
                .iter()
                .fold(std::f32::MIN, |acc, v| if acc > v[2] { acc } else { v[2] }),
        ];

        // The radius in each direction and its use to find the center
        let radius_v = max_v.sub(&min_v).scale(0.5);

        let center = min_v.add(&radius_v);

        // Find the greatest side
        let h_side = radius_v
            .iter()
            .fold(std::f32::MIN, |acc, v| if acc > *v { acc } else { *v });

        Self { center, h_side }
    }

    pub fn intersection(&self, other: &Self) -> Interaction {
        let dist = [
            (self.center[0] - other.center[0]).abs(),
            (self.center[1] - other.center[1]).abs(),
            (self.center[2] - other.center[2]).abs(),
        ];

        // Treat the two boundries as though they are cubes so if any compent
        // is greater than the sum of the half sides, they must be disjoint
        let side_sum = self.h_side + other.h_side;
        if dist[0] > side_sum || dist[1] > side_sum || dist[2] > side_sum {
            return Interaction::Disjoint;
        }

        let side_diff = (other.h_side - self.h_side).abs();
        if dist[0] <= side_diff && dist[1] <= side_diff && dist[2] <= side_diff {
            if other.h_side > self.h_side {
                return Interaction::Within;
            }

            return Interaction::Contains;
        }

        Interaction::Overlap
    }
}

struct Octnode {
    children: [Option<Box<Octnode>>; 8],
    boundary: Boundary,
    point_indicies: Vec<usize>,
}

impl Octnode {
    fn new(boundary: Boundary) -> Self {
        Self {
            children: [None, None, None, None, None, None, None, None],
            boundary,
            point_indicies: Vec::new(),
        }
    }

    fn octrant(&self, point: &[f32; 3]) -> usize {
        let diff_v = point.sub(&self.boundary.center);
        match (
            diff_v[0].is_sign_positive(),
            diff_v[1].is_sign_positive(),
            diff_v[2].is_sign_positive(),
        ) {
            (false, false, false) => 0,
            (true, false, false) => 1,
            (false, true, false) => 2,
            (true, true, false) => 3,
            (false, false, true) => 4,
            (true, false, true) => 5,
            (false, true, true) => 6,
            (true, true, true) => 7,
        }
    }

    fn build(
        &mut self,
        points_to_process: &[usize],
        points: &[[f32; 3]],
        threshold: usize,
        maximum_depth: usize,
        current_depth: usize,
    ) {
        // Are we a leaf?
        let count = points_to_process.len();
        if count <= threshold || current_depth >= maximum_depth {
            // This is a leaf, copy the points over
            self.point_indicies = points_to_process.to_vec();
            return;
        }

        let mut codes = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        // Classify each point to a child node (octrant) (IE spacial partition)
        for p in points_to_process {
            let code = self.octrant(&points[*p]);
            codes[code].push(*p);
        }

        // Create children for each spatial partition
        for i in 0..8 {
            // Blank spatial partitions are already None
            if codes[i].is_empty() {
                continue;
            }

            let offset_table = [
                [-0.5, -0.5, -0.5], // 0
                [0.5, -0.5, -0.5],  // 1
                [-0.5, 0.5, -0.5],  // 2
                [0.5, 0.5, -0.5],   // 3
                [-0.5, -0.5, 0.5],  // 4
                [0.5, -0.5, 0.5],   // 5
                [-0.5, 0.5, 0.5],   // 6
                [0.5, 0.5, 0.5],    // 7
            ];

            // Each sub space partition has half the radius of the current partition
            // and has its center offset given from the table above
            let offset = offset_table[i].scale(self.boundary.h_side);
            let new_bounds = Boundary {
                h_side: self.boundary.h_side * 0.5,
                center: self.boundary.center.add(&offset),
            };

            let mut new_node = Box::new(Self::new(new_bounds));

            new_node.build(
                &codes[i],
                points,
                threshold,
                maximum_depth,
                current_depth + 1,
            );

            self.children[i] = Some(new_node);
        }
    }

    fn neighbors(&self, point: &Boundary, result: &mut Vec<usize>) {
        if self.boundary.intersection(point) == Interaction::Disjoint {
            return;
        }

        for p in &self.point_indicies {
            result.push(*p);
        }

        for child in &self.children {
            match child {
                Some(c) => c.neighbors(point, result),
                _ => continue,
            }
        }
    }
}

pub struct Octree<'a> {
    root: Octnode,
    points: &'a [[f32; 3]],
}

impl<'a> Octree<'a> {
    pub fn new(points: &'a [[f32; 3]], threshold: usize, maximum_depth: usize) -> Self {
        let point_indicies: Vec<_> = (0..points.len()).collect();
        let boundary = Boundary::from_points(points);

        let mut root = Octnode::new(boundary);
        root.build(&point_indicies[..], points, threshold, maximum_depth, 0);

        Self { points, root }
    }

    pub fn neighbors(&self, point: [f32; 3], radius: f32) -> Vec<usize> {
        let mut result = Vec::<usize>::new();

        let boundary = Boundary {
            center: point,
            h_side: radius,
        };

        self.root.neighbors(&boundary, &mut result);

        result
    }

    pub fn neighbors_filtered(
        &self,
        point: [f32; 3],
        radius: f32,
    ) -> impl Iterator<Item = usize> + '_ {
        let result = self.neighbors(point, radius);

        result.into_iter().filter_map(move |p| {
            let dist = self.points[p].sub(&point);
            if dist.mag() < radius {
                Some(p)
            } else {
                None
            }
        })
    }

    #[allow(dead_code)]
    pub fn closest_neighbor (
        &self,
        point: [f32; 3],
        radius: f32,
    ) -> Option<usize> {
        let neighbors = self.neighbors(point, radius);

        neighbors.into_iter().min_by(move |x, y| {
            let x_dist = self.points[*x].sub(&point).mag2();
            let y_dist = self.points[*y].sub(&point).mag2();

            if x_dist < y_dist {
                std::cmp::Ordering::Less
            } else if y_dist < x_dist {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    }

    #[allow(dead_code)]
    pub fn interactions(&self, radius: f32) -> Vec<Vec<usize>> {
        let mut result = Vec::<Vec<usize>>::new();

        for i in 0..self.points.len() {
            let neighbors = self.neighbors_filtered(self.points[i], radius);
            let removed_self = neighbors.filter_map(move |j| if i != j { Some(j) } else { None });

            let mut remaining: Vec<usize> = removed_self.collect();
            remaining.sort();
            result.push(remaining);
        }

        result
    }

    pub fn interaction_pairs(&self, radius: f32) -> Vec<[usize; 2]> {
        let mut result = Vec::<[usize; 2]>::new();

        for i in self.points.into_iter().enumerate() {
            let neighbors = self.neighbors_filtered(*i.1, radius);

            for j in neighbors {
                if i.0 < j {
                    result.push([i.0, j]);
                }
            }
        }

        result
    }
}

#[test]
fn test_boundry() {
    let benzene = vec![
        [0.809634648101, -1.147538780983, 0.014972631566],
        [1.398699588466, 0.127328782791, -0.002144982085],
        [0.589064935327, 1.274867581386, -0.017117194372],
        [-0.809634662319, 1.147538798992, -0.014971364596],
        [-1.398699586500, -0.127328768061, 0.002144873987],
        [-0.589064915101, -1.274867578697, 0.017119057137],
        [1.440394137801, -2.041547849809, 0.026629383195],
        [2.488379670712, 0.226526007921, -0.003823876212],
        [1.047985553449, 2.268074248434, -0.030457985645],
        [-1.440393936910, 2.041547849715, -0.026640881090],
        [-2.488379658318, -0.226526421675, 0.003809439886],
        [-1.047985861679, -2.268074256425, 0.030447941745],
    ];

    let boundary = Boundary::from_points(&benzene);
    assert!(boundary.center[0].abs() < 1e-3);
    assert!(boundary.center[1].abs() < 1e-3);
    assert!(boundary.center[2].abs() < 1e-3);

    let expected_size = (2.488379670712 + 2.488379658318) / 2.;
    assert!((boundary.h_side - expected_size).abs() < 1e-3);

    let disjoint = Boundary {
        center: [0.0, 0.0, 10.0],
        h_side: 1.0,
    };
    assert!(boundary.intersection(&disjoint) == Interaction::Disjoint);
    assert!(disjoint.intersection(&boundary) == Interaction::Disjoint);

    let overlap = Boundary {
        center: [0.0, 0.0, 10.0],
        h_side: 10.0,
    };
    assert!(boundary.intersection(&overlap) == Interaction::Overlap);
    assert!(overlap.intersection(&boundary) == Interaction::Overlap);

    let within = Boundary {
        center: [0.0, 0.0, 0.0],
        h_side: 0.5,
    };
    assert!(boundary.intersection(&within) == Interaction::Contains);
    assert!(within.intersection(&boundary) == Interaction::Within);

    let contains = Boundary {
        center: [0.0, 0.0, 10.0],
        h_side: 1000.0,
    };
    assert!(boundary.intersection(&contains) == Interaction::Within);
    assert!(contains.intersection(&boundary) == Interaction::Contains);
}

#[test]
fn test_octree() {
    let benzene = vec![
        [0.809634648101, -1.147538780983, 0.014972631566],
        [1.398699588466, 0.127328782791, -0.002144982085],
        [0.589064935327, 1.274867581386, -0.017117194372],
        [-0.809634662319, 1.147538798992, -0.014971364596],
        [-1.398699586500, -0.127328768061, 0.002144873987],
        [-0.589064915101, -1.274867578697, 0.017119057137],
        [1.440394137801, -2.041547849809, 0.026629383195],
        [2.488379670712, 0.226526007921, -0.003823876212],
        [1.047985553449, 2.268074248434, -0.030457985645],
        [-1.440393936910, 2.041547849715, -0.026640881090],
        [-2.488379658318, -0.226526421675, 0.003809439886],
        [-1.047985861679, -2.268074256425, 0.030447941745],
    ];

    let octree = Octree::new(benzene.as_slice(), 1, 6);

    assert_eq!(octree.neighbors_filtered([0., 0., 0.], 1000.).count(), 12);
    assert_eq!(octree.neighbors_filtered([0., 0., 0.], 1.).count(), 0);
    assert_eq!(octree.neighbors_filtered([1.1, -1.6, 0.0], 1.).count(), 2);

    let bonds = octree.interactions(2.);
    assert_eq!(bonds.len(), 12);
    assert_eq!(bonds[0], [1, 5, 6]);
    assert_eq!(bonds[1], [0, 2, 7]);
    assert_eq!(bonds[2], [1, 3, 8]);
    assert_eq!(bonds[3], [2, 4, 9]);
    assert_eq!(bonds[4], [3, 5, 10]);
    assert_eq!(bonds[5], [0, 4, 11]);
    assert_eq!(bonds[6], [0]);
    assert_eq!(bonds[7], [1]);
    assert_eq!(bonds[8], [2]);
    assert_eq!(bonds[9], [3]);
    assert_eq!(bonds[10], [4]);
    assert_eq!(bonds[11], [5]);

    let bonds2 = octree.interaction_pairs(2.);
    assert_eq!(bonds2.len(), 12);
    assert_eq!(bonds2[0], [0, 1]);
    assert_eq!(bonds2[1], [0, 5]);
    assert_eq!(bonds2[2], [0, 6]);
    assert_eq!(bonds2[3], [1, 7]);
    assert_eq!(bonds2[4], [1, 2]);
    assert_eq!(bonds2[5], [2, 3]);
    assert_eq!(bonds2[6], [2, 8]);
    assert_eq!(bonds2[7], [3, 9]);
    assert_eq!(bonds2[8], [3, 4]);
    assert_eq!(bonds2[9], [4, 5]);
    assert_eq!(bonds2[10], [4, 10]);
    assert_eq!(bonds2[11], [5, 11]);

    let closest1 = octree.closest_neighbor([0.81, -1.15, 0.015], 2.);
    assert_eq!(closest1.unwrap(), 0);
}
