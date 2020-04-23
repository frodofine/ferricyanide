use std::error::Error;
use std::rc::Rc;

pub mod element;
pub use element::Element;

pub struct Atom {
    pub position: [f32; 3],
    pub element: Element,
}

pub struct Molecule {
    pub atoms: Vec<Rc<Atom>>,
    pub bonds: Vec<[Rc<Atom>; 2]>,
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

    let mut atoms = Vec::<Rc<Atom>>::new();
    let mut name = String::new();

    let mut total_lines: usize = 0;

    for (line_count, line) in file.lines().enumerate() {
        match line_count {
            0 => total_lines = line.trim_end().parse().unwrap(),
            1 => name = line.to_owned(),
            _ => {
                let line_split: Vec<&str> = line.split_whitespace().collect();
                /*
                if line_split.len() < 4 {
                    return Error(std::error::Error::new("Invalid line"))
                }*/
                let element = unwrap_abort(line_split.get(0)).to_owned();
                let x: f32 = unwrap_abort(line_split.get(1)).parse()?;
                let y: f32 = unwrap_abort(line_split.get(2)).parse()?;
                let z: f32 = unwrap_abort(line_split.get(3)).parse()?;

                atoms.push( Rc::new(Atom {
                    position: [x, y, z],
                    element: Element::from(element),
                }))
            }
        }

        if total_lines == atoms.len() {
            break;
        }
    }

    let mut bonds = Vec::<[Rc<Atom>; 2]>::new();

    for i in 0..atoms.len() {
        for j in (i+1)..atoms.len() {
            let atom_1 = unwrap_abort(atoms.get(i));
            let atom_2 = unwrap_abort(atoms.get(j));
            let dist_vec = atom_1.position.sub(&atom_2.position);
            if dist_vec.mag() < 2.0 {
                bonds.push([atom_1.clone(), atom_2.clone()]);
            }
        }
    }

    Ok(
        Molecule {
            atoms,
            bonds,
            name,
        }
    )
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

impl std::error::Error for UnsupportedFormat { }

impl Molecule {
    pub fn center(&self) -> [f32; 3] {
        #![allow(clippy::cast_precision_loss)]
        use webgl_matrix::Vector;

        self.atoms.iter().fold(
            [0.0, 0.0, 0.0], |acc, x| {[
                acc[0] + x.position[0],
                acc[1] + x.position[1],
                acc[2] + x.position[2],
        ]}).scale(1.0 / self.atoms.len() as f32)
    }

    pub fn from_string_with_format(contents: &str, format: &str) -> Result<Self, Box<dyn Error>> {
        match format {
            "xyz" => read_xyz(contents),
            _ => Err(Box::new(UnsupportedFormat{
                format: format.to_owned()
            })),
        }
    }
}
