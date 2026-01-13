precision mediump float;

uniform vec3 color;
uniform float ambient;

uniform vec3 lightDirection;
uniform vec3 lightColor;
uniform float lightIntensity;

varying vec3 vNormal;
varying vec3 vWorldPos;

void main() {
	vec3 normal = normalize(vNormal);
	vec3 lightDir = normalize(-lightDirection);
	
	float diff = max(dot(normal, lightDir), 0.0);
	vec3 diffuse = diff * lightColor * lightIntensity;
	
	vec3 result = (ambient + diffuse) * color;
	gl_FragColor = vec4(result, 1.0);
}