If you have issue compiling spdlog on linux make sure the set the deault libcxx to  std++11

conan profile update settings.compiler.libcxx=libstdc++11 default
