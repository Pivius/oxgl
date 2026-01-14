precision highp float;

uniform sampler2D screenTexture;

varying vec2 vUv;

void main() {
	vec4 color = texture2D(screenTexture, vUv);
	gl_FragColor = vec4(1.0 - color.rgb, 1.0);
}