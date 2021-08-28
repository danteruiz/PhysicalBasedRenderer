#include "Imgui.h"

#include <imgui.h>

#include <imgui_impl_glfw.h>
#include <imgui_impl_opengl3.h>

static auto vector_getter = [](void *vec, int idx, const char **out_text)
{
    auto &vector = *static_cast<std::vector<std::string> *>(vec);
    if (idx < 0 || idx >= static_cast<int>(vector.size()))
    {
        return false;
    }
    *out_text = vector.at(idx).c_str();
    return true;
};
namespace imgui
{
void initialize(GLFWwindow *window)
{
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    // ImGuiIO &io = ImGui::GetIO();
    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init("#version 150");
    ImGui::StyleColorsDark();
}
void uninitialize() {}

void render()
{
    ImGui::Render();
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
}

void newFrame()
{
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
}

bool ListBox(std::string name, int *index, std::vector<std::string> &items)
{
    return ImGui::ListBox(name.c_str(), index, vector_getter,
                          static_cast<void *>(&items), (int)items.size());
}

bool Combo(std::string name, int *index, std::vector<std::string> &items)
{
    return ImGui::Combo(name.c_str(), index, vector_getter,
                        static_cast<void *>(&items), (int)items.size());
}

bool InputText(std::string label, std::string &buffer)
{
    const char *buf = buffer.data();
    return ImGui::InputText(label.c_str(), const_cast<char *>(buf),
                            buffer.size());
}
} // namespace imgui
