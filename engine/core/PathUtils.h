#pragma once
#include <string>

namespace PathUtils
{
std::string getResourcePath();
std::string getShaderUrl(const std::string &shaderName);
std::string getModelUrl(const std::string &modelName);
} // namespace PathUtils
