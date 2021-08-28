#pragma once

#include "InputDevice.h"

#include <memory>
#include <unordered_map>
#include <vector>

class Mouse;
class Input
{
public:
    Input();
    ~Input() = default;

    void pollInput();

private:
    std::unordered_map<int, InputDevice::Pointer> m_inputDevices;
};
