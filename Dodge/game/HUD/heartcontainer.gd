extends HBoxContainer


@export var scene:PackedScene;

# Called when the node enters the scene tree for the first time.
func _ready():
	on_health_change(1)

# Called every frame. 'delta' is the elapsed time since the previous frame.
# guys this is a quick patch. not essential so not written in rust
func on_health_change(health_num:int):
	for i in get_children():
		i.queue_free()
	for i in health_num:
		var hscene=scene.instantiate()
		add_child(hscene)
	
