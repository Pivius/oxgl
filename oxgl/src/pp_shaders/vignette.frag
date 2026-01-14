precision highp float;

uniform sampler2D screenTexture;
uniform float intensity;
uniform float smoothness;

varying vec2 vUv;

void main() {
	vec4 color = texture2D(screenTexture, vUv);

	vec2 center = vUv - 0.5;
	float dist = length(center);
	float vignette = smoothstep(0.5, 0.5 - smoothness, dist * intensity);

	gl_FragColor = vec4(color.rgb * vignette, 1.0);
}