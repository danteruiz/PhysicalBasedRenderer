#pragma once

#include <memory>
#include <string>

class InputDevice
{
public:
    using Pointer = std::shared_ptr<InputDevice>;
    enum Type
    {
        JOYSTICK,
        KEYBOARD,
        MOUSE,
        INVALID
    };
    InputDevice(Type type) noexcept;

    Type getType() const { return m_type; }
    int getID() const { return m_id; }
    std::string getName() const { return m_name; }

    virtual void update() = 0;

protected:
    Type m_type;
    int m_id {0};

    std::string m_name;
};
