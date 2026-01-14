precision highp float;

uniform sampler2D screenTexture;
uniform vec2 resolution;
uniform float strength;

varying vec2 vUv;

void main() {
	vec2 offset = (strength / resolution) * (vUv - 0.5);

	float r = texture2D(screenTexture, vUv + offset).r;
	float g = texture2D(screenTexture, vUv).g;
	float b = texture2D(screenTexture, vUv - offset).b;

	gl_FragColor = vec4(r, g, b, 1.0);
}