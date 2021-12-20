#pragma once

#include "GL.h"

#include <stdint.h>
class GLObject
{
public:
    enum class Type : uint8_t
    {
        Buffer,
        Texture,
        Framebuffer
    };

    GLObject(Type type);
    ~GLObject();

    Type getType() const { return m_type; }
    GLuint getId() const { return m_id; }

protected:
    Type m_type;
    GLuint m_id;
};
