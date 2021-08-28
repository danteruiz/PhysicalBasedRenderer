#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

class ICamera
{
public:
    ICamera();
    glm::mat4 getViewMatrix() const;
    glm::vec3 getViewPosition() const;
    void rotateAroundTarget(bool enable) { m_rotateAroundTarget = enable; }
    bool isRotatingAroundTarget() const { return m_rotateAroundTarget; }
    void update(float deltaTime);

protected:
    glm::vec3 position;
    glm::quat orientation;
    bool m_rotateAroundTarget {false};
};
