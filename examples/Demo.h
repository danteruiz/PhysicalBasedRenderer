/*
 * Demo.h
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include "DebugDraw.h"
#include "Entity.h"
#include "GlfwApplication.h"

#include <Skybox.h>
#include <memory>
#include <vector>

class Window;
class Buffer;
class DebugUI;
class Shader;
class ModelCache;
class Backend;
class DebugDraw;
struct Marker;

class DemoApplication : public GlfwApplication
{
public:
    DemoApplication();
    void exec() override;

private:
    void generateIBLEnvironment(std::string &texturePath);
    unsigned int generateEnviromentMap();
    Light m_light;
    Entity m_modelEntity;
    Skybox m_skybox;
    std::vector<Light> m_lights;
    std::shared_ptr<ModelCache> m_modelCache;
    std::shared_ptr<DebugUI> m_debugUI;
    std::shared_ptr<Shader> m_pipeline {nullptr};
    std::shared_ptr<Shader> m_irradiance {nullptr};
    std::shared_ptr<Shader> m_convertToCubeMap {nullptr};
    std::shared_ptr<Shader> m_filterMap {nullptr};
    std::shared_ptr<Shader> m_brdfLut {nullptr};
    std::shared_ptr<DebugDraw> m_debugDraw {nullptr};
    std::vector<Marker> m_markers;
    std::shared_ptr<Backend> m_backend {nullptr};
};
