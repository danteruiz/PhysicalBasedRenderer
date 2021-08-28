#pragma once

#include <string>
struct GLFWwindow;
namespace glfw
{
bool isInitialized();
bool initialize();
void terminate();
void destroyWindow(GLFWwindow *window);
void swapBuffers(GLFWwindow *window);
GLFWwindow *createWindow(int width, int height, const std::string &title);
bool windowShouldClose(GLFWwindow *window);
void pollEvents();
} // namespace glfw
