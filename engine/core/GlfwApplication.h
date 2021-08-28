/*
 * GlfwApplication.h
 *
 * Created on 2021/08/03 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#pragma once

#include <iostream>
#include <memory>
#include <string>

class Window;
class GlfwApplication
{
public:
    GlfwApplication();
    ~GlfwApplication();

    virtual void exec() = 0;

    std::shared_ptr<Window> getWindow() { return m_window; }
    static GlfwApplication *instance();

protected:
    std::shared_ptr<Window> m_window {nullptr};
};
