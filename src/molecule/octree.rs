use webgl_matrix::Vector;

// ----------------------------------------------------------------------------
// Based on http://www.flipcode.com/archives/Octree_Implementation.shtml
// ----------------------------------------------------------------------------

#[allow(dead_code)]
enum Interaction {
    Disjoint,
    Overlap,
    Within,
    Contains,
}

#[allow(dead_code)]
struct Boundary {
    pub center: [f32; 3],
    pub h_side: f32,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct Octnode {
    children: [Option<Box<Octnode>>; 8],
    boundary: Boundary,
    point_indicies: Vec<usize>,
}

#[allow(dead_code)]
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

    fn neighbors(&self, point: &[f32; 3]) -> Option<&Vec<usize>> {
        if !self.point_indicies.is_empty() {
            return Some(&self.point_indicies);
        }

        let code = self.octrant(point);

        if let Some(child) = &self.children[code] {
            child.neighbors(point)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub struct Octree {
    root: Octnode,
}

#[allow(dead_code)]
impl Octree {
    pub fn new(points: &[[f32; 3]], threshold: usize, maximum_depth: usize) -> Self {
        let point_indicies: Vec<_> = (0..points.len()).collect();
        let boundary = Boundary::from_points(points);

        let mut root = Octnode::new(boundary);
        root.build(&point_indicies[..], points, threshold, maximum_depth, 0);

        Self { root }
    }

    pub fn neighbors(&self, point: &[f32; 3]) -> Option<&Vec<usize>> {
        self.root.neighbors(point)
    }
}
