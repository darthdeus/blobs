use glam::Vec2;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub type CellIndex = (i32, i32);
pub type Id = u64;

#[derive(Copy, Clone, Debug)]
struct CellPoint {
    id: Id,
    position: Vec2,
}

pub struct SpatialHash {
    cell_size: f32,
    next_id: u64,
    hash_map: HashMap<CellIndex, Vec<CellPoint>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self { cell_size, next_id: 0, hash_map: HashMap::new() }
    }

    fn hash_point(&self, point: Vec2) -> (i32, i32) {
        (
            (point.x / self.cell_size).floor() as i32,
            (point.y / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, point: Vec2) -> Id {
        let hash = self.hash_point(point);
        let id = self.next_id;
        self.next_id += 1;

        let point = CellPoint { id: 0, position: point };

        match self.hash_map.entry(hash) {
            Entry::Occupied(mut entry) => entry.get_mut().push(point),
            Entry::Vacant(entry) => {
                entry.insert(vec![point]);
            }
        }

        id
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

    pub fn query(&self, point: Vec2, radius: f32) -> Vec<Vec2> {
        let mut results = Vec::new();
        let min_hash = self.hash_point(point - Vec2::splat(radius));
        let max_hash = self.hash_point(point + Vec2::splat(radius));

        for x in min_hash.0..=max_hash.0 {
            for y in min_hash.1..=max_hash.1 {
                if let Some(points) = self.hash_map.get(&(x, y)) {
                    for &p in points {
                        if (p - point).length_squared() <= radius * radius {
                            results.push(p);
                        }
                    }
                }
            }
        }

        results
    }

    pub fn move_point(&mut self, old_point: Vec2, new_point: Vec2) -> bool {
        if self.remove(old_point) {
            self.insert(new_point);
            true
        } else {
            false
        }
    }
}
