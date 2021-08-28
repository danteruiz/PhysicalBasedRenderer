project(ui)
set(TARGET_NAME ui)

file(GLOB ui_headers "*.h" "imgui/*.h" "imgui/bindings/*.h")
file(GLOB ui_source "*.cpp" "imgui/*.cpp" "imgui/bindings/*.cpp")

add_library(${TARGET_NAME} STATIC ${ui_headers} ${ui_source})

link_personal_libraries(core render)

target_link_libraries(${TARGET_NAME} PRIVATE glfw)
target_link_libraries(${TARGET_NAME} PUBLIC imgui::imgui)
target_include_directories(${TARGET_NAME}
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

target_compile_definitions(${TARGET_NAME}  PRIVATE IMGUI_IMPL_OPENGL_LOADER_GLEW)
