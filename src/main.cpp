#include <hyprland/src/helpers/Vector2D.hpp>
#include <hyprland/src/managers/HookSystemManager.hpp>
#include <memory>
#include <string>
#define WLR_USE_UNSTABLE

#include "./EventManager.hpp"
#include "globals.hpp"
#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/Window.hpp>
#include <hyprland/src/debug/Log.hpp>
#include <hyprland/src/managers/input/InputManager.hpp>

#include <format>
#include <sys/socket.h>
#include <sys/un.h>
#include <thread>
#include <unistd.h>

// Do NOT change this function.
APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

std::unique_ptr<EventManager> g_pInputSocketManager;

// void hkOnMouseDown(void* _, SCallbackInfo& cbinfo, std::any e) {
//     auto ev = std::any_cast<wlr_pointer_motion_event*>(e);
// }
//
// void hkOnMouseUp(void* _, SCallbackInfo& cbinfo, std::any e) {
//     auto ev = std::any_cast<wlr_touch_down_event*>(e);
// }

void hkOnMouseMove(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<const Vector2D>(e);

    g_pInputSocketManager->postEvent({"mouseMove", std::format("{},{}", ev.x, ev.y)});
}

void hkOnTouchDown(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_down_event*>(e);

    g_pInputSocketManager->postEvent({"touchDown", std::format("{},{},{}", ev->touch_id, ev->x, ev->y)});
}

void hkOnTouchUp(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_up_event*>(e);

    g_pInputSocketManager->postEvent({"touchUp", std::to_string(ev->touch_id)});
}

void hkOnTouchMove(void* _, SCallbackInfo& cbinfo, std::any e) {
    auto ev = std::any_cast<wlr_touch_motion_event*>(e);

    g_pInputSocketManager->postEvent({"touchMove", std::format("{},{},{}", ev->touch_id, ev->x, ev->y)});
}

HOOK_CALLBACK_FN gMouseMoveCallback = hkOnMouseMove;
HOOK_CALLBACK_FN gTouchDownCallback = hkOnTouchDown;
HOOK_CALLBACK_FN gTouchUpCallback   = hkOnTouchUp;
HOOK_CALLBACK_FN gTouchMoveCallback = hkOnTouchMove;

APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    g_pInputSocketManager = std::make_unique<EventManager>();
    g_pInputSocketManager->startThread(default_socket_path("input"));

    HyprlandAPI::registerCallbackStatic(PHANDLE, "mouseMove", &gMouseMoveCallback);
    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchDown", &gTouchDownCallback);
    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchUp", &gTouchUpCallback);
    HyprlandAPI::registerCallbackStatic(PHANDLE, "touchMove", &gTouchMoveCallback);

    HyprlandAPI::reloadConfig();

    return {"input-socket", "Advertise input events over socket", "horriblename", "1.0"};
}

APICALL EXPORT void PLUGIN_EXIT() {
    g_pInputSocketManager.reset();
}
