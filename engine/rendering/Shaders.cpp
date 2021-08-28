/*
 * Shaders.cpp
 *
 * Created on 2021/08/24 by Enter Your Name Here
 * Copyright 2021 Enter Your Name Here
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Shaders.h"

#include <iostream>

namespace shader
{

std::string glShaderTypeToString(GLenum shaderType)
{
    std::string shaderName;

    switch (shaderType)
    {
    case GL_VERTEX_SHADER:
        shaderName = "VERTEX";
        break;

    case GL_FRAGMENT_SHADER:
        shaderName = "FRAGMENT";
        break;

    default:
        break;
    }

    return shaderName;
}

bool compileShader(GLenum shaderType, const std::string &shaderSource,
                   GLuint &programObject, std::string &message)
{
    programObject = glCreateShader(shaderType);

    if (!programObject)
    {
        std::cout << "Failed to create shader program \n";
        return false;
    }

    GLchar *source = (GLchar *)shaderSource.c_str();
    glShaderSource(programObject, 1, &source, NULL);
    glCompileShader(programObject);

    GLint compiled = 0;
    glGetShaderiv(programObject, GL_COMPILE_STATUS, &compiled);

    GLint infoLength = 0;
    glGetShaderiv(programObject, GL_INFO_LOG_LENGTH, &infoLength);
    char *infoLog = new char[infoLength];
    glGetShaderInfoLog(programObject, infoLength, NULL, infoLog);

    bool success = true;
    if (!compiled)
    {
        std::cout << "ERROR::SHADER::COMPILATION_FAILED\n"
                  << infoLog << std::endl;
        success = false;
    }

    delete[] infoLog;
    return success;
}

GLuint buildProgram(const std::vector<GLuint> &shaders)
{
    GLuint glprogram = glCreateProgram();

    for (auto shader : shaders)
    {
        glAttachShader(glprogram, shader);
    }

    return glprogram;
}

bool linkProgram(GLuint shaderProgram, std::string &message)
{
    glLinkProgram(shaderProgram);

    GLint linked = 0;
    glGetProgramiv(shaderProgram, GL_LINK_STATUS, &linked);

    if (!linked)
    {
        std::cout << "shader not linked \n";
        return false;
    }
    return true;
}
} // namespace shader
