#include "PathUtils.h"

namespace PathUtils
{
std::string getResourcePath() { return RESOURCE_PATH; }

std::string getShaderUrl(const std::string &shaderName)
{
    return getResourcePath() + "shaders/" + shaderName;
}

std::string getModelUrl(const std::string &modelName)
{
    return getResourcePath() + "glTF-Models/" + modelName;
}
} // namespace PathUtils
