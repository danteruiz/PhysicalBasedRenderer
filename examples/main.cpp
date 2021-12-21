/*
 * main.cpp
 *
 * Created on 2021/08/21 by Dante Ruiz
 * Copyright 2021 Dante Ruiz
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */

#include "Demo.h"
#include "spdlog/sinks/basic_file_sink.h"
#include "spdlog/sinks/stdout_color_sinks.h"
#include "spdlog/spdlog.h"

int main([[maybe_unused]] int argc, [[maybe_unused]] char *argv[])
{
    // initializeSpdlog();
    DemoApplication demoApplication;
    demoApplication.exec();

    return 0;
}
