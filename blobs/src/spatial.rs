use glam::Vec2;
use std::collections::{HashMap, HashSet};

pub type CellIndex = (i32, i32);
pub type Id = u64;

#[derive(Copy, Clone, Debug)]
pub struct CellPoint {
    pub id: u64,
    pub position: Vec2,
    pub radius: f32,
}

impl PartialEq for CellPoint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct SpatialHash {
    pub cell_size: f32,
    pub next_id: u64,
    pub points: HashMap<u64, CellPoint>,
    pub grid: HashMap<(i32, i32), HashSet<u64>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            next_id: 0,
            points: HashMap::new(),
            grid: HashMap::new(),
        }
    }

    fn get_cell_coords(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, position: Vec2, radius: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.insert_with_id(id, position, radius);
        id
    }

    pub fn insert_with_id(&mut self, id: u64, position: Vec2, radius: f32) {
        let point = CellPoint { id, position, radius };
        let cell_coords = self.get_cell_coords(point.position);

        self.grid
            .entry(cell_coords)
            .or_insert_with(HashSet::new)
            .insert(point.id);
        self.points.insert(point.id, point);
    }

    pub fn remove(&mut self, id: u64) -> Option<CellPoint> {
        if let Some(point) = self.points.remove(&id) {
            let cell_coords = self.get_cell_coords(point.position);
            if let Some(cell) = self.grid.get_mut(&cell_coords) {
                cell.remove(&id);
            }
            Some(point)
        } else {
            None
        }
    }

    pub fn move_point(&mut self, id: u64, offset: Vec2) -> Option<()> {
        if let Some(point) = self.points.get(&id) {
            let old_position = point.position;
            let new_position = old_position + offset;

            let old_cell_coords = self.get_cell_coords(old_position);
            let new_cell_coords = self.get_cell_coords(new_position);

            if old_cell_coords != new_cell_coords {
                if let Some(cell) = self.grid.get_mut(&old_cell_coords) {
                    cell.remove(&id);
                }
                self.grid
                    .entry(new_cell_coords)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }

            if let Some(point) = self.points.get_mut(&id) {
                point.position = new_position;
            }
            Some(())
        } else {
            None
        }
    }

    fn get_neighbor_cells(&self, cell_coords: (i32, i32)) -> Vec<(i32, i32)> {
        let (x, y) = cell_coords;
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    }

    pub fn query(&self, position: Vec2, query_radius: f32) -> Vec<CellPoint> {
        let cell_coords = self.get_cell_coords(position);
        let neighbor_cells = self.get_neighbor_cells(cell_coords);

        let mut points_within_radius = Vec::new();
        for cell_coords in &neighbor_cells {
            if let Some(cell) = self.grid.get(cell_coords) {
                for point_id in cell {
                    let point = self.points.get(point_id).unwrap();
                    if (point.position - position).length() - point.radius <=
                        query_radius
                    {
                        points_within_radius.push(point.clone());
                    }
                }
            }
        }

        points_within_radius
    }

    pub fn query_with_cells(
        &self,
        position: Vec2,
        query_radius: f32,
    ) -> (Vec<CellPoint>, Vec<(i32, i32)>) {
        let cell_coords = self.get_cell_coords(position);
        let neighbor_cells = self.get_neighbor_cells(cell_coords);

        let mut points_within_radius = Vec::new();
        for cell_coords in &neighbor_cells {
            if let Some(cell) = self.grid.get(cell_coords) {
                for point_id in cell {
                    let point = self.points.get(point_id).unwrap();
                    if (point.position - position).length() - point.radius <=
                        query_radius
                    {
                        points_within_radius.push(point.clone());
                    }
                }
            }
        }
        (points_within_radius, neighbor_cells)
    }
}
