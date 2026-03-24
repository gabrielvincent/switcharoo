#include "globals.h"
#include "handlers.h"
#include "defs.h"

PluginDescriptionInfo init(HANDLE handle) {
    PHANDLE = handle;
    // ALWAYS add this to your plugins. It will prevent random crashes coming from
    // mismatched header versions.
    const std::string HASH        = __hyprland_api_get_hash();
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
    static auto P1 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "openLayer",[&](void*, SCallbackInfo&, const std::any &data) { onOpenLayerChange(std::any_cast<PHLLS>(data), true); });
    static auto P2 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "closeLayer",[&](void*, SCallbackInfo&, const std::any &data) { onOpenLayerChange(std::any_cast<PHLLS>(data), false); });
    static auto P3 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "keyPress",[&](void*, SCallbackInfo& info, const std::any &data) { onKeyPress(std::any_cast<std::unordered_map<std::string, std::any>>(data), info); });
    static auto P4 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "mouseButton",[&](void*, SCallbackInfo&, const std::any &data) { onMouseButton(std::any_cast<IPointer::SButtonEvent>(data)); });
    static auto P5 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "keyboardFocus",[&](void*, SCallbackInfo&, const std::any &data) { onKeyboardFocus(std::any_cast<SP<CWLSurfaceResource>>(data)); });
    // clang-format on

    const std::string name = SWITCHAROO_PLUGIN_NAME;
    const std::string author = SWITCHAROO_PLUGIN_AUTHOR;
    const std::string description = SWITCHAROO_PLUGIN_DESC;
    const std::string version = SWITCHAROO_PLUGIN_VERSION;
    return {name, description, author, version};
}
