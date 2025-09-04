#include <hyprland/src/plugins/PluginAPI.hpp>
#include <hyprland/src/devices/IPointer.hpp>
#include <hyprland/src/desktop/LayerSurface.hpp>

#include "globals.h"

void onMouseButton(const IPointer::SButtonEvent event) {
    MOUSE_BUTTON_PRESSED = true;
    if constexpr (HYPRSHELL_PRINT_DEBUG == 1) {
        HyprlandAPI::addNotification(PHANDLE, "[Hyprshell Plugin] Mouse button pressed", GREEN, 4000);
    }
}