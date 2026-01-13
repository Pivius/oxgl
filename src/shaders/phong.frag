precision mediump float;

uniform vec3 color;
uniform float ambient;
uniform float shininess;
uniform float specularStrength;

uniform vec3 lightDirection;
uniform vec3 lightColor;
uniform float lightIntensity;

varying vec3 vNormal;
varying vec3 vWorldPos;
varying vec3 vViewDir;

void main() {
	vec3 normal = normalize(vNormal);
	vec3 viewDir = normalize(vViewDir);
	vec3 lightDir = normalize(-lightDirection);
	
	// Diffuse
	float diff = max(dot(normal, lightDir), 0.0);
	vec3 diffuse = diff * lightColor * lightIntensity;
	
	// Specular (Blinn-Phong)
	vec3 halfDir = normalize(lightDir + viewDir);
	float spec = pow(max(dot(normal, halfDir), 0.0), shininess);
	vec3 specular = specularStrength * spec * lightColor * lightIntensity;
	
	vec3 result = (ambient + diffuse) * color + specular;
	gl_FragColor = vec4(result, 1.0);
}