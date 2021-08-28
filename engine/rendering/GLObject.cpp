#include "GLObject.h"

GLObject::GLObject(Type type) : m_type(type)
{
    switch (type)
    {
    case Type::Buffer:
        glGenBuffers(1, &m_id);
        break;
    case Type::Texture:
        glGenTextures(1, &m_id);
        break;
    case Type::Framebuffer:
        glGenFramebuffers(1, &m_id);
        break;
    default:
        // we so not support this type
        break;
    }
}

GLObject::~GLObject()
{
    switch (m_type)
    {
    case Type::Buffer:
        glDeleteBuffers(1, &m_id);
        break;
    case Type::Texture:
        glDeleteTextures(1, &m_id);
        break;
    case Type::Framebuffer:
        glDeleteFramebuffers(1, &m_id);
        break;
    default:
        // we so not support this type
        break;
    }
}
