use std::usize;

use godot::engine::{INode, Node};
use godot::prelude::*;
use shire_emblem_server_lib::false_matrix::FalseMatrix;

struct ShireEmblemLib;

#[gdextension]
unsafe impl ExtensionLibrary for ShireEmblemLib {}

#[derive(GodotClass)]
#[class(base=Node)]
struct ShireEmblemStatefulLibs {
    base: Base<Node>,
    tile_map: Option<FalseMatrix>,
}

#[godot_api]
impl INode for ShireEmblemStatefulLibs {
    fn init(base: Base<Node>) -> Self {
        ShireEmblemStatefulLibs {
            base: base,
            tile_map: None,
        }
    }
}

#[godot_api]
impl ShireEmblemStatefulLibs {
    #[func]
    fn test(&mut self) {
        godot_print!("this worked");
    }

    #[func]
    fn set_tile_map(&mut self, tile_map: Array<i32>, row_size: i32, column_size: i32) {
        let tile_map_vec: Vec<i32> = tile_map.iter_shared().map(|val| val).collect();
        self.tile_map = Some(FalseMatrix::new(
            tile_map_vec,
            row_size as usize,
            column_size as usize,
        ));
    }

    #[func]
    fn get_player_range(&mut self) -> Array<i32> {
        match &self.tile_map {
            Some(x) => {
                let col_size = x.cols;

                let row_size = x.rows;

                let mut output_map: Array<i32> = array![];

                for i in 0..col_size {
                    for j in 0..row_size {
                        output_map.push(x[(j as usize, i as usize)]);
                    }
                }
                godot_print!("{:?}", x);
                output_map
            }
            None => panic!("tile map has not been set yet"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
