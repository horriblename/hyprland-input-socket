#pragma once
#include <deque>
#include <fstream>
#include <mutex>

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/defines.hpp>
#include <hyprland/src/helpers/MiscFunctions.hpp>
#include <hyprland/src/managers/EventManager.hpp>

inline std::string default_socket_path(std::string socket_name) {
    // return "/tmp/hypr/" + g_pCompositor->m_szInstanceSignature + "/." + socket_name + ".sock";
    return "/tmp/hypr/.input.sock";
}

class EventManager {
  public:
    EventManager();
    ~EventManager();

    void postEvent(const SHyprIPCEvent event);

    void startThread(std::string sock_name);

    std::thread m_tThread;

  private:
    void flushEvents();

    std::mutex eventQueueMutex;
    std::deque<SHyprIPCEvent> m_dQueuedEvents;

    std::deque<std::pair<int, wl_event_source*>> m_dAcceptedSocketFDs;
};
