/*
 * Texture.h
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "Format.h"

#include <array>
#include <memory>
#include <string>
#include <unordered_map>

struct Texture
{
    using Pointer = std::shared_ptr<Texture>;
    enum class Type : uint8_t
    {
        Tex2D = 0,
        TexCube
    };
    uint32_t m_id;
    uint32_t m_width;
    uint32_t m_height;
    Type m_type;
};

Texture::Pointer createTextureFromGLTF(int width, int height, Format textureFormat,
                                       void const *data);
Texture::Pointer createEmptyCubeMap(int width, int height, Format textureFormat);

using TextureHandle = uint32_t;
class TextureCache
{
public:
    TextureCache();
    ~TextureCache() = default;
    TextureHandle loadTexture(std::string const &filePath);
    TextureHandle createTexture(std::string const &name, int width, int height,
                                Format textureFormat, void const *pixels);

    Texture const &getTextureFromHandle(TextureHandle textureHandle);

private:
    std::unordered_map<std::string, TextureHandle> m_textureHandleMap;
    std::vector<Texture> m_textures;
};
