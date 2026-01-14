precision highp float;

uniform sampler2D screenTexture;
uniform vec2 resolution;
uniform float pixelSize;

varying vec2 vUv;

void main() {
	vec2 pixels = resolution / pixelSize;
	vec2 uv = floor(vUv * pixels) / pixels;
	gl_FragColor = texture2D(screenTexture, uv);
}