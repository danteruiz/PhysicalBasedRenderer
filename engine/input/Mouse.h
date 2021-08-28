#pragma once

#include "InputDevice.h"

#include <array>
#include <tuple>

static const int MAX_MOUSE_BUTTONS {9};

struct GLFWwindow;
class Mouse : public InputDevice
{
public:
    enum MouseButtonChannel
    {
        MOUSE_BUTTON_1 = 0,
        MOUSE_BUTTON_2,
        MOUSE_BUTTON_3,
        MOUSE_BUTTON_4,
        MOUSE_BUTTON_5,
        MOUSE_BUTTON_6,
        MOUSE_BUTTON_7,
        MOUSE_BUTTON_LAST,
        MOUSE_BUTTON_NUM,

        MOUSE_BUTTON_LEFT = MOUSE_BUTTON_1,
        MOUSE_BUTTON_RIGHT = MOUSE_BUTTON_2,
        MOUSE_BUTTON_MIDDLE = MOUSE_BUTTON_3
    };

    enum MouseAxisChannel
    {
        MOUSE_AXIS_Y,
        MOUSE_AXIS_X,
        MOUSE_AXIS_X_DELTA,
        MOUSE_AXIS_Y_DELTA,
        MOUSE_AXIS_NUM
    };

    Mouse(InputDevice::Type type) noexcept;

    int getButton(int channel) { return m_buttonStates[channel]; }
    float getAxis(int channel) { return m_axisStates[channel]; }

    void update() override;

private:
    struct Cursor
    {
        double x {0.0};
        double y {0.0};
    };

    Cursor m_lastCursor;
    std::array<int, MOUSE_BUTTON_NUM> m_buttonStates;
    std::array<float, MOUSE_AXIS_NUM> m_axisStates;
};
