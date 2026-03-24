#include <hyprland/src/plugins/PluginAPI.hpp>
#include <hyprland/src/desktop/view/LayerSurface.hpp>

#include "globals.h"

void onOpenLayerChange(const PHLLS &window, const bool open) {
    if (window->m_namespace.starts_with("switcharoo_")) {
        if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
            HyprlandAPI::addNotification(PHANDLE, "[Switcharoo] Layer active: " + std::string(open ? "true" : "false") + " namespace: " + window->m_namespace, GREEN, 5000);
        }
        LAYER_VISIBLE = open;
    }
}
