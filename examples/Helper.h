#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <iostream>
#include <memory>

std::ostream &operator<<(std::ostream &os, const glm::quat &q);
std::ostream &operator<<(std::ostream &os, const glm::vec3 &v);
