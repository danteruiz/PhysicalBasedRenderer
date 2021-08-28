/*
 * Skybox.h
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "Model.h"
#include "Shader.h"
#include "Texture.h"

#include <memory>

struct Skybox
{
    std::shared_ptr<Texture> texture;
    std::shared_ptr<Shader> shader;
    std::shared_ptr<Model> model;
};
