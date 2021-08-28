project(PbrViewer)
set(TARGET_NAME PbrViewer)

generate_model_paths()
generate_ibl_paths()

if (MSVC)
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} /SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup")
  add_definitions(-DWINDOWS)
endif()
file(GLOB source_files "*.h" "*.cpp"
  "${CMAKE_CURRENT_BINARY_DIR}/src/*.cpp" "${CMAKE_CURRENT_BINARY_DIR}/src/*.h")

if (MSVC)
  add_executable(${PROJECT_NAME} WIN32 ${source_files})
else()
  add_executable(${PROJECT_NAME} ${source_files})
endif()

target_compile_definitions(${PROJECT_NAME} PUBLIC RESOURCE_PATH="${CMAKE_SOURCE_DIR}/resources/")
link_personal_libraries(render input core ui)
target_link_libraries(${PROJECT_NAME} PUBLIC fmt::fmt spdlog::spdlog)
target_include_directories(${TARGET_NAME}
  PUBLIC ${CMAKE_CURRENT_BINARY_DIR}/src)
