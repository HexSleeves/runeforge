use crate::prelude::*;
use pathfinding::prelude::dfs;

/// Depth-first search algorithm.
pub struct Dfs;

impl PathAlgorithm for Dfs {
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<IVec2> {
        let result = dfs(
            origin,
            |&p| provider.get_neighbors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        result.unwrap_or_else(Vec::new)
    }
}
