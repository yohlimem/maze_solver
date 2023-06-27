use nannou::{prelude::*, draw, image::math};
use rand::seq::index;

#[derive(Debug, Clone, Copy)]
pub enum Sides{
    Right,
    Left,
    Downward,
    Upward,
}

const SQUARE_SIZE:u32 = 4;

pub struct Grid{
    pub size: u32,
    pub filled: Vec<i8>,
}

pub struct Agent{
    pub rules: [Sides; 4],
    pub pos: Vec2,
    last_pos: Vec2,
    pub current_side: i32, // 0 = up, right = 1, left = 2, 3 = down
    last_side: i32, // 0 = up, right = 1, left = 2, 3 = down

}




impl Grid{
    pub fn generate_grid(&mut self){
        for i in 0..(self.size * self.size) as usize{
            self.filled.push(-1);
        }
    }

    pub fn draw_grid(&self, draw: &Draw){

        // draw grid
        for i in 0..(self.size * self.size){
            // find position using the index
            let x = (i % self.size) * (self.size * SQUARE_SIZE);
            let y = (i.div_euclid(self.size)) * (self.size * SQUARE_SIZE);

            match self.filled[i as usize] {
                -1 => draw.rect().color(WHITE).h((self.size * SQUARE_SIZE) as f32).w((self.size * SQUARE_SIZE) as f32).x_y(x as f32, y as f32),
                // up and down
                1 => draw.rect().color(BLACK).h((self.size * SQUARE_SIZE) as f32).w((self.size * SQUARE_SIZE) as f32 / 2.0).x_y(x as f32, y as f32),
                3 => draw.rect().color(BLACK).h((self.size * SQUARE_SIZE) as f32).w((self.size * SQUARE_SIZE) as f32 / 2.0).x_y(x as f32, y as f32),
                // right and left
                2 => draw.rect().color(BLACK).h((self.size * SQUARE_SIZE) as f32 / 2.0).w((self.size * SQUARE_SIZE) as f32).x_y(x as f32, y as f32),
                0 => draw.rect().color(BLACK).h((self.size * SQUARE_SIZE) as f32 / 2.0).w((self.size * SQUARE_SIZE) as f32).x_y(x as f32, y as f32),
                // everything else
                _ => draw.rect().color(BLACK).h((self.size * SQUARE_SIZE) as f32 / 2.0).w((self.size * SQUARE_SIZE) as f32 / 2.0).x_y(x as f32, y as f32),
            };
        }
    }
}
impl Agent{

    pub fn new(rules: [Sides; 4]) -> Agent{
        Agent{
            current_side: 0,
            last_side: 0,
            last_pos: Vec2::ZERO,
            pos: Vec2::ZERO,
            rules,
        }
    }


    pub fn step(&mut self, grid: &mut Grid){
        // move according to first rules

        for i in 0..4{
            
            let current_generated_side: Vec2 = self.unpack_rules_index(i, true).unwrap_or(Vec2::ZERO);
            let current_rule_side: Vec2 = self.unpack_rules_index(i, true).unwrap_or(Vec2::ZERO);
            // find the new pos
            let check = self.pos + current_generated_side;
            let last_check = self.pos;
            
            // find its index in the grid
            let index = (check.y * grid.size as f32 + check.x) as usize;
            let last_index = (last_check.y * grid.size as f32 + last_check.x) as usize;
            
            
            // check if your not out of bounds
            if (check.x < 0.0 || check.y < 0.0) || (check.x >= grid.size as f32 || check.y >= grid.size as f32) {
                continue;
            } else if grid.filled[index] != -1 { // check if the current grid location is not filled
                continue;
            }
            // set current side
            // println!( "current side: {:?}, real side: {:?}, generated side: {:?}", Agent::convert_number_to_side(self.current_side), self.rules[i as usize], Agent::convert_number_to_side(Agent::convert_vec_to_number(self.unpack_rules_index(i, true).unwrap())));
            println!("current side: {:?}, real side: {:?}, generated side: {:?}", Agent::convert_number_to_side(self.current_side), self.rules[i as usize], Agent::convert_number_to_side(Agent::convert_vec_to_number(self.unpack_rules_index(i, true).unwrap())));
            
            if self.current_side == self.last_side {
                grid.filled[last_index] = self.last_side as i8;
            }
            self.last_side = self.current_side;
            self.current_side = Agent::convert_vec_to_number(current_rule_side);
            //                                                                                                             false because more interesting
            // change last position to the current self.pos
            self.last_pos = self.pos;
            // set position to the new one
            self.pos = check;
            // println!("check: {}, filled: {}", check, grid.filled[index]);
            // println!("");
            // println!("vec to num: {}, no vec to num: {:?}",  Agent::convert_vec_to_number(self.unpack_rules_index(i, true).unwrap_or(Vec2::ZERO)), self.unpack_rules_index(i, true).unwrap_or(Vec2::ZERO));
            // grid.filled[index] = current_generated_side;
            // TODO: fix bug where it doesnt go right.
            
            grid.filled[index] = 4;


            // break so you don't move more than once
            break;
            

        }
    }

