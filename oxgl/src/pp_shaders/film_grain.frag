precision highp float;

uniform sampler2D screenTexture;
uniform float time;
uniform float intensity;

varying vec2 vUv;

float random(vec2 co) {
	return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
	vec4 color = texture2D(screenTexture, vUv);

	float grain = random(vUv + time) * 2.0 - 1.0;
	color.rgb += grain * intensity;

	gl_FragColor = vec4(clamp(color.rgb, 0.0, 1.0), 1.0);
}