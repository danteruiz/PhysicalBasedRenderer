/*
 * FileSystem.h
 *
 * Created on 2021/09/01 by Enter Your Name Here
 * Copyright 2021 Enter Your Name Here
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */
#pragma once

#include <filesystem>
#include <functional>
#include <string>
#include <unordered_map>
namespace filesystem
{
// TODO make this class run on a different thread
class FileWatcher
{
public:
    using FileTime = std::filesystem::file_time_type;
    using Callback = std::function<void(std::string const &)>;
    FileWatcher() = default;
    ~FileWatcher() = default;

    void checkForFilesUpdate();
    void watchFile(std::string const &file, FileWatcher::Callback callback);

private:
    std::unordered_map<std::string, FileWatcher::Callback> m_callbackMap;
    std::unordered_map<std::string, FileTime> m_filesTimeMap;
};
} // namespace filesystem
