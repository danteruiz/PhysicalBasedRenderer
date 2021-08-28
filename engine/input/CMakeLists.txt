set(TARGET_NAME input)

file(GLOB input_sources "*.h" "*.cpp")

add_library(${TARGET_NAME} STATIC ${input_sources})

link_personal_libraries(core)
target_link_libraries(${TARGET_NAME}
  PRIVATE glfw)

target_include_directories(${TARGET_NAME}
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
