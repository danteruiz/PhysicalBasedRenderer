/*
 * Backend.h
 *
 * Created on 2021/08/03 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "Buffer.h"
#include "Layout.h"
#include "Resource.h"

#include <memory>
#include <vector>
class Pipeline;
class State;
class Backend : public std::enable_shared_from_this<Backend>
{
public:
    Backend() = default;
    ~Backend() = default;

    void setVertexBuffer(Buffer::Pointer &buffer);
    void setIndexBuffer(Buffer::Pointer &buffer);
    void enableAttributes(std::vector<Attribute> const &atrributes);

    void releaseResource(uint32_t resourceId, gpu::Resource::Type type);

private:
    void syncBuffer(Buffer::Pointer &buffer, uint32_t type);
    std::vector<uint32_t> m_releasedBuffers;
    Pipeline *m_pipline {nullptr};
    State *m_state {nullptr};
};
