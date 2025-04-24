use boid::Boid;
use my_glium_util::{
    canvas::traits::CanvasDrawable,
    datastruct::{aabb::Aabb, quadtree::Quadtree},
};

pub mod boid;

pub struct Flock {
    boids: Quadtree<f32,Boid, 10>,

    boundary: Aabb<f32>,
    z: f32,
}

impl Flock {
    pub fn new(boids: Vec<Boid>, bound: Aabb<f32>) -> Self {
        Self {
            boids: Quadtree::new(bound, boids),

            boundary: bound,
            z: 0.5,
        }
    }
}

impl CanvasDrawable for Flock {
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn canvas_uniforms(&self) -> Vec<glium::uniforms::DynamicUniforms> {
        let mut result = Vec::with_capacity(self.boids.len());
        for boid in self.boids.query_range(self.boundary) {
            let mut uni = boid.canvas_uniforms();
            result.append(&mut uni);
        }

        result
    }

    fn update(&mut self, canva_info: &my_glium_util::canvas::CanvasData, dt: f32) {
        let border: (f32, f32) = (
            (self.boundary.center.x + self.boundary.half_dim)
                .min(canva_info.size.0 * canva_info.window_resolution.0 as f32),
            (self.boundary.center.y + self.boundary.half_dim)
                .min(canva_info.size.1 * canva_info.window_resolution.1 as f32),
        );

        const PHYSIC_SUB_STEP: u16 = 10;

        let sub_dt = dt / f32::from(PHYSIC_SUB_STEP);
        let boids = &mut self.boids;

        let range_mapping = |boid: &Boid| {
            Aabb::new(
                (*boid.position.as_array()).into(),
                boid.separation.max(boid.alignement).max(boid.cohesion),
            )
        };

        let first_map = |boid: &mut Boid| {
            boid.reset_forces();
            boid.handle_border_colision(border);
        };

        let map_with_other = |boid: &mut Boid, other: &mut Boid| {
            boid.handle_color(other);
            boid.handle_separation(other);
            boid.handle_alignement(other);
            boid.handle_cohesion(other);
        };

        let last_map = |boid: &mut Boid| {
            boid.apply_color();
            boid.apply_separation(sub_dt);
            boid.apply_alignement(sub_dt);
            boid.apply_cohesion(sub_dt);

            boid.apply_forces(sub_dt);
        };

        for _ in 0..PHYSIC_SUB_STEP {
            boids.map_then_map_with_elem_in_range_then_map(
                first_map,
                range_mapping,
                map_with_other,
                last_map,
            );
        }
    }

    fn is_absolute_coord_in(&self, _: (f32, f32)) -> bool {
        true
    }

    fn is_relative_coord_in(&self, _: (f32, f32)) -> bool {
        true
    }

    fn on_click(&mut self, coord: (f32, f32)) {
        let _ = self.boids.insert(Boid::new(coord,self.boids.len()));
    }

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        println!("-- Flock resized");
        let new_bound = Aabb::from_min_max((0.,0.), (new_size.0 as f32,new_size.1 as f32));

        for boid in self.boids.iter_mut() {
            boid.on_window_resized(new_size);
        }

        let _ = self.boids.change_bounds(new_bound);
    }
}
