use std::usize;

use godot::engine::{INode2D, Node2D};
use godot::prelude::*;
use shire_emblem_server_lib::false_matrix::FalseMatrix;

use std::ops::{Index, IndexMut};

struct ShireEmblemLib;

#[gdextension]
unsafe impl ExtensionLibrary for ShireEmblemLib {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct ShireEmblemStaticLibs {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for ShireEmblemStaticLibs {
    fn init(base: Base<Node2D>) -> Self {
        ShireEmblemStaticLibs { base }
    }
}

#[godot_api]
impl ShireEmblemStaticLibs {
    #[func]
    fn test(&mut self) {
        godot_print!("this worked");
    }

    #[func]
    fn array_test(&mut self, tile_map: Array<i32>, row_size: i32, column_size: i32) -> Array<i32> {
        let tile_map_vec: Vec<i32> = tile_map.iter_shared().map(|val| val).collect();

        let matrix: FalseMatrix =
            FalseMatrix::new(tile_map_vec, row_size as usize, column_size as usize);

        let mut output_map: Array<i32> = array![];

        for x in 0..column_size {
            for y in 0..row_size {
                output_map.push(matrix[(y as usize, x as usize)]);
            }
        }

        godot_print!("{}, {}, {}", tile_map, row_size, column_size);

        output_map
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
