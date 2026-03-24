#include <strings.h>
#include <hyprland/src/plugins/PluginAPI.hpp>
#include <hyprland/src/devices/IKeyboard.hpp>
#include <hyprland/src/event/EventBus.hpp>
#include <hyprland/src/managers/input/InputManager.hpp>

#include "globals.h"
#include "handlers.h"
#include "defs.h"
#include "send.h"

// modifier must pre pressed and released without any other keys pressed in between
bool last_press_was_mod_press = false;

// last keyboard (a bit hacky)
SP<IKeyboard> last_keyboard;

void onKeyPress(const IKeyboard::SKeyEvent &event, Event::SCallbackInfo &info) {
    const uint32_t keycode = event.keycode + 8; // +8 because xkbcommon expects +8 from libinput
    const bool release = event.state == WL_KEYBOARD_KEY_STATE_RELEASED;

    bool valid = false;
    SP<IKeyboard> keyboard;
    if (!release) {
        // Get the correct keyboard from the input manager
        for (const auto &kb: g_pInputManager->m_keyboards) {
            if (kb->getPressed(event.keycode)) {
                keyboard = kb;
                last_keyboard = kb;
                valid = true;
                break;
            }
        }
    } else {
        keyboard = last_keyboard;
        valid = true;
    }

    if (!valid || !keyboard) {
        return;
    }

    if (g_pInputManager->shouldIgnoreVirtualKeyboard(keyboard)) {
        return;
    }
    const auto state = keyboard->m_xkbState;

    const bool ctrlActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_CTRL, XKB_STATE_MODS_EFFECTIVE) == 1;
    const bool superActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_LOGO, XKB_STATE_MODS_EFFECTIVE) == 1;
    const bool altActive = xkb_state_mod_name_is_active(state, XKB_MOD_NAME_ALT, XKB_STATE_MODS_EFFECTIVE) == 1;

    const xkb_keysym_t keysym = xkb_state_key_get_one_sym(state, keycode);

    if (!release) {
        bool modActive = false;
        if (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Alt_L && altActive) modActive = true;
        else if (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Super_L && superActive) modActive = true;
        else if (SWITCHAROO_SWTICH_XKB_MOD_L == XKB_KEY_Control_L && ctrlActive) modActive = true;

        if (modActive) {
            if (keysym == SWITCH_KEY || keysym == XKB_KEY_Tab || keysym == XKB_KEY_ISO_Left_Tab) {
                info.cancelled = true;
                sendStringToSwitcharooSocket(SWITCHAROO_OPEN_SWITCH);
                return;
            } else if (keysym == XKB_KEY_grave || keysym == XKB_KEY_dead_grave) {
                info.cancelled = true;
                sendStringToSwitcharooSocket(SWITCHAROO_OPEN_SWITCH_REVERSE);
                return;
            }
        }
    }

    // release switch mode
    if (release && (keysym == SWITCHAROO_SWTICH_XKB_MOD_R || keysym == SWITCHAROO_SWTICH_XKB_MOD_L)) {
        sendStringToSwitcharooSocket(SWITCHAROO_CLOSE);
    }
}
