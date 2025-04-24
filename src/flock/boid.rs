use glium::dynamic_uniform;
use my_glium_util::{
    canvas::traits::CanvasDrawable,
    datastruct::points::As2dPoint, math::{EuclidianSpace, Vec2, Vec3, VectorSpace},
};

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,

    pub size: f32,

    separation_force: Vec2,
    alignement_force: Vec2,
    cohesion_force: Vec2,

    cohesion_number: f32,
    alignement_number: f32,

    border_margin: f32,
    pub separation: f32,
    pub alignement: f32,
    pub cohesion: f32,

    avoid_factor: f32,
    matching_factor: f32,
    centering_factor: f32,
    turn_factor: f32,

    color:Vec3,
    avg_color : Vec3,
    avg_color_nominator : Vec3,
    avg_color_denominator : f32,
    z: f32,
}

impl Boid {
    pub fn new(pos: (f32, f32),id:usize) -> Self {
        let pos = [pos.0, pos.1].into();

        let color =hue_to_rgb(id as f32 * 4. * std::f32::consts::FRAC_PI_2 / 32.).into(); 
        Boid {
            position: pos,
            velocity: [0., 0.].into(),
            size: 2.,

            separation_force: Vec2::v_space_zero(),
            alignement_force: Vec2::v_space_zero(),
            alignement_number: 0.,
            cohesion_force: Vec2::v_space_zero(),
            cohesion_number: 0.,

            border_margin: 50.,
            separation: 8.,
            alignement: 40.,
            cohesion: 40.,

            avoid_factor: 3.,
            matching_factor: 3.,
            centering_factor: 0.03,
            turn_factor: 2.,

            color,
            avg_color:color,

            avg_color_nominator : Vec3::v_space_zero(),
            avg_color_denominator : 0.,
            z: 1.,
        }
    }
}

impl CanvasDrawable for Boid {
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
            color : self.avg_color.as_array(),

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

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        println!("  |- Boid resized");
        let new_size = (new_size.0 as f32, new_size.1 as f32);
        self.handle_border_colision(new_size);
        self.handle_border_colision(new_size);

    }

}

impl Boid {
    pub fn handle_border_colision(&mut self, (b_x, b_y): (f32, f32)) {
        let [x, y] = &mut self.position.as_mut_array();
        let [v_x, v_y] = &mut self.velocity.as_mut_array();

        // println!("{:?}", (b_x, b_y));
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

        if *x < self.border_margin {
            *v_x += self.turn_factor;
        }
        if *x > b_x - self.border_margin {
            *v_x -= self.turn_factor;
        }
        if *y < self.border_margin {
            *v_y += self.turn_factor;
        }
        if *y > b_y - self.border_margin {
            *v_y -= self.turn_factor;
        }
    }

    pub fn reset_forces(&mut self) {
        self.separation_force = Vec2::v_space_zero();
        self.alignement_force = Vec2::v_space_zero();
        self.cohesion_force = Vec2::v_space_zero();

        self.alignement_number = 0.;
        self.cohesion_number = 0.;
        self.avg_color_nominator = Vec3::v_space_zero();
        self.avg_color_denominator = 0.;
    }

    const MIN_SPEED : f32 = 50.;

    pub fn apply_forces(&mut self, dt: f32) {
        let speed = self.velocity.length();
        if speed < Self::MIN_SPEED{
            if speed <= 0.{
                self.velocity = Vec2::from([Self::MIN_SPEED;2]) * 2.;
            }else{
                self.velocity = self.velocity.normalized() * Self::MIN_SPEED;
            }
        }

        self.position += self.velocity * dt;
    }

    pub fn handle_separation(&mut self, other: &mut Self) {
        if self.position.distance_sq(other.position) < self.separation * self.separation {
            self.separation_force += self.position - other.position;
            other.separation_force += other.position - self.position;

            //static collision
            if (self.position[0] - other.position[0]) * (self.position[0] - other.position[0])
                + (self.position[1] - other.position[1]) * (self.position[1] - other.position[1])
                < (self.size + other.size) * (self.size + other.size)
            {
                let dist = self.position.distance(other.position).max(0.001);

                let overlap = 0.5 * ((self.size + other.size) - dist).max(0.0);

                //2. resolve overlap
                self.position += (self.position - other.position) * overlap / dist;
                other.position -= (self.position - other.position) * overlap / dist;
            }
        }
    }
    pub fn apply_separation(&mut self, dt: f32) {
        self.velocity += self.separation_force * self.avoid_factor * dt;
    }

    pub fn handle_alignement(&mut self, other: &mut Self) {
        if self.position.distance_sq(other.position) < self.alignement * self.cohesion {
            self.alignement_number += 1.;
            self.alignement_force += other.velocity;

            other.alignement_number += 1.;
            other.alignement_force += self.velocity;
        }
    }

    pub fn apply_alignement(&mut self, dt: f32) {
        if self.alignement_number > 0. {
            let avg_vel = self.alignement_force / self.alignement_number;
            self.velocity += (avg_vel - self.velocity) * self.matching_factor * dt;
        }
    }

    pub fn handle_cohesion(&mut self, other: &mut Self) {
        if self.position.distance_sq(other.position) < self.cohesion * self.cohesion {
            self.cohesion_number += 1.;
            self.cohesion_force += other.position;

            other.cohesion_number += 1.;
            other.cohesion_force += self.position;
        }
    }

    pub fn apply_cohesion(&mut self, dt: f32) {
        if self.cohesion_number > 0. {
            let avg_pos = self.cohesion_force / self.cohesion_number;
            self.velocity += (avg_pos - self.position) * self.centering_factor * dt;
        }
    }

    pub fn handle_color(&mut self, other: &mut Self) {
        if self.position.distance_sq(other.position) < self.cohesion * self.cohesion {
            let dist = self.position.distance(other.position).max(0.00001);
            
            self.avg_color_nominator += other.avg_color / dist ;
            self.avg_color_denominator += dist.recip();

            other.avg_color_nominator += self.avg_color / dist;
            other.avg_color_denominator += dist.recip();
        }
    }

    pub fn apply_color(&mut self){
        if self.avg_color_denominator > 0.{   
            self.avg_color =(self.avg_color_nominator)/(self.avg_color_denominator);
        }else{
            self.avg_color = self.color;
        }

    }
}

impl As2dPoint<f32> for Boid {
    #[inline]
    fn x(&self) -> f32 {
        self.position[0]
    }

    #[inline]
    fn y(&self) -> f32 {
        self.position[1]
    }
}

fn hue_to_rgb(h: f32) -> [f32; 3] {
    let h = h % (2. * std::f32::consts::PI);
    let c = 1.0;
    let h_prime = h / (std::f32::consts::FRAC_PI_3);
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    match h_prime as u32 {
        0 => [c, x, 0.0],
        1 => [x, c, 0.0],
        2 => [0.0, c, x],
        3 => [0.0, x, c],
        4 => [x, 0.0, c],
        5 => [c, 0.0, x],
        _ => [1.0, 0.0, 0.0], // fallback (shouldn't happen)
    }
}