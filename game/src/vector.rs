use fyrox::core::algebra::Vector3;

#[derive(Copy, Clone)]
pub struct Position {
    vector: Vector3<f32>,
}

impl Position {
    pub fn new(position: Vector3<f32>) -> Self {
        Self { vector: position }
    }

    pub fn vector(&self) -> Vector3<f32> {
        self.vector
    }
}

const COEFFICIENT_OF_ACCELERATION: f32 = 8.0;

#[derive(Copy, Clone)]
pub struct Velocity {
    vector: Vector3<f32>,
}

impl Velocity {
    pub fn new(velocity: Vector3<f32>) -> Self {
        Self { vector: velocity }
    }

    pub fn x(&self) -> f32 {
        self.vector.x
    }

    pub fn y(&self) -> f32 {
        self.vector.y
    }

    pub fn z(&self) -> f32 {
        self.vector.z
    }

    pub fn accelerated_vector(&self) -> Vector3<f32> {
        return Vector3::new(&self.vector.x * COEFFICIENT_OF_ACCELERATION, &self.vector.y * 1.0, &self.vector.z * COEFFICIENT_OF_ACCELERATION)
    }

    pub fn add_forward(&mut self, look_forward_vector: Vector3<f32>) {
        self.vector += look_forward_vector;
    }

    pub fn sub_backward(&mut self, look_backward_vector: Vector3<f32>) {
        self.vector -= look_backward_vector;
    }

    pub fn add_left(&mut self, look_side_vector: Vector3<f32>) {
        self.vector += look_side_vector;
    }

    pub fn add_right(&mut self, look_side_vector: Vector3<f32>) {
        self.vector -= look_side_vector;
    }

    pub fn add_vertical_up(&mut self, look_vertical_up: Vector3<f32>) {
        self.vector += look_vertical_up;
        self.vector.y += 1.0;
    }
}