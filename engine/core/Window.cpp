/*
 * Window.cpp
 *
 * Created on 2021/08/03 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Window.h"

#include "Glfw.h"

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <spdlog/spdlog.h>

Window::Window(int width, int height, std::string title)
{
    m_width = width;
    m_height = height;
    m_windowTitle = title;
}

Window::~Window()
{
    glfw::destroyWindow(m_glfwWindow);
    glfw::terminate();
}

void Window::simpleUpdate() { glfw::pollEvents(); }

void Window::swap() { glfw::swapBuffers(m_glfwWindow); }

// Init the window and check for any window errors
bool Window::createWindow()
{

#ifdef __APPLE__
    glfwWindowHint(GLFW_SAMPLES, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
#else
    glfwWindowHint(GLFW_SAMPLES, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 4);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
#endif

    m_glfwWindow = glfw::createWindow(m_width, m_height, m_windowTitle);

    if (!m_glfwWindow)
    {
        glfw::terminate();
        return false;
    }

    glfwMakeContextCurrent(m_glfwWindow);
    /*
     * const unsigned char* vendor = glGetString(GL_VENDOR);
     * const unsigned char* renderer = glGetString(GL_RENDERER);
     * 
     * spdlog::debug("render info {} - {}", vendor, renderer);
     */
/*
 * #define GL_GPU_MEM_INFO_TOTAL_AVAILABLE_MEM_NVX 0x9048
 * #define GL_GPU_MEM_INFO_CURRENT_AVAILABLE_MEM_NVX 0x9049
 * 
 *     GLint total_mem_kb = 0;
 *     glGetIntegerv(GL_GPU_MEM_INFO_TOTAL_AVAILABLE_MEM_NVX, 
 *                   &total_mem_kb);
 * 
 *     GLint cur_avail_mem_kb = 0;
 *     glGetIntegerv(GL_GPU_MEM_INFO_CURRENT_AVAILABLE_MEM_NVX, 
 *                   &cur_avail_mem_kb);
 * 
 * 
 *     spdlog::debug("gpu data - total mem: {}, used mem: {}", 
total_mem_kb, total_mem_kb - cur_avail_mem_kb);
 */
    return true;
}

bool Window::shouldClose() { return glfw::windowShouldClose(m_glfwWindow); }

void Window::setWidthAndHeight(int width, int height)
{
    m_width = width;
    m_height = height;
    glViewport(0, 0, m_width, m_height);
}
int const *m_value;
void Window::resetWindowSize()
{
    int width, height;

    glfwGetFramebufferSize(m_glfwWindow, &width, &height);
    glViewport(0, 0, width, height);
}
