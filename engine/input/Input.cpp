#include "Input.h"

#include "InputChannels.h"
#include "Joystick.h"

#include <GLFW/glfw3.h>
#include <functional>
#include <iostream>

Joystick *joystick;

void joystickConnectioncallback(int jid, int event)
{
    switch (event)
    {
    case GLFW_CONNECTED:
        std::cout << "conencted: " << jid << std::endl;
        break;

    case GLFW_DISCONNECTED:
        std::cout << "disconnected: " << jid << std::endl;
    default:
        break;
    };
}

Input::Input()
{
    glfwSetJoystickCallback(joystickConnectioncallback);

    std::cout << "GLFW_PRESS: " << GLFW_PRESS << "\n";
    std::cout << "GLFW_RELEASE: " << GLFW_RELEASE << "\n";

    std::shared_ptr<Joystick> joystick =
        std::make_shared<Joystick>(GLFW_JOYSTICK_1, InputDevice::JOYSTICK);
    m_inputDevices[joystick->getID()] = joystick;
}

bool isButtonPressed(const unsigned char button)
{
    return (button == GLFW_PRESS);
}

void Input::pollInput()
{

    for (const auto &inputDevice : m_inputDevices)
    {
        inputDevice.second->update();
    }
}
