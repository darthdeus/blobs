#[cfg(test)]
mod tests {
    use crate::*;
    use glam::vec2;
    use assert_approx_eq::assert_approx_eq;

    // A helper function that creates a spatial hash with some points
    fn create_spatial_hash() -> SpatialHash {
        let mut spatial_hash = SpatialHash::new(100.0);
        spatial_hash.insert(vec2(50.0, 50.0), 1.0); // id 0
        spatial_hash.insert(vec2(150.0, 150.0), 1.0); // id 1
        spatial_hash.insert(vec2(250.0, 250.0), 1.0); // id 2
        spatial_hash.insert(vec2(350.0, 350.0), 1.0); // id 3
        spatial_hash.insert(vec2(450.0, 450.0), 1.0); // id 4
        spatial_hash
    }

    #[test]
    fn test_get_cell_coords() {
        let spatial_hash = create_spatial_hash();
        assert_eq!(spatial_hash.get_cell_coords(vec2(50.0, 50.0)), (0, 0));
        assert_eq!(spatial_hash.get_cell_coords(vec2(150.0, 150.0)), (1, 1));
        assert_eq!(spatial_hash.get_cell_coords(vec2(250.0, 250.0)), (2, 2));
        assert_eq!(spatial_hash.get_cell_coords(vec2(350.0, 350.0)), (3, 3));
        assert_eq!(spatial_hash.get_cell_coords(vec2(450.0, 450.0)), (4, 4));
    }

    #[test]
    fn test_insert() {
        let mut spatial_hash = SpatialHash::new(100.0);
        let id = spatial_hash.insert(vec2(50.5, -25.5), 1.0);

        assert_eq!(id, 0);
        assert_eq!(spatial_hash.next_id, 1);

        let cell_coords = spatial_hash.get_cell_coords(vec2(50.5, -25.5));
        let cell = spatial_hash.grid.get(&cell_coords).unwrap();
        assert!(!cell.is_empty());

        let point = spatial_hash.points.get(&id).unwrap();
        assert_eq!(point.id, 0);

        assert_approx_eq!(point.position.x, 50.5);
        assert_approx_eq!(point.position.y, -25.5);
    }

    #[test]
    fn insert_and_query() {
        let mut hash = SpatialHash::new(1.0);
        let p1 = hash.insert(Vec2::new(0.0, 0.0), 0.0);
        let p2 = hash.insert(Vec2::new(2.0, 2.0), 0.0);

        let results = hash.query(Vec2::new(1.0, 1.0), 1.5);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.id == p1));
        assert!(results.iter().any(|p| p.id == p2));
    }

    #[test]
    fn move_point() {
        let mut hash = SpatialHash::new(1.0);
        let p = hash.insert(Vec2::new(0.0, 0.0), 0.0);

        hash.move_point(p, Vec2::new(2.0, 2.0)).unwrap();

        let results = hash.query(Vec2::new(1.0, 1.0), 1.5);
        assert_eq!(results.len(), 1);
        assert!(results.iter().any(|point| point.id == p && point.position == Vec2::new(2.0, 2.0)));
    }

    #[test]
    fn remove() {
        let mut hash = SpatialHash::new(1.0);
        let p = hash.insert(Vec2::new(0.0, 0.0), 0.0);

        hash.remove(p);

        let results = hash.query(Vec2::new(0.0, 0.0), 1.5);
        assert!(results.is_empty());
    }
}
