function(FIND_EXTERNAL_LIBRARIES)
  find_package(OpenGL REQUIRED)
  find_package(glew REQUIRED)
  find_package(glm REQUIRED)
  find_package(fmt REQUIRED)
  find_package(glfw3 REQUIRED)
  find_package(imgui REQUIRED)
  find_package(spdlog REQUIRED)
  find_package(stb REQUIRED)
  find_package(TinyGLTF REQUIRED)
endfunction()
