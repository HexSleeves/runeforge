//! Random number generation with roguelike-specific utilities.
//!
//! This crate provides convenient wrappers around the `rand` crate,
//! including dice roll notation parsing and weighted random selection.

#![deny(missing_docs)]

use rand::{prelude::*, rng, Rng as RngTrait};

/// A random number generator wrapper with convenience methods for roguelike development.
pub struct Rng {
    rng: ThreadRng,
}

impl Rng {
    /// Creates a new random number generator using the thread-local RNG.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let value = rng.range(1, 10);
    /// assert!(value >= 1 && value <= 10);
    /// ```
    pub fn new() -> Self {
        Self { rng: rng() }
    }

    /// Generates a random number in the inclusive range [min, max].
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let value = rng.range(1, 6);  // Equivalent to a d6
    /// assert!(value >= 1 && value <= 6);
    /// ```
    #[inline]
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.random_range(min..=max)
    }

    /// Generates a random floating-point number in the range [0.0, 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let value = rng.float();
    /// assert!(value >= 0.0 && value < 1.0);
    /// ```
    #[inline]
    pub fn float(&mut self) -> f32 {
        self.rng.random()
    }

    /// Returns true with the given probability (0.0 to 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let _ = rng.chance(0.5);  // 50% chance of true
    /// ```
    #[inline]
    pub fn chance(&mut self, probability: f32) -> bool {
        self.rng.random::<f32>() < probability
    }

    /// Rolls dice in XdY format (e.g., 3d6 = roll 3 six-sided dice).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let result = rng.roll_dice(3, 6);  // Roll 3d6
    /// assert!(result >= 3 && result <= 18);
    /// ```
    pub fn roll_dice(&mut self, num_dice: u32, sides: i32) -> i32 {
        (0..num_dice).map(|_| self.range(1, sides)).sum()
    }

    /// Shuffles a slice in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let mut items = vec![1, 2, 3, 4, 5];
    /// rng.shuffle(&mut items);
    /// // items is now in random order
    /// ```
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(&mut self.rng);
    }

    /// Chooses a random element from a slice.
    ///
    /// Returns `None` if the slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let items = vec!["sword", "shield", "potion"];
    /// if let Some(item) = rng.choose(&items) {
    ///     println!("You found a {}!", item);
    /// }
    /// ```
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        slice.choose(&mut self.rng)
    }

    /// Chooses a random element with weighted probabilities.
    ///
    /// Returns `None` if the slice is empty or all weights are zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_random::Rng;
    ///
    /// let mut rng = Rng::new();
    /// let items = vec!["common", "uncommon", "rare"];
    /// let weights = vec![70, 25, 5];  // 70%, 25%, 5%
    /// if let Some(&item) = rng.weighted_choose(&items, &weights) {
    ///     println!("You found a {} item!", item);
    /// }
    /// ```
    pub fn weighted_choose<'a, T>(&mut self, items: &'a [T], weights: &[u32]) -> Option<&'a T> {
        if items.is_empty() || items.len() != weights.len() {
            return None;
        }

        let total: u32 = weights.iter().sum();
        if total == 0 {
            return None;
        }

        let mut roll = self.rng.random_range(0..total);
        for (item, &weight) in items.iter().zip(weights.iter()) {
            if roll < weight {
                return Some(item);
            }
            roll -= weight;
        }

        items.last()
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses dice notation (e.g., "3d6+2", "1d20-1", "2d10").
///
/// # Format
///
/// The format is: `[num]d[sides][+/-modifier]`
///
/// - `num`: Number of dice to roll (defaults to 1 if omitted)
/// - `sides`: Number of sides on each die
/// - `modifier`: Optional bonus or penalty
///
/// # Examples
///
/// ```
/// use runeforge_random::parse_dice;
///
/// assert_eq!(parse_dice("3d6").unwrap(), (3, 6, 0));
/// assert_eq!(parse_dice("1d20+5").unwrap(), (1, 20, 5));
/// assert_eq!(parse_dice("d8-2").unwrap(), (1, 8, -2));
/// ```
pub fn parse_dice(notation: &str) -> Result<(u32, i32, i32), String> {
    let notation = notation.trim().to_lowercase();

    // Find the 'd' separator
    let Some(d_pos) = notation.find('d') else {
        return Err(format!(
            "Invalid dice notation: missing 'd' in '{}'",
            notation
        ));
    };

    // Parse number of dice
    let num_str = &notation[..d_pos];
    let num_dice = if num_str.is_empty() {
        1
    } else {
        num_str
            .parse::<u32>()
            .map_err(|_| format!("Invalid number of dice: '{}' in '{}'", num_str, notation))?
    };

    // Find modifier (+ or -)
    let rest = &notation[d_pos + 1..];
    let (sides_str, modifier) = if let Some(plus_pos) = rest.find('+') {
        let modifier_str = &rest[plus_pos + 1..];
        let modifier = modifier_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid modifier: '{}' in '{}'", modifier_str, notation))?;
        (&rest[..plus_pos], modifier)
    } else if let Some(minus_pos) = rest.find('-') {
        let modifier_str = &rest[minus_pos + 1..];
        let modifier = modifier_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid modifier: '{}' in '{}'", modifier_str, notation))?;
        (&rest[..minus_pos], -modifier)
    } else {
        (rest, 0)
    };

    // Parse sides
    let sides = sides_str
        .parse::<i32>()
        .map_err(|_| format!("Invalid number of sides: '{}' in '{}'", sides_str, notation))?;

    if sides < 1 {
        return Err(format!("Dice must have at least 1 side, got {}", sides));
    }

    Ok((num_dice, sides, modifier))
}

