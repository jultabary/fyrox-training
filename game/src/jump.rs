use fyrox::core::log::Log;
use crate::vector::{Position, Velocity};

pub const MAX_Y_JUMP: f32 = 0.5;
pub const MIN_Y_TO_CONSIDER_FALLING: f32 = -0.1;
pub const MIN_Y_TO_CONSIDER_JUMPING: f32 = 0.1;
pub const NUMBER_OF_CYCLE_TO_CONSIDER_NOT_FALLING: i8 = 2;

pub struct Jump {
    jump_origin_position_opt: Option<Position>,
    jump_origin_velocity_opt: Option<Velocity>,
    number_of_cycle_player_is_not_falling: i8,
    can_move: bool,
}

impl Jump {
    pub fn new() -> Self {
        Self {
            jump_origin_position_opt: None,
            jump_origin_velocity_opt: None,
            number_of_cycle_player_is_not_falling: 0,
            can_move: true,
        }
    }

    pub fn update_jump(&mut self, velocity: &Velocity) {
        if let Some(_) = self.jump_origin_velocity_opt {
            self.can_move = false;
            if !self.is_falling(velocity) && !self.is_upping(velocity) {
                self.number_of_cycle_player_is_not_falling += 1;
            }
            if self.number_of_cycle_player_is_not_falling == NUMBER_OF_CYCLE_TO_CONSIDER_NOT_FALLING {
                self.number_of_cycle_player_is_not_falling = 0;
                Log::info("reset jump");
                self.reset_jump();
                self.can_move = true;
            }
        } else {
            self.can_move = true;
        }
    }

    pub fn can_move(&self) -> bool {
        self.can_move
    }

    pub fn is_falling(&self, velocity: &Velocity) -> bool {
        if velocity.y() < MIN_Y_TO_CONSIDER_FALLING { true } else { false }
    }

    pub fn is_upping(&self, velocity: &Velocity) -> bool {
        if velocity.y() > MIN_Y_TO_CONSIDER_JUMPING { true } else { false }
    }

    pub fn is_jump_initiated(&self) -> bool {
        self.jump_origin_velocity_opt.is_some()
    }

    pub fn can_jump(&mut self, current_position: &Position, velocity: &Velocity) -> bool {
        if self.is_falling(velocity) {
            return false;
        }
        if let Some(jump_origin_position) = &self.jump_origin_position_opt {
            let delta_y = current_position.vector().y - jump_origin_position.vector().y;
            return if delta_y < MAX_Y_JUMP { true } else { false }
        }
        true
    }

    pub fn jump(&mut self, current_position: &Position, velocity: &Velocity) {
        self.set_jump_origin_position(&current_position);
        self.set_jump_origin_velocity(&velocity);
    }

    pub fn reset_jump(&mut self) {
        self.jump_origin_position_opt = None;
        self.jump_origin_velocity_opt = None;
    }

    pub fn set_jump_origin_position(&mut self, position: &Position) {
        if self.jump_origin_position_opt.is_none() {
            self.jump_origin_position_opt = Some((*position).clone());
        }
    }

    pub fn set_jump_origin_velocity(&mut self, velocity: &Velocity) {
        if self.jump_origin_velocity_opt.is_none() {
            self.jump_origin_velocity_opt = Some((*velocity).clone());
        }
    }

    pub fn jumping_velocity(&mut self) -> Result<Velocity, ()> {
        if let Some(jumping_velocity) = self.jump_origin_velocity_opt.as_ref() {
            return Ok((*jumping_velocity).clone());
        }
        return Err(());
    }
}