/*
 * Format.cpp
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Format.h"

constexpr int TYPE_SIZE[Type::TypeNum] = {4, 4, 2, 1, 4, 2, 1};

constexpr int DIMENSION_SIZE[Dimension::DimensionNum] = {1, 2, 3, 4};

uint32_t Format::getSize() const { return TYPE_SIZE[m_type]; }

uint32_t Format::getStride() const { return TYPE_SIZE[m_type] * DIMENSION_SIZE[m_dimension]; }

uint32_t Format::getDimensionSize() const { return DIMENSION_SIZE[m_dimension]; }

Format Format::fromComponentsAndBits(int components, int bits)
{
    Dimension dimension;

    switch (components)
    {
    case 1:
        dimension = Dimension::Scalar;
        break;
    case 2:
        dimension = Dimension::Vec2;
        break;
    case 3:
        dimension = Dimension::Vec3;
        break;
    case 4:
    default:
        dimension = Dimension::Vec4;
        break;
    }

    Type type;
    switch (bits)
    {
    case 8:
        type = Type::UInt8;
        break;
    case 16:
        type = Type::UInt16;
        break;
    case 32:
    default:
        type = Type::UInt32;
        break;
    }

    return Format(type, dimension);
}
