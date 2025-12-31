use crate::prelude::*;

/// A trait for providing information about the field of view.
pub trait FovProvider<T> {
    /// Returns true if the specified position is opaque.
    fn is_opaque(&mut self, position: IVec2, pass_through_data: &mut T) -> bool;
}

/// A trait for computing the field of view.
pub trait FovAlgorithm {
    /// Computes the field of view.
    fn compute_fov<T>(
        origin: IVec2,
        range: u32,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<IVec2>;
}

pub enum Fov {
    /// Use the Adams algorithm for computing field of view
    Adams,
    /// Use the Shadowcast algorithm for computing field of view
    Shadowcast,
    /// Use the Shadowcast algorithm for computing field of view, but only in a single direction
    ShadowcastDirection(Direction),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>, T>(
        &self,
        origin: IVec2,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<IVec2> {
        let range = range.into();
        match self {
            Self::Adams => AdamsFov::compute_fov(origin, range, provider, pass_through_data),
            Self::Shadowcast => Shadowcast::compute_fov(origin, range, provider, pass_through_data),
            Self::ShadowcastDirection(direction) => Shadowcast::compute_direction(
                origin,
                range,
                provider,
                *direction,
                pass_through_data,
            ),
        }
    }

    pub fn within_fov<FovRange: Into<u32>, T>(
        &self,
        origin: IVec2,
        target: IVec2,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> bool {
        let range = range.into();
        Self::compute(self, origin, range, provider, pass_through_data).contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use runeforge_geometry::prelude::*;

    struct Provider;
    impl FovProvider<()> for Provider {
        fn is_opaque(&mut self, _position: IVec2, _pass_through_data: &mut ()) -> bool {
            false
        }
    }

    /// A simple canvas for ASCII debugging.
    struct Canvas {
        size: UVec2,
        data: String,
    }

    impl Canvas {
        fn new(size: UVec2) -> Self {
            let data = " ".repeat((size.x * size.y) as usize);
            Self { size, data }
        }

        fn put(&mut self, pos: UVec2, glyph: char) {
            let i = pos.y as usize * self.size.x as usize + pos.x as usize;
            self.data
                .replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
        }

        fn print(&self) {
            print!("   ");
            (0..self.size.x).for_each(|i| print!("{i}"));
            println!();
            let chars: Vec<_> = self.data.replace(' ', ".").chars().collect();
            for (i, line) in chars.chunks(self.size.x as usize).enumerate() {
                println!("{:>2} {}", i, String::from_iter(line.iter()));
            }
        }
    }

    mod shadowcast {
        use super::*;

        #[test]
        fn shadowcast() {
            let pos = IVec2::new(5, 5);
            let visible_sets = Fov::Shadowcast.compute(pos, 4_u32, &mut Provider, ());
            assert_eq!(visible_sets.len(), 49);

            // Pretty print to canvas for visual inspection
            let mut canvas = Canvas::new(UVec2::new(10, 10));
            visible_sets.iter().for_each(|pos| {
                println!("{}", pos);
                canvas.put(pos.as_uvec2(), '*');
            });
            canvas.print();
        }
    }

    mod adams {
        use super::*;

        #[test]
        fn adams() {
            let pos = IVec2::new(5, 5);
            let visible_sets = Fov::Adams.compute(pos, 4_u32, &mut Provider, ());
            assert_eq!(visible_sets.len(), 49);

            // Pretty print to canvas for visual inspection
            let mut canvas = Canvas::new(UVec2::new(10, 10));
            visible_sets.iter().for_each(|pos| {
                println!("{}", pos);
                canvas.put(pos.as_uvec2(), '*');
            });
            canvas.print();
        }
    }
}
