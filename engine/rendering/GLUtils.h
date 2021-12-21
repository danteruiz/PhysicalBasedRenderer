/*
 * GLUtils.h
 *
 * Created on 2021/12/20 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "Format.h"
#include "GL.h"

namespace gl
{
void checkErrors();

inline GLenum toGLTextureFormat(Dimension dimension);
inline GLenum toGLType(Type type);
} // namespace gl
