extends Node2D

var statefulLibs = null

# Called when the node enters the scene tree for the first time.
func _ready():
	statefulLibs = ShireEmblemStatefulLibs.new();
	get_tree().change_scene_to_file("res://scenes/debug/debug_level.tscn");
	
	#pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	#var array: Array[int] = [1, 2, 3, 4];
	#statefulLibs.set_tile_map(array, 2, 2);
	#print(statefulLibs.get_player_range());
	pass
