macro(GENERATE_MODEL_PATHS)
  message("Generate_Model_Paths")
  message("${GLTF_MODEL_PATH}")
  file(GLOB children RELATIVE
    ${GLTF_MODEL_PATH} ${GLTF_MODEL_PATH}*)

  set(GLTF_MODEL_NAMES "")
  set(MODEL_NAMES "")
  set(MODEL_PATHS "")
  foreach (child ${children})
    if(IS_DIRECTORY ${GLTF_MODEL_PATH}${child})
      string(CONCAT GLTF_MODEL_NAMES  "${GLTF_MODEL_NAMES}${child}, \n")
      string(CONCAT MODEL_NAMES "${MODEL_NAMES} \"${child}\",\n")
      string(CONCAT MODEL_PATHS "${MODEL_PATHS} \"${GLTF_MODEL_PATH}${child}/glTF/${child}.gltf\",\n")
    endif()
  endforeach()


  string(REGEX REPLACE "\\\\" "/" MODEL_PATHS ${MODEL_PATHS})
  message("${MODEL_PATHS}")

  string(REGEX REPLACE "^[0-9]" "" GLTF_MODEL_NAMES ${GLTF_MODEL_NAMES})
  configure_file(ModelPaths.h.in src/ModelPaths.h)
  configure_file(ModelPaths.cpp.in src/ModelPaths.cpp)
endmacro()
