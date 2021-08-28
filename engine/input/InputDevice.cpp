#include "InputDevice.h"

static int INPUT_DEVICES = 0;

InputDevice::InputDevice(InputDevice::Type type) noexcept
    : m_type(type), m_id(++INPUT_DEVICES)
{
}
