/*
 * FrameBuffer.h
 *
 * Created on 2021/08/15 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "Texture.h"
struct FrameBuffer
{
    uint32_t m_id { 0 };
    uint32_t m_width { 0 };
    uint32_t m_height { 0 };
    Texture::Pointer m_texture { nullptr };
};
