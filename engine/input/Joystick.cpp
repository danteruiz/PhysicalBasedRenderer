#include "Joystick.h"

Joystick::Joystick(int joystickIndex, InputDevice::Type type) noexcept
    : InputDevice(type), m_joystickIndex(joystickIndex)
{

    m_name = std::string(glfwGetJoystickName(joystickIndex));
}

int Joystick::getButton(int buttonChannel) const
{
    return m_gamepadState.buttons[buttonChannel];
}

float Joystick::getAxis(int axisChannel) const
{
    return m_gamepadState.axes[axisChannel];
}

void Joystick::update()
{
    if (!glfwJoystickPresent(m_joystickIndex))
    {
        return;
    }
    glfwGetGamepadState(m_joystickIndex, &m_gamepadState);
}

