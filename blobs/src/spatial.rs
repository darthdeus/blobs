use glam::Vec2;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub type CellIndex = (i32, i32);
pub type Id = u64;

#[derive(Copy, Clone, Debug)]
pub struct CellPoint {
    pub id: Id,
    pub radius: f32,
    pub position: Vec2,
}

impl PartialEq for CellPoint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct SpatialHash {
    pub cell_size: f32,
    pub next_id: u64,
    pub hash_map: HashMap<CellIndex, Vec<CellPoint>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self { cell_size, next_id: 0, hash_map: HashMap::new() }
    }

    pub fn hash_point(&self, point: Vec2) -> (i32, i32) {
        (
            (point.x / self.cell_size).floor() as i32,
            (point.y / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, point: Vec2, radius: f32) -> CellPoint {
        let id = self.next_id;
        self.next_id += 1;

        let point = CellPoint { id, position: point, radius };

        self.insert_with_id(point);
        point
    }

    fn insert_with_id(&mut self, point: CellPoint) {
        let hash = self.hash_point(point.position);

        match self.hash_map.entry(hash) {
            Entry::Occupied(mut entry) => entry.get_mut().push(point),
            Entry::Vacant(entry) => {
                entry.insert(vec![point]);
            }
        }
    }

    pub fn remove(&mut self, point: &CellPoint) -> bool {
        let hash = self.hash_point(point.position);

        if let Some(points) = self.hash_map.get_mut(&hash) {
            if let Some(index) = points.iter().position(|&p| p.id == point.id) {
                points.swap_remove(index);
                return true;
            }
        }

        false
    }

    pub fn query(&self, point: Vec2, radius: f32) -> Vec<CellPoint> {
        let mut results = Vec::new();
        let min_hash = self.hash_point(point - Vec2::splat(radius));
        let max_hash = self.hash_point(point + Vec2::splat(radius));

        for x in min_hash.0..=max_hash.0 {
            for y in min_hash.1..=max_hash.1 {
                if let Some(points) = self.hash_map.get(&(x, y)) {
                    for &p in points {
                        // if (p.position - point).length_squared() <=
                        //     radius * radius
                        // {
                        //     results.push(p);
                        // }

                        if (p.position - point).length() <= (radius + p.radius)
                        {
                            results.push(p);
                        }
                    }
                }
            }
        }

        results
    }

    pub fn move_point(&mut self, point: CellPoint) -> bool {
        if self.remove(&point) {
            self.insert_with_id(point);
            true
        } else {
            false
        }
    }
}
