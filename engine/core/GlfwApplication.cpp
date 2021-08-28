/*
 * GlfwApplication.cpp
 *
 * Created on 2021/08/03 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "GlfwApplication.h"

#include "Glfw.h"
#include "Window.h"

#include <GLFW/glfw3.h>
#include <cassert>

static GlfwApplication *g_application {nullptr};

void onWindowSizeChanged([[maybe_unused]] GLFWwindow *window, int width,
                         int height)
{
    auto mainWindow = GlfwApplication::instance()->getWindow();
    mainWindow->setWidthAndHeight(width, height);
}

GlfwApplication::GlfwApplication()
{
    assert(g_application == nullptr);
    g_application = this;
    glfwInitHint(GLFW_JOYSTICK_HAT_BUTTONS, GLFW_FALSE);
    if (!glfw::initialize())
    {
    }

    m_window = std::make_shared<Window>(1800, 1200, "Demo");
    if (!m_window->createWindow())
    {
        std::cout << "window faied to be created" << std::endl;
    }
    glfwSetWindowSizeCallback(m_window->getWindowPtr(), onWindowSizeChanged);
}

GlfwApplication *GlfwApplication::instance() { return g_application; }

GlfwApplication::~GlfwApplication() { g_application = nullptr; }
