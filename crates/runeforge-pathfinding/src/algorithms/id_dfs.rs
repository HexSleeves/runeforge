use crate::prelude::*;
use pathfinding::prelude::iddfs;

/// Iterative deepening depth-first search algorithm.
pub struct IDDfs;

impl PathAlgorithm for IDDfs {
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<IVec2> {
        let result = iddfs(
            origin,
            |&p| provider.get_neighbors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        result.unwrap_or_else(Vec::new)
    }
}
