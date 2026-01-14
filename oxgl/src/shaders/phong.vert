attribute vec3 position;
attribute vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 lightSpace;
uniform bool shadowsEnabled;

varying vec3 vNormal;
varying vec3 vWorldPos;
varying vec4 vPosLightSpace;

void main() {
	vec4 worldPos = model * vec4(position, 1.0);
	vWorldPos = worldPos.xyz;
	vNormal = mat3(model) * normal;
	
	if (shadowsEnabled) {
		vPosLightSpace = lightSpace * worldPos;
	} else {
		vPosLightSpace = vec4(0.0);
	}

	gl_Position = projection * view * worldPos;
}