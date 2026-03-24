#include "globals.h"
#include "handlers.h"
#include "defs.h"

#include <hyprland/src/event/EventBus.hpp>
#include <hyprland/src/managers/input/InputManager.hpp>

PluginDescriptionInfo init(HANDLE handle) {
    PHANDLE = handle;

    // ALWAYS add this to your plugins. It will prevent random crashes coming from
    // mismatched header versions.
    const std::string HASH = __hyprland_api_get_hash();
    const std::string CLIENT_HASH = __hyprland_api_get_client_hash();
    if (HASH != CLIENT_HASH) {
        throw std::runtime_error("[Switcharoo Plugin] Version mismatch");
    }

    // ignore that this can return XKB_KEY_NoSymbol, it is only used to check if keysym equals
    SWITCH_KEY = xkb_keysym_from_name(SWITCHAROO_SWITCH_KEY, XKB_KEYSYM_CASE_INSENSITIVE);
    
    if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
        const auto info = std::string("Config: ") +
                          std::to_string(SWITCHAROO_SWTICH_XKB_MOD_L) + ", " +
                          std::to_string(SWITCHAROO_SWTICH_XKB_MOD_R) + ", ";
        HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] Plugin started " + info, GREEN, 8000);
    }

    // clang-format off
    static auto P1 = Event::bus()->m_events.layer.opened.listen([&](const PHLLS &data) { onOpenLayerChange(data, true); });
    static auto P2 = Event::bus()->m_events.layer.closed.listen([&](const PHLLS &data) { onOpenLayerChange(data, false); });
    static auto P3 = Event::bus()->m_events.input.keyboard.key.listen(onKeyPress);
    static auto P4 = Event::bus()->m_events.input.mouse.button.listen(onMouseButton);
    static auto P5 = Event::bus()->m_events.input.keyboard.focus.listen(onKeyboardFocus);
    // clang-format on

    const std::string name = SWITCHAROO_PLUGIN_NAME;
    const std::string author = SWITCHAROO_PLUGIN_AUTHOR;
    const std::string description = SWITCHAROO_PLUGIN_DESC;
    const std::string version = SWITCHAROO_PLUGIN_VERSION;
    return {name, description, author, version};
}
