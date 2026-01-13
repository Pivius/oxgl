precision mediump float;

uniform vec3 color;
uniform float ambient;

const int MAX_LIGHTS = 4;

struct Light {
	int type;
	vec3 position;
	vec3 direction;
	vec3 color;
	float intensity;
	float radius;
};

uniform int numLights;
uniform Light lights[MAX_LIGHTS];

varying vec3 vNormal;
varying vec3 vWorldPos;

vec3 calculateLight(Light light, vec3 normal) {
	vec3 lightDir;
	float attenuation = 1.0;

	if (light.type == 0) {
		lightDir = normalize(-light.direction);
	} else {
		vec3 toLight = light.position - vWorldPos;
		float distance = length(toLight);
		lightDir = normalize(toLight);
		attenuation = clamp(1.0 - (distance / light.radius), 0.0, 1.0);
		attenuation *= attenuation;
	}

	float diff = max(dot(normal, lightDir), 0.0);
	return diff * light.color * light.intensity * attenuation;
}

void main() {
	vec3 normal = normalize(vNormal);
	vec3 result = ambient * color;

	for (int i = 0; i < MAX_LIGHTS; i++) {
		if (i >= numLights) break;
		result += calculateLight(lights[i], normal) * color;
	}

	gl_FragColor = vec4(result, 1.0);
}