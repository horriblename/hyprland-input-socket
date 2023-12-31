#define WLR_USE_UNSTABLE

#include "globals.hpp"
#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/Window.hpp>
#include <hyprland/src/debug/Log.hpp>
#include <hyprland/src/managers/input/InputManager.hpp>

#include <sys/socket.h>
#include <sys/un.h>
#include <thread>
#include <unistd.h>

int g_SockFd = -1;

void emit_event(std::string ev) {
    if (g_SockFd >= 0) {}
}

void hkOnMouseDown(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_pointer_motion_event*>(e);
}

void hkOnMouseUp(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_down_event*>(e);
}

void hkOnMouseMove(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_down_event*>(e);
}

void hkOnTouchDown(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_down_event*>(e);
}

void hkOnTouchUp(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_up_event*>(e);
}

void hkOnTouchMove(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_motion_event*>(e);
}

HOOK_CALLBACK_FN gTouchDownCallback = hkOnTouchDown;
HOOK_CALLBACK_FN gTouchUpCallback   = hkOnTouchUp;
HOOK_CALLBACK_FN gTouchMoveCallback = hkOnTouchMove;

int start_socket() {
    int sock_fd = socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);

    if (sock_fd < 0) {
        Debug::log(ERR, "Couldn't start the input socket. (1)");
        return;
    }

    sockaddr_un addr = {.sun_family = AF_UNIX};

    std::string socketPath = "/tmp/hypr/" + g_pCompositor->m_szInstanceSignature + "/.input.sock";

    strcpy(addr.sun_path, socketPath.c_str());

    if (bind(sock_fd, (sockaddr*)&addr, SUN_LEN(&addr)) < 0) {
        Debug::log(ERR, "Couldn't start the input Socket. (2)");
        return;
    }

    // 10 max queued.
    listen(sock_fd, 10);

    Debug::log(LOG, "Hypr socket started at {}", socketPath);

    // wl_event_loop_add_fd(g_pCompositor->m_sWLEventLoop, sock_fd, WL_EVENT_READABLE, hyprCtlFDTick, nullptr);
    return sock_fd;
}

APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchDown", &gTouchDownCallback);
    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchUp", &gTouchUpCallback);
    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchMove", &gTouchMoveCallback);

    HyprlandAPI::reloadConfig();

    return {"input-socket", "Advertise input events over socket", "horriblename", "1.0"};
}

APICALL EXPORT void PLUGIN_EXIT() {
    if (g_SockFd >= 0) {
        close(g_SockFd);
    }
}
