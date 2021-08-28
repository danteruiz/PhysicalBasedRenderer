/*
 * Format.h
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once
#include <cstdint>
#include <vector>

enum Type : uint8_t
{
    Float = 0,
    Half,
    Int32,
    Int16,
    Int8,
    UInt32,
    UInt16,
    UInt8,
    TypeNum
};

enum Dimension : uint8_t
{
    Scalar = 0,
    Vec2,
    Vec3,
    Vec4,
    DimensionNum
};

class Format
{
public:
    Format() : m_type(Type::Int32), m_dimension(Dimension::Scalar) {}
    Format(Type type, Dimension dimension)
        : m_type(type), m_dimension(dimension) { }

    Type getType() const { return m_type; }
    Dimension getDimension() const { return m_dimension; }
    uint32_t getSize() const;
    uint32_t getStride() const;
    uint32_t getDimensionSize() const;

private:
    Type m_type;
    Dimension m_dimension;
};
