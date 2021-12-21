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
#include "Resource.h"

#include <memory>
namespace gpu
{
struct FrameBuffer
{
    uint32_t width { 0 };
    uint32_t height { 0 };
    std::unique_ptr<gpu::Resouse> gpuResource { nullptr };
    Texture::Pointer texture { nullptr };
};
}
