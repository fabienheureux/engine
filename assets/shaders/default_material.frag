#version 330 core
layout(std140) uniform;

out vec4 FragColor;

in vec2 TexCoords;
in vec3 Normal;
in vec3 FragPos;

struct Light {
  int kind;
  vec3 direction;
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct Material {
  sampler2D diffuse;
  vec3 specular;
  float shininess;
};

uniform Material material;

uniform Camera {
  mat4 projection;
  mat4 view;
  vec3 cam_pos;
};

#define MAX_LIGHTS_COUNT 30
uniform Lights {
  Light light[MAX_LIGHTS_COUNT];
} lights;

vec3 computeSunLight(Light current, vec3 normal, vec3 viewDir) {
	vec3 lightDir = normalize(current.position - FragPos);

	// Diffuse shading...
	float diff = max(dot(normal, lightDir), 0.0);
	// Specular shading...
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	// Combine results...
	vec3 diffuse = current.diffuse * diff * vec3(texture(material.diffuse, TexCoords));
	vec3 ambient = current.ambient * vec3(texture(material.diffuse, TexCoords));
	vec3 specular = current.specular * spec * material.specular;

	return (ambient + diffuse + specular);
}

vec3 computeDirectionalLight(Light current, vec3 normal, vec3 viewDir) {
	vec3 lightDir = normalize(-current.direction);

	// Diffuse shading...
	float diff = max(dot(normal, lightDir), 0.0);
	// Specular shading...
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	// Combine results...
	vec3 diffuse = current.diffuse * diff * vec3(texture(material.diffuse, TexCoords));
	vec3 ambient = current.ambient * vec3(texture(material.diffuse, TexCoords));
	vec3 specular = current.specular * spec * material.specular;

	return (ambient + diffuse + specular);
}

void main() {
  vec3 norm = normalize(Normal);
	vec3 viewDir = normalize(cam_pos - FragPos);

	vec3 result = vec3(0.);

	for(int l_index = 0; l_index < MAX_LIGHTS_COUNT; l_index++) {
		Light current = lights.light[l_index];

		if (current.kind == 1) {
			result += computeSunLight(current, norm, viewDir);
		} else if (current.kind == 2) {
			result += computeDirectionalLight(current, norm, viewDir);
		}

	}

	FragColor = vec4(result, 1.);
}
