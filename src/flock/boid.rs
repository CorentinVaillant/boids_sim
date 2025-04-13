use glium::dynamic_uniform;
use my_glium_util::{canvas::traits::CanvasDrawable, datastruct::quadtree::As2dPoint, maths::{types::Vec2, EuclidianSpace, VectorSpace}};

pub struct Boid{
    position : Vec2,
    velocity : Vec2,

    size : f32,

    separation_force : Vec2,
    alignement_force : Vec2,
    cohesion_force : Vec2,

    cohesion_number : f32,
    alignement_number: f32,


    border_margin : f32,
    separation: f32,
    alignement : f32,
    cohesion : f32,

    turn_factor : f32,

    z : f32
}

impl Boid{
    pub fn new(resolution: (f32,f32))->Self{
        Boid { 
            position: [resolution.0/2.,resolution.1/2.].into(), 
            velocity: [100.,50.].into(), 
            size: 5., 

            separation_force : Vec2::v_space_zero(),
            alignement_force : Vec2::v_space_zero(),
            alignement_number: 0.,
            cohesion_force : Vec2::v_space_zero(),
            cohesion_number : 0.,

            border_margin : 50.,
            separation: 20., 
            alignement: 50., 
            cohesion: 80., 

            turn_factor : 2.,

            z: 1., 

            
        }
    }
}

impl CanvasDrawable for Boid{
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn canvas_uniforms(&self) -> Vec<glium::uniforms::DynamicUniforms> {
        vec![dynamic_uniform! {
            position : self.position.as_array(),
            velocity : self.velocity.as_array(),

            separation: &self.separation,
            alignement: &self.alignement,
            cohesion: &self.cohesion,

            size : &self.size,

            z : &self.z,
        }]
    }

    fn update(&mut self, canva_info: &my_glium_util::canvas::CanvasData, dt: f32) {

        let border: (f32, f32) = (
            canva_info.size.0 * canva_info.window_resolution.0 as f32,
            canva_info.size.1 * canva_info.window_resolution.1 as f32,
        );

        self.handle_border_colision(border);
        self.apply_forces(dt);
    }
}


impl Boid{
    pub fn handle_border_colision(&mut self, (b_x, b_y): (f32, f32)) {
        let [x, y] = &mut self.position.as_mut_array();
        let [v_x, v_y] = &mut self.velocity.as_mut_array();
        let size = self.size;

        // Bounding box
        if *x < size {
            *x = size; // prevent sticking
            *v_x = 0.;
        } else if *x > b_x - size {
            *x = b_x - size; // prevent sticking
            *v_x = 0.;
        }

        if *y < size {
            *y = size; // prevent sticking
            *v_y = 0.;
        } else if *y > b_y - size {
            *y = b_y - size; // prevent sticking
            *v_y = 0.;
        }


        if *x < self.border_margin{
            *v_x += self.turn_factor;
        }
        if *x > b_x -  self.border_margin{
            *v_x -= self.turn_factor;
        }
        if *y < self.border_margin{
            *v_y += self.turn_factor;
        }
        if *y > b_y -  self.border_margin{
            *v_y -= self.turn_factor;
        }


    }

    pub fn reset_forces(&mut self){
        self.separation_force = Vec2::v_space_zero();
        self.alignement_force = Vec2::v_space_zero();
        self.cohesion_force = Vec2::v_space_zero();
    }

    pub fn apply_forces(&mut self, dt:f32){
        self.position += self.velocity * dt;
    }

    pub fn apply_separation(&mut self,dt:f32){
        self.velocity += self.separation_force * dt;
    }

    pub fn apply_cohesion(&mut self, dt:f32){
        
        if self.cohesion_number > 0.{
            self.velocity += (self.cohesion_force / self.cohesion_number) * dt
        }
    }

    pub fn apply_alignement(&mut self,dt:f32){
        if self.alignement_number > 0.{
            self.velocity += self.alignement_force
        }
    }


    pub fn handle_separation(&mut self, other : &Self){
        if self.position.distance_sq(other.position) < self.separation * self.separation{
            self.separation_force -= self.position - other.position;
        }
    }

    pub fn handle_cohesion(&mut self, other: &Self){
        
        if self.position.distance_sq(other.position) < self.cohesion*self.cohesion{
            self.cohesion_number += 1.;
            self.cohesion_force += other.position;
        }
    }

    pub fn handle_alignement(&mut self, other: &Self){
        if self.position.distance_sq(other.position) < self.alignement * self.cohesion{
            self.alignement_number +=1.;
            self.alignement_force += other.velocity;
        }
    }
}

impl As2dPoint for Boid{
    
    #[inline]
    fn x(&self) -> f32 {
        self.position[0]
    }

    #[inline]
    fn y(&self) -> f32 {
        self.position[1]
    }
}