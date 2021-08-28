#pragma once

#include <string>
#include <vector>

struct GLFWwindow;

namespace imgui
{
void initialize(GLFWwindow *window);
void uninitialize();
void render();
void newFrame();
bool ListBox(std::string name, int *index, std::vector<std::string> &items);
bool Combo(std::string name, int *index, std::vector<std::string> &items);
bool InputText(std::string label, std::string &buffer);
} // namespace imgui
