#include "Glfw.h"

#include <GLFW/glfw3.h>
namespace glfw
{
bool initialize()
{
    if (glfwInit())
    {
        return true;
    }

    return false;
}

void terminate() { glfwTerminate(); }

void swapBuffers(GLFWwindow *window) { glfwSwapBuffers(window); }

GLFWwindow *createWindow(int width, int height, const std::string &title)
{
    return glfwCreateWindow(width, height, title.c_str(), NULL, NULL);
}

void destroyWindow(GLFWwindow *window) { glfwDestroyWindow(window); }

bool windowShouldClose(GLFWwindow *window)
{
    return glfwWindowShouldClose(window);
}

void pollEvents() { glfwPollEvents(); }
} // namespace glfw
