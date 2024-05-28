extends Node2D

@export var tile_map: TileMap

# Called when the node enters the scene tree for the first time.
func _ready():
	var rect: Rect2i = tile_map.get_used_rect() as Rect2i;
	var size_vect: Vector2i = rect.size as Vector2i;
	print(size_vect.x);
	print(size_vect.y);
	
	
	var tile_array: Array[int] = []
	
	for i in range(size_vect.x * size_vect.y):
		tile_array.push_back(-1)
	
	for i in range(size_vect.x):
		for j in range(size_vect.y):
			var tile_data = tile_map.get_cell_tile_data(0, Vector2i(i, j), false) as TileData
			if tile_data != null :
				tile_array[j * size_vect.x + i] = 0
				
	print(tile_array)
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
