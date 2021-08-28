/*
 * Texture.cpp
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Texture.h"

#include <cassert>
#include <iostream>

#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "GL.h"
#include <stb_image.h>
#include <stb_image_write.h>
#include "spdlog/spdlog.h"

std::shared_ptr<Texture> createTextureFromGLTF(int width, int height,
                                               int component, int bits,
                                               unsigned char *data)
{
    std::shared_ptr<Texture> texture = std::make_shared<Texture>();
    glGenTextures(1, &texture->m_id);
    glBindTexture(GL_TEXTURE_2D, texture->m_id);

    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    GLenum format = GL_RGBA;
    if (component == 1)
    {
        format = GL_RED;
    }
    else if (component == 2)
    {
        format = GL_RG;
    }
    else if (component == 3)
    {
        format = GL_RGB;
    }

    GLenum type = GL_UNSIGNED_BYTE;
    if (bits == 8)
    {
        // ok
    }
    else if (bits == 16)
    {
        type = GL_UNSIGNED_SHORT;
    }

    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, format, type,
                 data);

    return texture;
}

inline GLenum getGLTextureFormat(int components)
{
    GLenum format = GL_RGBA;

    switch (components)
    {
        case 1:
            format = GL_RED;
            break;
        case 2:
            format = GL_RG;
            break;
        case 3:
            format = GL_RGB;
            break;

        default:
            break;
    }
    return format;
}

inline GLenum getGLTextureType(int bits)
{
    GLenum type = GL_UNSIGNED_BYTE;

    switch (bits)
    {
        case 16:
            type = GL_UNSIGNED_SHORT;
            break;

        case 32:
            type = GL_FLOAT;
            break;

        case 8:
        default:
            break;
    }

    return type;
}

constexpr size_t TEXTURE_COUNT = 20;
TextureCache::TextureCache()
{
    m_textures.reserve(TEXTURE_COUNT);
}

Texture const &TextureCache::getTextureFromHandle(TextureHandle textureHandle)
{
    assert(textureHandle >= 0 &&
           textureHandle < static_cast<TextureHandle>(m_textures.size()));
    return m_textures[textureHandle];
}

TextureHandle TextureCache::loadTexture(std::string const &filePath) {

    auto textureHandleIter = m_textureHandleMap.find(filePath);

    if (textureHandleIter != m_textureHandleMap.end())
    {
        spdlog::debug("getting texture");
        return (*textureHandleIter).second;
    }

    if (stbi_is_hdr(filePath.c_str()))
    {
        stbi_set_flip_vertically_on_load(true);
    }

    int width, height, components;
    float *pixels = stbi_loadf(filePath.c_str(), &width, &height,
                               &components, 0);

    stbi_set_flip_vertically_on_load(false);


    TextureHandle textureHandle = createTexture(width, height, components, 32,
                                                static_cast<void*>(pixels));

    m_textureHandleMap[filePath] = textureHandle;

    stbi_image_free(pixels);
    return textureHandle;
}

TextureHandle TextureCache::createTexture(int width, int height, int component,
                                          int bits, void const *pixels)
{

    assert(pixels);
    Texture texture;
    texture.m_width = width;
    texture.m_height = height;
    GLenum type = getGLTextureType(bits);
    GLenum format = getGLTextureFormat(component);
    glGenTextures(1, &texture.m_id);
    glBindTexture(GL_TEXTURE_2D, texture.m_id);
    glTexImage2D(GL_TEXTURE_2D, 0, format, width, height, 0, format,
                 type, pixels);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    TextureHandle textureHandle = textureCount;
    textureCount++;
    m_textures.push_back(texture);

    return textureHandle;
}
