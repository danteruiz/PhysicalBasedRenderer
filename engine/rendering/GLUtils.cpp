#include "GLUtils.h"

#include "GL.h"
namespace gl
{
void checkErrors() { auto error = glGetError(); }

GLenum toGLTextureFormat(Dimension dimension)
{
    GLenum glTextureFormat;

    switch (dimension)
    {
    case Dimension::Scalar:
        glTextureFormat = GL_RED;
        break;
    case Dimension::Vec2:
        glTextureFormat = GL_RG;
        break;
    case Dimension::Vec3:
        glTextureFormat = GL_RGB;
        break;

    case Dimension::Vec4:
    default:
        glTextureFormat = GL_RGBA;
        break;
    }

    return glTextureFormat;
}

GLenum toGLType(Type type)
{
    GLenum glType;
    switch (type)
    {
    case Type::UInt8:
        glType = GL_UNSIGNED_BYTE;
        break;

    case Type::UInt16:
        glType = GL_UNSIGNED_SHORT;
        break;

    case Type::UInt32:
        glType = GL_UNSIGNED_INT;
        break;
    case Type::Int8:
        glType = GL_BYTE;
        break;
    case Type::Int16:
        glType = GL_SHORT;
        break;
    case Type::Int32:
    default:
        glType = GL_INT;
        break;
    }

    return glType;
}
} // namespace gl