/// Rolls dice from a dice notation string.
///
/// # Examples
///
/// ```
/// use runeforge_random::roll_dice_notation;
///
/// let result = roll_dice_notation("3d6+2").unwrap();
/// assert!(result >= 5 && result <= 20);  // 3*1+2 to 3*6+2
/// ```
pub fn roll_dice_notation(notation: &str) -> Result<i32, String> {
    let (num_dice, sides, modifier) = parse_dice(notation)?;
    let mut rng = Rng::new();
    Ok(rng.roll_dice(num_dice, sides) + modifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let mut rng = Rng::new();
        for _ in 0..100 {
            let value = rng.range(1, 6);
            assert!((1..=6).contains(&value));
        }
    }

    #[test]
    fn test_roll_dice() {
        let mut rng = Rng::new();
        let result = rng.roll_dice(3, 6);
        assert!((3..=18).contains(&result));
    }

    #[test]
    fn test_parse_dice() {
        assert_eq!(parse_dice("3d6").unwrap(), (3, 6, 0));
        assert_eq!(parse_dice("1d20+5").unwrap(), (1, 20, 5));
        assert_eq!(parse_dice("d8-2").unwrap(), (1, 8, -2));
        assert_eq!(parse_dice("2d10+3").unwrap(), (2, 10, 3));
    }

    #[test]
    fn test_parse_dice_errors() {
        assert!(parse_dice("invalid").is_err());
        assert!(parse_dice("3x6").is_err());
        assert!(parse_dice("d0").is_err());
    }

    #[test]
    fn test_chance() {
        let mut rng = Rng::new();

        // Test with 100% chance - should always be true
        for _ in 0..10 {
            assert!(rng.chance(1.0));
        }

        // Test with 0% chance - should always be false
        for _ in 0..10 {
            assert!(!rng.chance(0.0));
        }
    }

    #[test]
    fn test_weighted_choose() {
        let mut rng = Rng::new();
        let items = vec!["a", "b", "c"];
        let weights = vec![100, 0, 0]; // 100% chance of 'a'

        for _ in 0..10 {
            assert_eq!(rng.weighted_choose(&items, &weights), Some(&"a"));
        }
    }
}
