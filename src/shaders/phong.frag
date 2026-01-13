precision mediump float;

// Material properties
uniform vec3 color;
uniform float ambient;
uniform float shininess;
uniform float specularStrength;

// Camera
uniform vec3 cameraPosition;

// Light types: 0 = directional, 1 = point, 2 = spot
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

vec3 calculateLight(Light light, vec3 normal, vec3 viewDir) {
	vec3 lightDir;
	float attenuation = 1.0;

	if (light.type == 0) {
		// Directional light
		lightDir = normalize(-light.direction);
	} else if (light.type == 1) {
		// Point light
		vec3 toLight = light.position - vWorldPos;
		float distance = length(toLight);
		lightDir = normalize(toLight);
		
		// Attenuation based on radius
		attenuation = clamp(1.0 - (distance / light.radius), 0.0, 1.0);
		attenuation *= attenuation; // Quadratic falloff
	} else {
		// Spot light (simplified)
		vec3 toLight = light.position - vWorldPos;
		lightDir = normalize(toLight);
		float distance = length(toLight);
		attenuation = clamp(1.0 - (distance / light.radius), 0.0, 1.0);
	}

	// Diffuse
	float diff = max(dot(normal, lightDir), 0.0);
	vec3 diffuse = diff * light.color * light.intensity;

	// Specular (Blinn-Phong)
	vec3 halfDir = normalize(lightDir + viewDir);
	float spec = pow(max(dot(normal, halfDir), 0.0), shininess);
	vec3 specular = specularStrength * spec * light.color * light.intensity;

	return (diffuse + specular) * attenuation;
}

void main() {
	vec3 normal = normalize(vNormal);
	vec3 viewDir = normalize(cameraPosition - vWorldPos);

	// Start with ambient
	vec3 result = ambient * color;

	// Add contribution from each light
	for (int i = 0; i < MAX_LIGHTS; i++) {
		if (i >= numLights) break;
		result += calculateLight(lights[i], normal, viewDir) * color;
	}

	gl_FragColor = vec4(result, 1.0);
}