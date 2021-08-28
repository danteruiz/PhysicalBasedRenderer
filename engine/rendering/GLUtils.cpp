#include "GLUtils.h"

#include "GL.h"
namespace gl
{
void checkErrors() { auto error = glGetError(); }
} // namespace gl
