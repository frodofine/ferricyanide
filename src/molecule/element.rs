pub enum Element {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Phosphorus,
    Sulfur,
    Other,
}

impl Element {
    pub fn covalent_radius(&self) -> f32 {
        use Element::*;
        match self {
            Hydrogen => 0.31,
            Helium => 0.28,
            Lithium => 1.28,
            Beryllium => 0.96,
            Boron => 0.84,
            Carbon => 0.76,
            Nitrogen => 0.71,
            Oxygen => 0.66,
            Fluorine => 0.57,
            Neon => 0.58,
            Phosphorus => 1.07,
            Sulfur => 1.05,
            Other => 2.0,
        }
    }

    pub fn cpk_color(&self) -> [f32; 4] {
        #![allow(clippy::identity_op, clippy::cast_precision_loss)]

        use Element::*;
        let color: u32 = match self {
            Hydrogen => 0xD0_D0_D0_FF,
            Helium => 0xD9_FF_FF_FF,
            Lithium => 0xCC_80_FF_FF,
            Beryllium => 0xC2_FF_00_FF,
            Boron => 0xFF_B5_B5_FF,
            Carbon => 0x90_90_90_FF,
            Nitrogen => 0x30_50_F8_FF,
            Oxygen => 0xFF_0D_0D_FF,
            Fluorine => 0x90_E0_50_FF,
            Neon => 0xB3_E3_F5_FF,
            Phosphorus => 0xFF_80_00_FF,
            Sulfur => 0xFF_FF_30_FF,
            Other => 0x00_00_00_FF,
        };

        [
            ((color & 0xFF_00_00_00) >> 24) as f32 / 255.0,
            ((color & 0x00_FF_00_00) >> 16) as f32 / 255.0,
            ((color & 0x00_00_FF_00) >> 8) as f32 / 255.0,
            ((color & 0x00_00_00_FF) >> 0) as f32 / 255.0,
        ]
    }
}

impl From<&str> for Element {
    fn from(element: &str) -> Self {
        use Element::*;
        match element {
            "H" | "Hydrogen" => Hydrogen,
            "He" | "Helium" => Helium,
            "Li" | "Lithium" => Lithium,
            "Be" | "Beryllium" => Beryllium,
            "B" | "Boron" => Boron,
            "C" | "Carbon" => Carbon,
            "N" | "Nitrogen" => Nitrogen,
            "O" | "Oxygen" => Oxygen,
            "F" | "Fluorine" => Fluorine,
            "Ne" | "Neon" => Neon,
            "S" | "Sulfur" => Sulfur,
            "P" | "Phosphorus" => Phosphorus,
            _ => Other,
        }
    }
}
