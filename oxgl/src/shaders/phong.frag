precision highp float;

uniform vec3 color;
uniform float ambient;
uniform float shininess;
uniform float specularStrength;

uniform vec3 cameraPosition;

uniform sampler2D shadowMap;
uniform bool shadowsEnabled;

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
varying vec4 vPosLightSpace;

float calculateShadow(vec4 posLightSpace) {
	if (!shadowsEnabled) return 0.0;
	
	vec3 projCoords = posLightSpace.xyz / posLightSpace.w;
	projCoords = projCoords * 0.5 + 0.5;
	
	if (projCoords.x < 0.0 || projCoords.x > 1.0 ||
		projCoords.y < 0.0 || projCoords.y > 1.0 ||
		projCoords.z > 1.0) {
		return 0.0;
	}
	
	float currentDepth = projCoords.z;
	float bias = 0.005;
	
	float shadow = 0.0;
	float texelSize = 1.0 / 1024.0;
	
	for (int x = -1; x <= 1; x++) {
		for (int y = -1; y <= 1; y++) {
			float pcfDepth = texture2D(shadowMap, projCoords.xy + vec2(float(x), float(y)) * texelSize).r;
			shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;
		}
	}
	shadow /= 9.0;
	
	return shadow;
}

vec3 calculateLight(Light light, vec3 normal, vec3 viewDir) {
	vec3 lightDir;
	float attenuation = 1.0;

	if (light.type == 0) {
		// Directional
		lightDir = normalize(-light.direction);
	} else if (light.type == 1) {
		// Point
		vec3 toLight = light.position - vWorldPos;
		float distance = length(toLight);
		lightDir = normalize(toLight);
		
		attenuation = clamp(1.0 - (distance / light.radius), 0.0, 1.0);
		attenuation *= attenuation;
	} else {
		// Spot
		vec3 toLight = light.position - vWorldPos;
		lightDir = normalize(toLight);
		float distance = length(toLight);
		attenuation = clamp(1.0 - (distance / light.radius), 0.0, 1.0);
	}

	float diff = max(dot(normal, lightDir), 0.0);
	vec3 diffuse = diff * light.color * light.intensity;

	vec3 halfDir = normalize(lightDir + viewDir);
	float spec = pow(max(dot(normal, halfDir), 0.0), shininess);
	vec3 specular = specularStrength * spec * light.color * light.intensity;

	return (diffuse + specular) * attenuation;
}

void main() {
	vec3 normal = normalize(vNormal);
	vec3 viewDir = normalize(cameraPosition - vWorldPos);

	float shadow = calculateShadow(vPosLightSpace);

	vec3 result = ambient * color;

	for (int i = 0; i < MAX_LIGHTS; i++) {
		if (i >= numLights) break;
		result += (1.0 - shadow) * calculateLight(lights[i], normal, viewDir) * color;
	}

	gl_FragColor = vec4(result, 1.0);
}