/*
 * FileSystem.cpp
 *
 * Created on 2021/09/01 by Enter Your Name Here
 * Copyright 2021 Enter Your Name Here
 *
 * Distributed under the MIT Lisense
 * https://mit-license.org/
 */
#include "FileSystem.h"

#include <cassert>

#include <spdlog/spdlog.h>
namespace filesystem
{
void FileWatcher::checkForFilesUpdate()
{
    for (auto &fileIter : m_filesTimeMap)
    {
        auto const &file = fileIter.first;

        FileTime lastWriteTime = std::filesystem::last_write_time(file);

        if (lastWriteTime > fileIter.second)
        {
            fileIter.second = lastWriteTime;

            auto callback =  m_callbackMap[fileIter.first];
            callback(fileIter.first);
        }
    }
}

void FileWatcher::watchFile(std::string const &file,
                            FileWatcher::Callback callback)
{
    assert(std::filesystem::exists(file));

    FileTime lastWriteTime = std::filesystem::last_write_time(file);

    m_filesTimeMap[file] = lastWriteTime;
    m_callbackMap[file] = callback;
}
} // namespace filesystem
