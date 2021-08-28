#include "Camera.h"

//#include <Input.h>

ICamera::ICamera() {}

glm::mat4 ICamera::getViewMatrix() const { return glm::mat4(); }

glm::vec3 ICamera::getViewPosition() const { return glm::vec3(); }

void ICamera::update(float deltaTime) {}
