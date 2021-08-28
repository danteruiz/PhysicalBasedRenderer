/*
 * Shaders.h
 *
 * Created on 2021/08/24 by Enter Your Name Here
 * Copyright 2021 Enter Your Name Here
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "GL.h"

#include <string>
#include <vector>
namespace shader
{
bool compileShader(GLenum shaderType, const std::string &shaderSource,
                   GLuint &programObject, std::string &message);
GLuint buildProgram(const std::vector<GLuint> &shaders);
bool linkProgram(GLuint shaderProgram, std::string &message);
} // namespace shader