    pub fn unpack_rules_index(&self, rule_num: u32, procedural_dir: bool) -> Option<Vec2>{

        // dir var
        let mut dir:Sides;
        
        if procedural_dir {
            // if you want the sides to change depending on the look dir use this
            dir = self.find_sides_by_direction(&self.rules[rule_num as usize]);
        } else {
            // else don't
            dir = self.rules[rule_num as usize].clone();
        }
        
        // find the move dir
        // match = switch in other languages
        let place = match dir {
            Sides::Right => Some(vec2(1.,0.)),
            Sides::Left => Some(vec2(-1.,0.)),
            Sides::Upward => Some(vec2(0.,1.)),
            Sides::Downward => Some(vec2(0.,-1.)),
        };
        
        place
    }

    pub fn find_sides_by_direction(&self, input_dir: &Sides) -> Sides{

        /*
            explanation:
            think about the sides like a clock with four number 0, 1, 2 ,3

            now if we take the current side let's say 3
            and add it to the side we want to get let's say 2
            we will get 3 + 2 = 5, 5 % 4 = 1
            (basically arrows thats spin)
            lets take the right direction:
                the "up" of right is right.
                the "down" of right is left.
                the "left" of right is up.
            
            another way to think of it is to point right
            then tilt your head right and try to think of the directions like that
        
        */ 


        // convert side to number
        let input_dir_num = Agent::convert_side_to_number(&input_dir);

        // take the number of the dir your in right now and use modulo it by three (4 sides thats start from zero)
        match (self.current_side + input_dir_num) % 4 {
            0 => Sides::Upward,
            1 => Sides::Right,
            2 => Sides::Downward,
            3 => Sides::Left,
            _ => Sides::Upward,
        }
        
        // (right (3) - left(1)) % 3 = 2
    }

    pub fn convert_side_to_number(input_dir: &Sides) -> i32{
        match input_dir {
            Sides::Upward => 0,
            Sides::Right => 1,
            Sides::Downward => 2,
            Sides::Left => 3,
        }
    }
    pub fn convert_number_to_side(input_dir: i32) -> Sides{
        match input_dir{
            0 => Sides::Upward,
            1 => Sides::Right,
            2 => Sides::Downward,
            3 => Sides::Left,
            _ => Sides::Upward,
        }
    }
    pub fn convert_number_to_vec(input_dir: i32) -> Vec2{
        match input_dir{
            0 => vec2(1.,0.),// Sides::Upward,
            1 => vec2(-1.,0.),// Sides::Right,
            2 => vec2(0.,1.),// Sides::Upward,
            3 => vec2(0.,-1.), // Sides::Left,
            _ => Vec2::ZERO // Sides::Upward,
        }
    }
    pub fn convert_vec_to_number(input_dir: Vec2) -> i32{
        if input_dir == vec2(0.,1.) {
            0 // Sides::Upward,
        } else if input_dir == vec2(1.,0.) {
            1 // Sides::Right,

        } else if input_dir == vec2(0.,-1.) {
            2 // Sides::down,

        } else if input_dir == vec2(-1.,0.) {
            3 // Sides::Left,

        } else {
            0 // Sides::Upward,
        }
    }

}

