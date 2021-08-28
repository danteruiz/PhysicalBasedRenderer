macro(GENERATE_IBL_PATHS)
  message("Generate_IBL_Paths")
  message("${IBL_PATH}")
  file(GLOB children RELATIVE
    ${IBL_PATH} ${IBL_PATH}*)

  set(IBL_NAMES "")
  set(IBL_ENUMS_NAMES "")
  set(IBL_PATHS "")
  foreach (child ${children})
    if(IS_DIRECTORY ${IBL_PATH}${child})
      string(CONCAT IBL_ENUMS_NAMES  "${IBL_ENUMS_NAMES}${child}, \n")
      string(CONCAT IBL_NAMES "${IBL_NAMES} \"${child}\",\n")
      string(CONCAT IBL_PATHS "${IBL_PATHS} \"${IBL_PATH}${child}/${child}.hdr\",\n")
    endif()
  endforeach()


  string(REGEX REPLACE "\\\\" "/" IBL_PATHS ${IBL_PATHS})
  message("${IBL_PATHS}")

  string(REGEX REPLACE "^[0-9]" "" IBL_ENUMS_NAMES ${IBL_ENUMS_NAMES})
  configure_file(IBLEnvironments.h.in src/IBLEnvironments.h)
  configure_file(IBLEnvironments.cpp.in src/IBLEnvironments.cpp)
endmacro()
