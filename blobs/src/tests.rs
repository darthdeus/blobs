#[cfg(test)]
mod tests {
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    use glam::vec2;

    // A helper function that creates a spatial hash with some points
    fn create_spatial_hash() -> SpatialHash {
        let mut spatial_hash = SpatialHash::new(100.0);
        spatial_hash.insert(vec2(50.0, 50.0)); // id 0
        spatial_hash.insert(vec2(150.0, 150.0)); // id 1
        spatial_hash.insert(vec2(250.0, 250.0)); // id 2
        spatial_hash.insert(vec2(350.0, 350.0)); // id 3
        spatial_hash.insert(vec2(450.0, 450.0)); // id 4
        spatial_hash
    }

    #[test]
    fn test_hash_point() {
        let spatial_hash = create_spatial_hash();
        assert_eq!(spatial_hash.hash_point(vec2(50.0, 50.0)), (0, 0));
        assert_eq!(spatial_hash.hash_point(vec2(150.0, 150.0)), (1, 1));
        assert_eq!(spatial_hash.hash_point(vec2(250.0, 250.0)), (2, 2));
        assert_eq!(spatial_hash.hash_point(vec2(350.0, 350.0)), (3, 3));
        assert_eq!(spatial_hash.hash_point(vec2(450.0, 450.0)), (4, 4));
    }

    #[test]
    fn test_insert() {
        let mut spatial_hash = SpatialHash::new(100.0);
        let id = spatial_hash.insert(vec2(50.5, -25.5));
        assert_eq!(id, 0);
        assert_eq!(spatial_hash.next_id, 1);

        let cell_index = spatial_hash.hash_point(vec2(50.5, -25.5));

        if let Some(cell) = spatial_hash.hash_map.get(&cell_index) {
            assert!(!cell.is_empty());
            let point = cell[0];
            assert_eq!(point.id, 0);
            assert_approx_eq!(point.position.x, 50.5);
            assert_approx_eq!(point.position.y, -25.5);
        } else {
            panic!("Cell not found");
        }
    }

    #[test]
    fn insert_and_query() {
        let mut hash = SpatialHash::new(1.0);
        let p1 = hash.insert(Vec2::new(0.0, 0.0));
        let p2 = hash.insert(Vec2::new(2.0, 2.0));

        let results = hash.query(Vec2::new(1.0, 1.0), 1.5);
        assert_eq!(results.len(), 2);
        assert!(results
            .contains(&CellPoint { id: p1, position: Vec2::new(0.0, 0.0) }));
        assert!(results
            .contains(&CellPoint { id: p2, position: Vec2::new(2.0, 2.0) }));
    }

    #[test]
    fn move_point() {
        let mut hash = SpatialHash::new(1.0);
        let p = hash.insert(Vec2::new(0.0, 0.0));

        hash.move_point(CellPoint { id: p, position: Vec2::new(2.0, 2.0) });

        let results = hash.query(Vec2::new(1.0, 1.0), 1.5);
        assert_eq!(results.len(), 1);
        assert!(results
            .contains(&CellPoint { id: p, position: Vec2::new(2.0, 2.0) }));
    }

    #[test]
    fn remove() {
        let mut hash = SpatialHash::new(1.0);
        let p = hash.insert(Vec2::new(0.0, 0.0));

        hash.remove(&CellPoint { id: p, position: Vec2::new(0.0, 0.0) });

        let results = hash.query(Vec2::new(0.0, 0.0), 1.5);
        assert!(results.is_empty());
    }
}
