extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready():
	var node = ShireEmblemStatefulLibs.new();
	node.test();
	var array: Array[int] = [1, 2, 3, 4];
	node.set_tile_map(array, 2, 2);
	print(node.get_player_range());
	
	#pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
