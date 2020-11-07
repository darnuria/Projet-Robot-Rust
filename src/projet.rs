use std::str::FromStr;

#[derive(Debug, PartialEq)]
//enumeration Orientation avec les 4 cas possibles
pub enum Orientation {
    Right,
    Left,
    Move,
    Nothing,
}

// fonction qui lit un caractères char et renvoie une instruction
pub fn function_instruction(instruction: char) -> Orientation {
    match instruction {
        'R' => Orientation::Right,
        'L' => Orientation::Left,
        'F' => Orientation::Move,
        _ => Orientation::Nothing,
    }
}

#[derive(Debug, PartialEq)]
// structure du robot
pub struct Robot {
    pub id: i32,
    pub position_en_x: usize,
    pub position_en_y: usize,
    pub instruction: String,
    pub command: Vec<char>,
}

// creation d'autres robots
fn autres_Robots(
    id: i32,
    position_en_x: usize,
    position_en_y: usize,
    instruction: String,
    command: Vec<char>,
) -> Robot {
    Robot {
        id: id,
        position_en_x: position_en_x,
        position_en_y: position_en_y,
        instruction: instruction,
        command: command,
    }
}

// fonction qui gère les cas de collisions avec un autre robot,
// en indiquant les coordonnées x et y de la collision
pub fn collisions(grid: &mut Vec<Vec<char>>, robot_vecteur: &Vec<Robot>) {
    for i in 0..robot_vecteur.len() {
        for j in 0..robot_vecteur.len() {
            if (robot_vecteur[i].id != robot_vecteur[j].id)
                && (robot_vecteur[i].position_en_x == robot_vecteur[j].position_en_x)
                && (robot_vecteur[i].position_en_y == robot_vecteur[j].position_en_y)
            {
                println!(
                    "Robot ID<{}> Collision en ({}, {})",
                    robot_vecteur[i].id,
                    robot_vecteur[i].position_en_x,
                    robot_vecteur[i].position_en_y
                );
            }
        }
    }
}
