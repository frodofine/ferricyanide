use std::error::Error;

pub mod element;
pub use element::Element;

pub mod octree;

pub struct Atom<'a> {
    pub position: &'a[f32; 3],
    pub element: &'a Element,
}

pub struct Bond<'a> {
    pub atom_1: Atom<'a>,
    pub atom_2: Atom<'a>,
}

pub struct Molecule {
    positions: Vec<[f32; 3]>,
    elements: Vec<Element>,
    bonds: Vec<[usize; 2]>,
    pub name: String,
}

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

pub fn read_xyz(file: &str) -> Result<Molecule, Box<dyn Error>> {
    use webgl_matrix::Vector;

    let mut positions = Vec::<[f32; 3]>::new(); 
    let mut elements = Vec::<Element>::new();

    let mut lines = file.lines();

    let total_lines: usize = unwrap_abort(lines.next()).trim_end().parse().unwrap();
    positions.reserve_exact(total_lines);
    elements.reserve_exact(total_lines);

    let name = unwrap_abort(lines.next()).to_owned();

    for line in lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        /*
        if line_split.len() < 4 {
            return Error(std::error::Error::new("Invalid line"))
        }*/
        let element = unwrap_abort(line_split.get(0)).to_owned();
        let x: f32 = unwrap_abort(line_split.get(1)).parse()?;
        let y: f32 = unwrap_abort(line_split.get(2)).parse()?;
        let z: f32 = unwrap_abort(line_split.get(3)).parse()?;

        positions.push([x, y, z]);
        elements.push(Element::from(element));

        if total_lines == positions.len() {
            break;
        }
    }

    let mut bonds = Vec::<[usize; 2]>::new();

    let octree = octree::Octree::new(&positions, 100, 5);
    let interactions = octree.interaction_pairs(2.);

    for i in interactions {
        let atom_1 = unwrap_abort(positions.get(i[0]));
        let atom_2 = unwrap_abort(positions.get(i[1]));
        let dist_vec = atom_1.sub(atom_2);
        if dist_vec.mag() < 2.0 {
            bonds.push([i[0], i[1]]);
        }
    }

    Ok(Molecule { positions, elements, bonds, name })
}

#[derive(Debug)]
pub struct UnsupportedFormat {
    format: String,
}

impl std::fmt::Display for UnsupportedFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported format {}", self.format)
    }
}

impl std::error::Error for UnsupportedFormat {}

impl Molecule {

    pub fn atom(&self, id: usize) -> Atom {
        Atom {
            position: &self.positions[id],
            element: &self.elements[id],
        }        
    }

    pub fn atoms(&self) -> Box< dyn Iterator<Item=Atom> + '_> {
        Box::new(
            (0..self.positions.len()).map( move |i| {
                Atom{
                    position: &self.positions[i],
                    element: &self.elements[i],
                }
            })
        )
    }

    pub fn bonds(&self) -> Box< dyn Iterator<Item=Bond> + '_> {
        Box::new(
            self.bonds.iter().map( move |&x| {
                Bond {
                    atom_1: self.atom(x[0]),
                    atom_2: self.atom(x[1]),
                }
            })
        )
    }

    pub fn center(&self) -> [f32; 3] {
        #![allow(clippy::cast_precision_loss)]
        use webgl_matrix::Vector;

        self.positions
            .iter()
            .fold([0.0, 0.0, 0.0], |acc, x| {
                [
                    acc[0] + x[0],
                    acc[1] + x[1],
                    acc[2] + x[2],
                ]
            })
            .scale(1.0 / self.positions.len() as f32)
    }

    pub fn from_string_with_format(contents: &str, format: &str) -> Result<Self, Box<dyn Error>> {
        match format {
            "xyz" => read_xyz(contents),
            _ => Err(Box::new(UnsupportedFormat {
                format: format.to_owned(),
            })),
        }
    }
}
