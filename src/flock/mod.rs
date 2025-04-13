use boid::Boid;
use my_glium_util::{canvas::traits::CanvasDrawable, datastruct::quadtree::{Quadtree, AABB}};

pub mod boid;

pub struct Flock{
    boids : Quadtree<Boid,10>,

    boundary : AABB,
    z:f32,
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
}