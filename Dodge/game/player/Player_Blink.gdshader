shader_type canvas_item;
uniform float effect=1;
uniform bool reffect=false;
void vertex() {
	
	// Called for every vertex the material is visible on.
}

void fragment() {
	
	COLOR.a=min(COLOR.a,max(((sin(2.0*TIME*TAU)+1.0)/2.0),effect));
	//COLOR.rg=vec2(min(sin((TIME),COLOR.r))),cos((TIME)));
	if (reffect){
		COLOR.rg=vec2(max(sin(4.0*TIME+UV.x),COLOR.r),max(cos(4.0*TIME+UV.y),COLOR.g));
	}
	
	// Called for every pixel the material is visible on.
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
