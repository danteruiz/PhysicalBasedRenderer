#pragma once

#include <string>
#include <memory>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
class Shader
{
public:
    using Pointer = std::shared_ptr<Shader>;
    Shader(std::string const &fragmentSource, std::string const &vertexSource, std::string const& defines = "");
    int getID() const { return m_id; }

    void bind() const;
    void setUniformMat4(std::string const &name, glm::mat4 const &matrix) const;
    void setUniformVec4(std::string const &name, glm::vec4 const &vec) const;
    void setUniformVec3(std::string const &name, glm::vec3 const &vec) const;
    void setUniform1f(std::string const &name, float f) const;
    void setUniform1i(std::string const &name, int i) const;

private:
    int m_id;
};
