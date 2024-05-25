use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

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
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
