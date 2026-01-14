precision highp float;

uniform sampler2D screenTexture;
uniform vec2 resolution;
uniform int radius;

varying vec2 vUv;

void main() {
	vec2 texelSize = 1.0 / resolution;
	vec3 result = vec3(0.0);
	float count = 0.0;

	for (int x = -4; x <= 4; x++) {
		for (int y = -4; y <= 4; y++) {
			if (abs(x) <= radius && abs(y) <= radius) {
				vec2 offset = vec2(float(x), float(y)) * texelSize;
				result += texture2D(screenTexture, vUv + offset).rgb;
				count += 1.0;
			}
		}
	}

	gl_FragColor = vec4(result / count, 1.0);
}