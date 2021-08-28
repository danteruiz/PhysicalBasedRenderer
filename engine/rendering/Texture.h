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

struct Texture
{
    using Pointer = std::shared_ptr<Texture>;
    uint32_t m_id;
    uint32_t m_width;
    uint32_t m_height;
};

Texture::Pointer createTextureFromGLTF(int width, int height,
                                       int component, int bits,
                                       unsigned char *data);

Texture::Pointer loadTexture(std::string path);
Texture::Pointer loadCubeMap(std::array<std::string, 6> imagePaths);
Texture loadHDRTexture(std::string path);

