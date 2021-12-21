file(GLOB glfw_headers "*.h")
file(GLOB glfw_source "*.cpp")
set(TARGET_NAME core)

add_library(${TARGET_NAME} STATIC ${glfw_source} ${glfw_headers})
target_link_libraries(${TARGET_NAME}
  PRIVATE glfw glew::glew
  PUBLIC fmt::fmt spdlog::spdlog)
target_compile_definitions(${TARGET_NAME} PUBLIC RESOURCE_PATH="${CMAKE_SOURCE_DIR}/resources/")
