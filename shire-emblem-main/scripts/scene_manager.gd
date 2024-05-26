extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready():
	var node = ShireEmblemStaticLibs.new();
	node.test();
	var array: Array[int] = [1, 2, 3];
	
	print(node.array_test(array, 2, 3));
	#pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
