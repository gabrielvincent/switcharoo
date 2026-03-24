#include <strings.h>
#include <hyprland/src/plugins/PluginAPI.hpp>
#include <hyprland/src/devices/IKeyboard.hpp>
#include <hyprland/src/managers/input/InputManager.hpp>

#include "globals.h"
#include "defs.h"
#include "send.h"

// modifier must pre pressed and released without any other keys pressed in between
bool last_press_was_mod_press = false;

void onKeyPress(const std::unordered_map<std::string, std::any> &data, SCallbackInfo &info) {
    const auto keyboardIt = data.find("keyboard");
    const auto eventIt = data.find("event");

    if (keyboardIt != data.end() && eventIt != data.end()) {
        const auto keyboard = std::any_cast<CSharedPointer<IKeyboard> >(keyboardIt->second);
        if (g_pInputManager->shouldIgnoreVirtualKeyboard(keyboard)) {
            return;
        }
        const auto event = std::any_cast<IKeyboard::SKeyEvent>(eventIt->second);
        const auto state = keyboard->m_xkbState;
        const uint32_t keycode = event.keycode + 8; // +8 because xkbcommon expects +8 from libinput
        const bool release = event.state == WL_KEYBOARD_KEY_STATE_RELEASED;

        // const bool shiftActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_SHIFT, XKB_STATE_MODS_EFFECTIVE) == 1;
        const bool ctrlActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_CTRL, XKB_STATE_MODS_EFFECTIVE) == 1;
        const bool superActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_LOGO, XKB_STATE_MODS_EFFECTIVE) == 1;
        const bool altActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_ALT, XKB_STATE_MODS_EFFECTIVE) == 1;

        const xkb_keysym_t keysym = xkb_state_key_get_one_sym(state, keycode);

        // open switch mode
        if (!release && !LAYER_VISIBLE) {
            if (keysym == SWITCH_KEY) {
                if ((SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Alt_L && altActive && !superActive && !ctrlActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Super_L && superActive && !altActive && !ctrlActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Control_L && ctrlActive && !superActive && !altActive)
                ) {
                    if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
                        HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] switch open (tab) pressed", GREEN,
                                                     2000);
                    }
                    info.cancelled = true;
                    sendStringToSwitcharooSocket(SWITCHAROO_OPEN_SWITCH);
                }
            }
            if (keysym == XKB_KEY_ISO_Left_Tab) {
                if ((SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Alt_L && altActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Super_L && superActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Control_L && ctrlActive)
                ) {
                    if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
                        HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] switch open (shift + tab) pressed",
                                                     GREEN, 2000);
                    }
                    info.cancelled = true;
                    sendStringToSwitcharooSocket(SWITCHAROO_OPEN_SWITCH_REVERSE);
                }
            }
            if (keysym == XKB_KEY_grave || keysym == XKB_KEY_dead_grave) {
                if ((SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Alt_L && altActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Super_L && superActive) ||
                    (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Control_L && ctrlActive)
                ) {
                    if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
                        HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] switch open (grave) pressed", GREEN,
                                                     2000);
                    }
                    info.cancelled = true;
                    sendStringToSwitcharooSocket(SWITCHAROO_OPEN_SWITCH_REVERSE);
                }
            }
        }

        // release switch mode
        if (release && (keysym == SWITCHAROO_SWTICH_XKB_MOD_R || keysym == SWITCHAROO_SWTICH_XKB_MOD_L)) {
            if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
                HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] mod release pressed", GREEN, 2000);
            }
            sendStringToSwitcharooSocket(SWITCHAROO_CLOSE);
        }
    }
}
