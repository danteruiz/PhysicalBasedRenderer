#include "Helper.h"

#include <Model.h>
#include <Texture.h>

std::ostream &operator<<(std::ostream &os, const glm::quat &q)
{
    os << "w: " << q.w;
    os << " x: " << q.x;
    os << " y: " << q.y;
    os << " z: " << q.z;

    return os;
}

std::ostream &operator<<(std::ostream &os, const glm::vec3 &v)
{
    os << "x: " << v.x;
    os << " y: " << v.y;
    os << " z: " << v.z;

    return os;
}
