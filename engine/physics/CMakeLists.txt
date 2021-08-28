file(GLOB physics_sources "*.cpp" "*.h")
add_library(physics "")

target_sources(physics PRIVATE ${physics_sources})

# target_link_libraries(physics
#   PUBLIC ${CONAN_LIBS_GLM}
#   PRIVATE ${CONAN_LIBS_BULET})
# 
# target_include_directories(physics
#   PUBLIC ${CONAN_INCLUDE_DIRS_GLM} ${CMAKE_CURRENT_SOURCE_DIR}
#   PRIVATE ${CONAN_INCLUDE_DIRS_BULLET})
