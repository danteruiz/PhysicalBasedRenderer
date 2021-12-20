project(render)
set(TARGET_NAME render)
file(GLOB graphics_headers "*.h")
file(GLOB graphics_source "*.cpp")

add_library(render STATIC ${graphics_source} ${graphics_headers})

target_link_libraries(render
  PUBLIC glm::glm OpenGL::GL GLEW::GLEW spdlog::spdlog fmt::fmt stb::stb TinyGLTF::TinyGLTF)

link_personal_libraries(core)
target_include_directories(render
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
  PRIVATE ${TINYGLTF_INCLUDE_DIRS}  ${STB_INCLUDE_DIRS})

target_compile_definitions(${PROJECT_NAME} PUBLIC RESOURCE_PATH="${CMAKE_SOURCE_DIR}/resources/")
